use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::mpsc;
use std::io::Write;
use std::io::BufReader;
use std::io::BufRead;
use std::io::BufWriter;
use std::io::ErrorKind;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    listener.set_nonblocking(true).unwrap();

    println!("Running on port 3000");

    let (tx, rx) = mpsc::channel::<String>();

    loop {
        match listener.accept() {
            Ok((socket, _addr)) => {
                tx.send(String::from("new client")).unwrap();

                thread::spawn(move || {
                    handle_connection(&socket);
                });
            },
            Err(error) => {
                match error.kind() {
                    ErrorKind::WouldBlock => (),
                    _ => ()
                }
            }
        }

        match rx.try_recv() {
            Ok(message) => {
                println!("recieve message in rx: {:?}", message);
            }
            Err(_) => ()
        }
    }
}


fn handle_connection(stream: &TcpStream) {
    let mut message = String::new();
    let mut reader = BufReader::new(stream);
    let mut writer = BufWriter::new(stream.try_clone().unwrap());

    loop {
        match reader.read_line(&mut message) {
            Ok(0) => {
                println!("client disconnect");
                break;
            },
            Ok(_) => {
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
