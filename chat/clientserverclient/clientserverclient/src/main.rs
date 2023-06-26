use std::net::{ TcpStream};
use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;
use std::{io, thread};
use std::io::{ErrorKind, Read, Write};
use std::time::Duration;

const LOCAL_IP: &str = "127.0.0.1:7890";
const MESSAGE_SIZE: usize = 32;

fn main() {
    let mut client = TcpStream::connect(LOCAL_IP).expect("clinent failed to connect");
    client.set_nonblocking(true).expect("Failed non blocking server initialization");

    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut buffer = vec![0; MESSAGE_SIZE];
        match client.read_exact(&mut buffer) {
            Ok(_) => {
                //take non empty messages
                let msg = buffer.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                println!("Message =  {:?}", msg);
            },
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (), //do nothing for  blocking error
            Err(_) => {  //all other errors
                println!("Closing connection");
                break;
            }
        }
        match rx.try_recv() {
            Ok(msg) => {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MESSAGE_SIZE, 0);
                client.write_all(&buff).expect("Writing to socket failed");
                println!("Message sent {:?}", msg);
            },
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break
        }
        thread::sleep(Duration::from_millis(100));
    });

    println!("Write a message to send");

    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Reading failed");
        let usermessage = buffer.trim().to_string();
        if usermessage== ":quit" || tx.send(usermessage).is_err(){break}
    }
    println!("Exit");
}
