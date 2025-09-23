use std::{
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    os::fd::AsRawFd,
    sync::Arc,
    thread,
    time::Duration,
};

mod future;
mod runtime;
mod shutdown;

use crate::future::*;
use crate::runtime::*;
use crate::shutdown::*;

fn main() {
    SCHEDULER.spawn(listen());
    SCHEDULER.run();
    println!("Graceful shutdown complete!");
}

fn listen() -> impl Future<Output = ()> {
    let tasks = Arc::new(Counter::default());
    let tasks_ref = tasks.clone();

    poll_fn(|waker| {
        let listener = TcpListener::bind("localhost:3000").unwrap();
        listener.set_nonblocking(true).unwrap();

        // Register the listener with epoll.
        // When a connection comes, epoll will return an event and the Reactor will wake the task,
        // causing the scheduler to push our task back onto the queue and poll us again.
        REACTOR.with(|reactor| {
            reactor.add(listener.as_raw_fd(), waker);
        });

        Some(listener)
    })
    .chain(|listener| {
        let listen = poll_fn(move |_| match listener.accept() {
            Ok((connection, _)) => {
                connection.set_nonblocking(true).unwrap();

                // Increment the counter.
                tasks.increment();

                let tasks = tasks.clone();
                let handle_connection = handle(connection).chain(|_| {
                    poll_fn(move |_| {
                        // Decrement the counter.
                        tasks.decrement();
                        Some(())
                    })
                });

                SCHEDULER.spawn(handle_connection);
                None::<()>
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                // If the listener is not ready, we can simply return None.
                // Remember, this tells the scheduler the future is not yet ready,
                // and it will be rescheduled once the reactor wakes us.
                None
            }
            Err(e) => panic!("{e}"),
        });

        select(ctrl_c(), listen)
    })
    .chain(|_ctrl_c| graceful_shutdown(tasks_ref))
}

fn graceful_shutdown(tasks: Arc<Counter>) -> impl Future<Output = ()> {
    let timer = spawn_blocking(|| thread::sleep(Duration::from_secs(1)));
    let request_counter = tasks.wait_for_zero();

    select(timer, request_counter).chain(|_| {
        poll_fn(|_| {
            // https://github.com/ibraheemdev/too-many-web-servers/issues/7
            println!("Start graceful shutdown");
            SCHEDULER.shutdown();
            Some(())
        })
    })
}

fn handle(connection: TcpStream) -> impl Future<Output = ()> {
    let connection = Arc::new(connection);
    let read_connection_ref = connection.clone();
    let write_connection_ref = connection.clone();
    let flush_connection_ref = connection.clone();

    poll_fn(move |waker| {
        // Start by registering our connection for notifications.
        REACTOR.with(|reactor| {
            reactor.add(connection.as_raw_fd(), waker);
        });

        Some(())
    })
    .chain(move |_| {
        let mut read = 0;
        let mut request = [0u8; 1024];

        poll_fn(move |_| {
            let mut connection = &*read_connection_ref;
            loop {
                // Try reading from the stream.
                match connection.read(&mut request[read..]) {
                    Ok(0) => {
                        println!("client disconnected unexpectedly");
                        return Some(());
                    }
                    Ok(n) => read += n,
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => return None,
                    Err(e) => panic!("{e}"),
                }

                // Did we reach the end of the request?
                let read = read;
                if read >= 4 && &request[read - 4..read] == b"\r\n\r\n" {
                    break;
                }
            }

            // We're done, print the request.
            let request = String::from_utf8_lossy(&request[..read]);
            println!("{request}");

            Some(())
        })
    })
    .chain(move |_| {
        let response = concat!(
            "HTTP/1.1 200 OK\r\n",
            "Content-Length: 12\n",
            "Connection: close\r\n\r\n",
            "Hello world!"
        );
        let mut written = 0;

        poll_fn(move |_| {
            let mut connection = &*write_connection_ref;
            loop {
                match connection.write(response[written..].as_bytes()) {
                    Ok(0) => {
                        println!("client disconnected unexpectedly");
                        return Some(());
                    }
                    Ok(n) => written += n,
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => return None,
                    Err(e) => panic!("{e}"),
                }

                // Did we write the whole response yet?
                if written == response.len() {
                    break;
                }
            }

            Some(())
        })
    })
    .chain(move |_| {
        poll_fn(move |_| {
            let mut connection = &*flush_connection_ref;
            match connection.flush() {
                Ok(_) => {}
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                    return None;
                }
                Err(e) => panic!("{e}"),
            };

            REACTOR.with(|reactor| reactor.remove(connection.as_raw_fd()));
            Some(())
        })
    })
}
