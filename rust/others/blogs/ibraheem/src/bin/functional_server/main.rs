use std::{
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    os::fd::AsRawFd,
    sync::Arc,
};

mod future;
mod runtime;

use crate::future::*;
use crate::runtime::*;

fn main() {
    SCHEDULER.spawn(listen());
    SCHEDULER.run();
}

fn listen() -> impl Future<Output = ()> {
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
        poll_fn(move |_| match listener.accept() {
            Ok((connection, _)) => {
                connection.set_nonblocking(true).unwrap();
                SCHEDULER.spawn(handle(connection));
                None
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                // If the listener is not ready, we can simply return None.
                // Remember, this tells the scheduler the future is not yet ready,
                // and it will be rescheduled once the reactor wakes us.
                None
            }
            Err(e) => panic!("{e}"),
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
