use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

//use std::io::Write;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use serde_json::{Value, Error};
/*
#[derive(Serialize, Deserialize)]
struct Flag {
    user_key: String,
    challenge_key: String,
    flag: String,
}
*/




fn handle_json(mut stream: TcpStream){
	let mut buffer = String::new();

	stream.read_to_string(&mut buffer).unwrap();
//	let response = String::from_utf8( buffer ).unwrap();
	//println!("{}",buffer)
    let flag: Value = serde_json::from_str(&buffer).unwrap();
    println!("{}",flag);
    println!("{} {} {}",flag["user_key"],flag["challenge_key"],flag["flag"])

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
