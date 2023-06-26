use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const LOCAL_IP: &str = "127.0.0.1:7890";
const MESSAGE_SIZE: usize = 32;

fn main() {
    let server = TcpListener::bind(LOCAL_IP).expect("listener binding failed");
    server.set_nonblocking(true).expect("Failed non blocking server initialization");


    let mut clients = vec![];

    let (tx, rx) = mpsc::channel::<String>();
    loop {
        if let Ok((mut socket, address))= server.accept(){
            println!("Connected client {}", address);
            let tx = tx.clone();
            clients.push(socket.try_clone().expect("Socket cloning failed"));

            thread::spawn(move || {
                loop {
                    let mut buffer = vec![0; MESSAGE_SIZE];
                    match socket.read_exact(&mut buffer) {
                        Ok(_) => {
                            let msg = buffer.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                            let msg = String::from_utf8(msg).expect("Invalid UTF-8 message");
                            println!("Address {}, Message {}", address, msg);
                            tx.send(msg).expect("Failed to send message");
                        },
                        Err(ref err) if err.kind() == ErrorKind::WouldBlock => (), //do nothing for  blocking error
                        Err(_) => {  //all other errors
                            println!("Closing connection {}", address);
                            break;
                        }
                    }
                }
                sleep();
            });
        }

        if let Ok(msg) = rx.try_recv(){
            clients = clients.into_iter().filter_map(|mut client| {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MESSAGE_SIZE, 0);
                client.write_all(&buff).map(|_| client).ok()
            }).collect::<Vec<_>>();
        }
        sleep();
    }
}

fn sleep(){
    thread::sleep(Duration::from_millis(100));
}