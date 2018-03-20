use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

//use std::io::Write;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use serde_json::{Value};
//use serde_json::{Value, Error};
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
    println!("Reading input");
	stream.read_to_string(&mut buffer).unwrap();
//	let response = String::from_utf8( buffer ).unwrap();
    if buffer.chars().count() == 0 {
        println!("No input was given");
        let response = "No input was given";
        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        println!("Got the input");
	    println!("Recieved roughly {} characters",buffer.chars().count());
        println!("{}",buffer);
        let flag: Value = serde_json::from_str(&buffer).unwrap_or_default();   
        if flag.is_null(){
            println!("Bad JSON");
        }else{ 
            println!("{}",flag)
        }
    }
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
		Err(_e) => {/*   Connection error handling*/}
		}

	}



}
