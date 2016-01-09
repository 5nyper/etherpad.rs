#![feature(io)]
extern crate winapi;
extern crate kernel32;
extern crate user32;

use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::ptr;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

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

fn set_data(foo: TcpStream) -> Result<i32, i32> {
	let bar: Vec<_> = foo.chars().map(|c| c.unwrap()).collect();
	let temp: String = bar.iter().cloned().collect::<String>();
	let result: &std::ffi::OsStr = OsStr::new(&temp).as_ref();
    unsafe {
		let buff: Vec<u16> = result.encode_wide().collect();
        let len: usize = (buff.len()+1) * 2;
		let open = user32::OpenClipboard(ptr::null_mut());
		if open == 0 {
			println!("couldnt open clipboard");
		}
		else {
        	let h_Mem = kernel32::GlobalAlloc(GMEM_MOVEABLE, len as u32);
			let data = kernel32::GlobalLock(h_Mem) as *mut u16;
			let len: usize = (len - 1) / 2;
            std::ptr::copy_nonoverlapping(buff.as_ptr(), data, len);
            let len: isize = len as isize;
            *data.offset(len) = 0;                                  //credits to clipboard-win for guidelines
			user32::EmptyClipboard();
			user32::SetClipboardData(13, h_Mem);
			kernel32::GlobalUnlock(h_Mem);
			user32::CloseClipboard();
		}
	}
	return Ok(0);
}
