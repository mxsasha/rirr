#[macro_use]
extern crate diesel;
extern crate dotenv;

use ipnetwork::IpNetwork;
use sqldb::handler::*;
use std::io::BufRead;
use std::{
    io::{BufReader, Write},
    net::{TcpListener, TcpStream},
};

mod enums;
mod sqldb;
mod utils;

fn handle_client(mut stream: TcpStream) {
    let connection = establish_connection();
    let mut buf = String::new();
    let mut reader = BufReader::new(stream.try_clone().unwrap());

    while reader.read_line(&mut buf).unwrap_or_default() > 0 {
        println!("Received query {}", buf.trim());
        if buf.trim() == "!!" {
            buf = String::new();
            continue;
        }
        if buf.trim() == "!a" {
            stream.write(b"F Missing required set name for A query\n");
            buf = String::new();
            continue;
        }
        if buf.trim() == "!q" {
            return;
        }
        let members = members_for_as_set_recursive(&connection, &buf.trim()[3..]);
        println!("{:?}", members);
        let prefixes: Vec<IpNetwork> = prefixes_for_origins(&connection, &members);
        let response = prefixes
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<String>>()
            .join(" ")
            + "\n";
        let mut wrapper = String::new();
        write!(stream, "A{}\n{}C\n", response.len(), response);
        buf = String::new();
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4343")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        println!("New client: {:?}", stream);
        handle_client(stream?);
    }
    Ok(())
}
