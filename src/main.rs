use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
//use regex::Regex;
//use serde_json::{Value, Error};
//extern crate regex;
extern crate serde;
extern crate serde_json;


//use std::io::Write;


#[macro_use]
extern crate serde_derive;

use serde_json::{Value, Error};

#[derive(Deserialize, Debug)]
struct Flag {
    #[serde(default = "bad_flag")]
    user_key: String,

    #[serde(default = "bad_flag")]
    challenge_key: String,

    #[serde(default = "bad_flag")]
    flag: String
}

fn bad_flag() -> String{
    "did_not_submit".to_string()
}


fn handle_json(mut stream: TcpStream){

//    let re_flag = Regex::new(r"^flag(.*.)$").unwrap();
    let mut buffer = String::new();
    println!("Reading input");
    stream.read_to_string(&mut buffer).unwrap();
//	let response = String::from_utf8( buffer ).unwrap();
    if buffer.chars().count() == 0 {
        println!("No input was given");
        let response = "No input was given";
        let _ = stream.write_all(response.as_bytes());
    } else {
        println!("Got the input");
	    println!("Recieved roughly {} characters",buffer.chars().count());
        println!("{}",buffer);
        let flag_json: Flag = serde_json::from_str(&buffer).unwrap(); /*_or_default();*/
//        let flag_json: Flag = serde_json::from_str(&buffer).unwrap();
        println!("{:?}",flag_json);
        println!("Set flag_json");
        let flag_value = flag_json;
		println!("{:?}",flag_value["flag"])
//        match flag_value {
//            Ok(_) => println!("it worked"),
//            Err(err) => println!("{}",err)
//        }

//        if re_flag.is_match(flag_json["flag"].as_str().unwrap()) {
  //          println!("it worked")
    //        } else{ println!("It didn't work :<") }
       /* match flag_json["flag"]{ //this shit is BROKE UUUGGGGGGGGHHHHHH
            let response = "Flag Matched regex";
            println!("{}", response);
            let _ = stream.write_all(response.as_bytes());
        }else{
            println!("Good JSON: {}",flag_json);
            let response = "Good JSON!";
            let _ = stream.write_all(response.as_bytes());
        }*/
    }
}
fn main() {
    let listener = TcpListener::bind("127.0.0.1:1234").unwrap();
    println!("Listening");
    listener.set_nonblocking(true)
        .expect("Can't set non-blocking");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Incoming connection", );
                let stream = stream;
                handle_json(stream)
            }
            Err(_e) => { /*   Connection error handling  */ }
        }
    }
}
//    let json = r#"
//        [
//            {
//
//            }
//        ]"#;
//
//    let flag_json: Vec<Flag> = serde_json::from_str(json).unwrap_or_default();
//    println!("{:?}", flag_json)



