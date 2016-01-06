#![feature(io)]
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

fn main() {
	let server = TcpListener::bind("127.0.0.1:8080").unwrap();

	for stream in server.incoming() {
		match stream {
			Ok(stream) => {set_data(stream);}
			Err(err) => println!("Unable to do process {}", err)
		}
	}
}

fn set_data(foo: TcpStream) {
	//TODO: set clipboard
}
