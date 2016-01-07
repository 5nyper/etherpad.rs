#![feature(io)]
extern crate winapi;
extern crate kernel32;
extern crate user32;

use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::ptr;
use std::ffi::CString;

const GMEM_MOVEABLE: u32 = 0x0002;

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
	let bar: Vec<_> = foo.chars().map(|c| c.unwrap()).collect();
	let temp = bar.iter().cloned().collect::<String>();
	let result =  CString::new(temp).unwrap();
    unsafe {
		let open = user32::OpenClipboard(ptr::null_mut());
		if open == 0 {
			println!("couldnt open clipboard");
		}
		else {
        	let h_Mem = kernel32::GlobalAlloc(GMEM_MOVEABLE, result.to_str().unwrap().len() as u32);
			let data = kernel32::GlobalLock(h_Mem);
			ptr::copy_nonoverlapping(result.as_ptr(), data as *mut i8, result.to_str().unwrap().len()); //look into
			user32::EmptyClipboard();
			user32::SetClipboardData(13, data);
			kernel32::GlobalUnlock(h_Mem);
			user32::CloseClipboard();
		}
	}
}
