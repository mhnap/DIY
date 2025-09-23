use std::{
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    os::fd::AsRawFd,
};

mod runtime;
use runtime::*;

fn main() {
    SCHEDULER.spawn(Main::Start);
    SCHEDULER.run();
}

// Main task: Accept loop.
enum Main {
    Start,
    Accept { listener: TcpListener },
}

impl Future for Main {
    type Output = ();

    fn poll(&mut self, waker: Waker) -> Option<()> {
        if let Main::Start = self {
            let listener = TcpListener::bind("localhost:3000").unwrap();
            listener.set_nonblocking(true).unwrap();

            // Register the listener with epoll.
            // When a connection comes, epoll will return an event and the Reactor will wake the task,
            // causing the scheduler to push our task back onto the queue and poll us again.
            REACTOR.with(|reactor| {
                reactor.add(listener.as_raw_fd(), waker);
            });

            *self = Main::Accept { listener };
        }

        if let Main::Accept { listener } = self {
            match listener.accept() {
                Ok((connection, _)) => {
                    connection.set_nonblocking(true).unwrap();
                    SCHEDULER.spawn(Handler { connection, state: HandlerState::Start });
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // If the listener is not ready, we can simply return None.
                    // Remember, this tells the scheduler the future is not yet ready,
                    // and it will be rescheduled once the reactor wakes us.
                    return None;
                }
                Err(e) => panic!("{e}"),
            }
        }

        None
    }
}

// Handler task: Handles every connection.
struct Handler {
    connection: TcpStream,
    state: HandlerState,
}

enum HandlerState {
    Start,
    Read { request: [u8; 1024], read: usize },
    Write { response: &'static [u8], written: usize },
    Flush,
}

impl Future for Handler {
    type Output = ();

    fn poll(&mut self, waker: Waker) -> Option<Self::Output> {
        if let HandlerState::Start = self.state {
            // Start by registering our connection for notifications.
            REACTOR.with(|reactor| {
                reactor.add(self.connection.as_raw_fd(), waker);
            });

            self.state = HandlerState::Read { request: [0u8; 1024], read: 0 };
        }

        // Read the request
        if let HandlerState::Read { request, read } = &mut self.state {
            loop {
                match self.connection.read(&mut request[*read..]) {
                    Ok(0) => {
                        println!("client disconnected unexpectedly");
                        return Some(());
                    }
                    Ok(n) => *read += n,
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => return None,
                    Err(e) => panic!("{e}"),
                }

                // Did we reach the end of the request?
                let read = *read;
                if read >= 4 && &request[read - 4..read] == b"\r\n\r\n" {
                    break;
                }
            }

            // We're done, print the request.
            let request = String::from_utf8_lossy(&request[..*read]);
            println!("{}", request);

            // And move into the write state.
            let response = concat!(
                "HTTP/1.1 200 OK\r\n",
                "Content-Length: 12\n",
                "Connection: close\r\n\r\n",
                "Hello world!"
            );

            self.state = HandlerState::Write { response: response.as_bytes(), written: 0 };
        }

        // Write the response.
        if let HandlerState::Write { response, written } = &mut self.state {
            loop {
                match self.connection.write(&response[*written..]) {
                    Ok(0) => {
                        println!("client disconnected unexpectedly");
                        return Some(());
                    }
                    Ok(n) => *written += n,
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => return None,
                    Err(e) => panic!("{e}"),
                }

                // Did we write the whole response yet?
                if *written == response.len() {
                    break;
                }
            }

            // Successfully wrote the response, try flushing next.
            self.state = HandlerState::Flush;
        }

        // Flush the response.
        if let HandlerState::Flush = self.state {
            match self.connection.flush() {
                Ok(_) => {}
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => return None,
                Err(e) => panic!("{e}"),
            }
        }

        REACTOR.with(|reactor| {
            reactor.remove(self.connection.as_raw_fd());
        });

        Some(())
    }
}

/*

It is nice that tasks are encapsulated, but we still have to write everything in a state-machine like way.
Granted, Rust makes this pretty easy to do with enums, but could we do better?

*/
