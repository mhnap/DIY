use std::{
    io::{self, Read, Write},
    net::TcpListener,
};

enum ConnectionState {
    Read { request: [u8; 1024], read: usize },
    Write { response: &'static [u8], written: usize },
    Flush,
}

fn main() {
    // Bind the listener.
    let listener = TcpListener::bind("localhost:3000").unwrap();
    listener.set_nonblocking(true).unwrap();

    let mut connections = Vec::new();

    loop {
        // Try accepting a new connection.
        match listener.accept() {
            Ok((connection, _)) => {
                connection.set_nonblocking(true).unwrap();

                // Keep track of connection state.
                let state = ConnectionState::Read { request: [0u8; 1024], read: 0 };

                connections.push((connection, state));
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {}
            Err(e) => panic!("{e}"),
        }

        let mut completed = Vec::new();

        // Try to drive connections forward.
        'next: for (i, (connection, state)) in connections.iter_mut().enumerate() {
            if let ConnectionState::Read { request, read } = state {
                loop {
                    // Try reading from the stream.
                    match connection.read(&mut request[*read..]) {
                        Ok(0) => {
                            println!("client disconnected unexpectedly");
                            completed.push(i);
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
                            completed.push(i);
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
                        completed.push(i);
                    }
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                        // Not ready yet, move on to the next connection.
                        continue 'next;
                    }
                    Err(e) => panic!("{e}"),
                }
            }
        }

        // Remove any connections that completed, iterating in reverse order
        // to preserve the indices.
        for i in completed.into_iter().rev() {
            connections.remove(i);
        }
    }
}

/*

Our server can now handle running multiple requests concurrently on a single thread.
Nothing ever blocks.
If some operation would have blocked, it remembers the current state and moves on to run something else, much like the kernel scheduler was doing for us.
However, our new design introduces two new problems.

The first problem is that everything runs on the main thread, utilizing only a single CPU core.
We're doing the best we can to use that one core efficiently, but we're still only running a single thing at a time.
With threads spread across multiple cores, we could be doing much more.

There's a bigger problem though.

Our main loop isn't actually very efficient.

We're making an I/O request to the kernel for every single active connection, every single iteration of the loop, to check if it's ready.
A call to read or write, even if it returns WouldBlock and doesn't actually perform any I/O, is still a syscall.
Syscalls aren't cheap. We might have 10k active connections, but only 500 of them are ready.
Calling read or write 10k times when only 500 of them will actually make progress is a massive waste of CPU cycles.

As the number of connections scales, our loop becomes less and less efficient, wasting more time doing useless work.

*/
