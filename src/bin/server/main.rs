use std::{
    thread,
    collections::HashMap,
    net::{TcpListener, TcpStream},
    sync::mpsc::{channel, Sender},
    io::{Write, BufReader, BufRead, BufWriter, ErrorKind}
};


fn main() {
    let server = Server::new("127.0.0.1:3000");

    server.run();
}


/// `Server` state structure.
///
/// While the server is running, it is necessary to keep the socket 
/// listening and the clients that have an active connection.
struct Server {
    listener: TcpListener,
    clients: HashMap<TcpStream, String>
}

/// Contains the type of action that occurred while processing the request.
#[derive(Debug)]
enum ActionType {
    ClientConnect,
    ClientDisconnect
}


/// `Action` message structure.
///
/// Each client is processed by a separate thread, so it is necessary 
/// to interaction between threads. This happens with `Action` structure. 
#[derive(Debug)]
struct Action {
    client: TcpStream,
    action_type: ActionType
}


impl Server {
    /// Create a new listener and return a `Server` instance.
    fn new(address: &str) -> Server {
        let listener = TcpListener::bind(address)
            .expect("Couldn't start a server");

        // This will result in the `accept` operation becoming 
        // nonblocking, immediately returning from their calls.
        listener.set_nonblocking(true)
            .expect("set_nonblocking call failed");

        Server {
            listener: listener,
            clients: HashMap::new()
        }
    }


    /// Accept a new incoming connections to the `Server`.
    fn run(&self) {
        // For each new client, the approach is to create a new thread 
        // to handle the client. The channel is used for communication 
        // between main thread of execution and all user processing threads.
        //
        // For each stream, a copy of the sender channel (tx - transceiver) 
        // is created, which sends information from the user's processing 
        // stream through this copy. 
        let (tx, rx) = channel::<Action>();

        loop {
            // Since the `accept` call does not block the thread of execution, 
            // we can process each iteration of the loop of new clients and 
            // messages from all clients.

            match self.listener.accept() {
                Ok((socket, _addr)) => {
                    let tx_thread_copy = tx.clone();

                    thread::spawn(move || {
                        Server::handle_connection(&socket, &tx_thread_copy);
                    });
                },
                Err(error) => {
                    match error.kind() {
                        ErrorKind::WouldBlock => (),
                        _ => ()
                    }
                }
            }

            for action in rx.try_iter() {
                println!("recieve action in rx: {:?}", action);
            }
        }
    }


    /// Handling a new client in a separate thread.
    fn handle_connection(stream: &TcpStream, tx: &Sender<Action>) {
        let mut reader = BufReader::new(stream);
        let mut writer = BufWriter::new(stream.try_clone().unwrap());

        tx.send(
            Action {
                client: stream.try_clone().unwrap(),
                action_type: ActionType::ClientConnect
            }
        ).unwrap();

        // Waiting for user input and posting a message to a channel.
        loop {
            let mut message = String::new();

            match reader.read_line(&mut message) {
                Ok(0) => {
                    // Upon receipt of 0 data from the client ended.
                    tx.send(
                        Action {
                            client: stream.try_clone().unwrap(),
                            action_type: ActionType::ClientDisconnect
                        }
                    ).unwrap();

                    break;
                },
                Ok(_) => {
                    // Echo server implementation.
                    println!("read: {:?}", message);

                    writer.write(message.as_bytes()).unwrap();
                    writer.flush().unwrap();
                },
                Err(error) => {
                    println!("Error handle connection: {:?}", error);
                }
            }
        }
    }
}
