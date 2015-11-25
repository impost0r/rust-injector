/* Rust linked library injector
   TODO : Fix GetWindowThreadProcessId
   		  Complete the damn code, tidy up.
  */

extern crate kernel32;
extern crate user32;
extern crate winapi;

use std::mem;
use std::u32;
use std::ptr;
use std::io::{self, BufRead};
use kernel32::OpenProcess;
use kernel32::GetProcessId;
use user32::GetWindowThreadProcessId;

fn get_input() -> String {
	let mut stdin = io::stdin();
	let mut buffer = String::new();
	stdin.read_line(&mut buffer).unwrap();
	buffer
}


fn main() {
	println!("Rust linked library injector");
	println!("Enter the process name:");
	let procName = get_input();
	println!("Process name: {}", procName);

	println!("Enter the absolute path to the library.");
	let dllPath = get_input();
	println!("Path -- {}", dllPath);

	let procNameVoid: *mut winapi::windef::HWND__ = unsafe {
		mem::transmute(&procName)
	};

	let mut reciever: *mut u32 = ptr::null_mut();

	unsafe { 
		GetWindowThreadProcessId(procNameVoid, reciever)
	};

	let recvStr: *mut String = unsafe {
		mem::transmute(&reciever)
	};

	println!("handle for {} -- {:?}", procName.trim(), recvStr);

	let recvVoid: *mut std::os::raw::c_void = unsafe {
		mem::transmute(&reciever)
	};

	let procID = unsafe {
		GetProcessId(recvVoid)
	};

	println!("pid for {} -- {:?}", procName.trim(), procID);



}