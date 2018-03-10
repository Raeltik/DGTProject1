use std::net::TcpListener;                                                                                                                                                      
use std::net::TcpStream;    
use std::io::prelude::*;

//use std::process::Command;                                                                                                                  
//use std::io::Write;                          



fn handle_json(mut stream: TcpStream){
	let mut buffer = String::new();

	stream.read_to_string(&mut buffer).unwrap();
//	let response = String::from_utf8( buffer ).unwrap();
	println!("{}",buffer)

}                                                                                                                                               
fn main() {
	
	let listener = TcpListener::bind("127.0.0.1:1234").unwrap();
	println!("Listening");
	listener.set_nonblocking(true)
		.expect("Can't set non-blocking");

	for stream in listener.incoming(){
		match stream {
			Ok(stream) => {
				println!("Incoming connection");
				let stream = stream;
				handle_json(stream)
			}
		Err(e) => {/*   Connection error handling*/}
		}

	}



}
