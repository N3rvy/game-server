use std::{net::TcpListener, thread};
use tungstenite::{
    server::accept,
};

fn main() {
    let server = TcpListener::bind("127.0.0.1").unwrap();

    for stream in server.incoming() {
        thread::spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();

            loop {
                let message = websocket.read_message().unwrap();
                if message.is_binary() || message.is_text() {
                    websocket.write_message(message).unwrap();
                }
            }
        });
    }
}