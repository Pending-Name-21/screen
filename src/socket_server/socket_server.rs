use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::io::BufReader;

fn handle_client(stream: TcpStream) {
    let mut reader = BufReader::new(stream);
    let mut buffer = String::new();
    match reader.read_to_string(&mut buffer) {
        Ok(_) => println!("Datos recibidos: {}", buffer),
        Err(e) => println!("No se pudo leer los datos: {}", e),
    }
}

pub fn run_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Could not bind");
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            },
            Err(e) => {
                println!("Failed to accept connection: {}", e);
            }
        }
    }
}
