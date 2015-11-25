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
use user32::GetWindowThreadProcessId;

//woo hardcore copypaste 
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
	//Remove the above, and "handle" returned is a null pointer. Perhaps GetWindowThreadProcessId function
	// is not complete, or I have to do something else to recieve the window handle. Check MSDN docs on
	// GetWindowThreadProcessId ()

	println!("pid for {} -- {:?}", procName.trim(), recvStr);
	//Above returns a memory address...?

	//Upon review of the MSDN documentation, GetWindowThreadProcessId does indeed return a PID. GetProcessId 
	//is no longer needed.
	



}