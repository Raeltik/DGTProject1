use std::net::TcpListener;                                                                                         
use std::net::TcpStream;    
use std::io::prelude::*;

//use std::process::Command;                                                                                                                  
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


#[test]
fn test_json(){
    let test_flag = r#"{
                        "user_key":"alkjv092itg92hf",
                        "challenge_key":"win-host-name",
                        "flag":"flag-chckyoself"
                       }"#;




    let tflag: Value = serde_json::from_str(test_flag).unwrap();
    let encoded_flag = String::from_utf8_lossy(tflag.as_slice());
    let mut test_stream = TcpStream::connect("127.0.0.1:1234").unwrap();
    let _ = test_stream.write(&tflag);
    




}


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
