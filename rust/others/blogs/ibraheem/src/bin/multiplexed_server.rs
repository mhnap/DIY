use std::{
    collections::HashMap,
    io::{self, Read, Write},
    net::TcpListener,
    os::fd::AsRawFd,
};

enum ConnectionState {
    Read { request: [u8; 1024], read: usize },
    Write { response: &'static [u8], written: usize },
    Flush,
}

fn main() {
    // Create an epoll instance.
    let epoll = epoll::create(false).unwrap();

    // Bind the listener.
    let listener = TcpListener::bind("localhost:3000").unwrap();
    listener.set_nonblocking(true).unwrap();

    // Add the listener to epoll.
    let event = epoll::Event::new(epoll::Events::EPOLLIN, listener.as_raw_fd() as _);
    epoll::ctl(epoll, epoll::ControlOptions::EPOLL_CTL_ADD, listener.as_raw_fd(), event).unwrap();

    let mut connections = HashMap::new();

    loop {
        let mut events = [epoll::Event::new(epoll::Events::empty(), 0); 1024];

        let timeout = -1; // Block forever, until something happens.
        let num_events = epoll::wait(epoll, timeout, &mut events).unwrap();
        let mut completed = Vec::new();

        // The fact that epoll::wait is "blocking" might put you off, but remember,
        // it only blocks if there is nothing else to do, where previously we would have been spinning and making pointless syscalls.
        // This idea of blocking on multiple operations simultaneously is known as I/O multiplexing.

        'next: for event in &events[..num_events] {
            let fd = event.data as i32;

            // Is the listener ready?
            if fd == listener.as_raw_fd() {
                // Try accepting a new connection.
                match listener.accept() {
                    Ok((connection, _)) => {
                        connection.set_nonblocking(true).unwrap();
                        let fd = connection.as_raw_fd();

                        // Register the connection with epoll.
                        let event = epoll::Event::new(
                            epoll::Events::EPOLLIN | epoll::Events::EPOLLOUT,
                            fd as _,
                        );
                        epoll::ctl(epoll, epoll::ControlOptions::EPOLL_CTL_ADD, fd, event).unwrap();

                        // Keep track of connection state.
                        let state = ConnectionState::Read { request: [0u8; 1024], read: 0 };

                        connections.insert(fd, (connection, state));
                    }
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => {}
                    Err(e) => panic!("{e}"),
                }

                // This `fd` is for the listener, not for a connection.
                continue 'next;
            }

            // epoll told us this connection is ready.
            let (connection, state) = connections.get_mut(&fd).unwrap();

            // Try to drive it forward based on its state.
            if let ConnectionState::Read { request, read } = state {
                loop {
                    // Try reading from the stream.
                    match connection.read(&mut request[*read..]) {
                        Ok(0) => {
                            println!("client disconnected unexpectedly");
                            completed.push(fd);
                            continue 'next;
                        }
                        Ok(n) => {
                            // Keep track of how many bytes we've read.
                            *read += n
                        }
                        Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                            // Not ready yet, move on to the next connection.
                            continue 'next;
                        }
                        Err(e) => panic!("{e}"),
                    }

                    // Did we reach the end of the request?
                    if request.get(*read - 4..*read) == Some(b"\r\n\r\n") {
                        break;
                    }
                }

                // We're done, print the request.
                let request = String::from_utf8_lossy(&request[..*read]);
                println!("{request}");

                // Move into the write state.
                let response = concat!(
                    "HTTP/1.1 200 OK\r\n",
                    "Content-Length: 12\n",
                    "Connection: close\r\n\r\n",
                    "Hello world!"
                );

                *state = ConnectionState::Write { response: response.as_bytes(), written: 0 };
            }

            if let ConnectionState::Write { response, written } = state {
                loop {
                    match connection.write(&response[*written..]) {
                        Ok(0) => {
                            println!("client disconnected unexpectedly");
                            completed.push(fd);
                            continue 'next;
                        }
                        Ok(n) => {
                            *written += n;
                        }
                        Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                            // Not ready yet, move on to the next connection.
                            continue 'next;
                        }
                        Err(e) => panic!("{e}"),
                    }

                    // Did we write the whole response yet?
                    if *written == response.len() {
                        break;
                    }
                }

                // Successfully wrote the response, try flushing next.
                *state = ConnectionState::Flush;
            }

            if let ConnectionState::Flush = state {
                match connection.flush() {
                    Ok(_) => {
                        completed.push(fd);
                    }
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                        // Not ready yet, move on to the next connection.
                        continue 'next;
                    }
                    Err(e) => panic!("{e}"),
                }
            }
        }

        // Remove any connections that completed.
        for fd in completed {
            let (connection, _state) = connections.remove(&fd).unwrap();
            // Unregister from epoll.
            drop(connection);
        }
    }
}

/*

What if we could write an abstraction like thread::spawn that let us write our tasks as individual units,
and handle the scheduling and event handling for all tasks in a single place, regaining some of that sequential control flow?

This idea is generally referred to as asynchronous programming.

*/
