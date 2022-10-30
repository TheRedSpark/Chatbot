use std::fmt::Display;
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str;
use mysql::*;
use mysql::prelude::*;

mod dataanbin;


fn handle_client(mut stream: TcpStream) {
    let mut buff = [0 as u8; 48];
    stream.read(&mut buff).unwrap();
    let echo_key = str::from_utf8(&buff[0..8]).unwrap();
    let client_id: &str = str::from_utf8(&buff[8..16]).unwrap();
    let data = str::from_utf8(&buff[16..32]).unwrap();
    let command = str::from_utf8(&buff[32..48]).unwrap();
    println!("Echo is :{}", echo_key);
    println!("Client {} ist verbunden", client_id);
    println!("Data is: {}", data);
    println!("Command is: {}", command);
    //let data = handle_data(data.as_bytes());
    let response: String = format!("{}{}{}{}", echo_key, client_id, data, command);
    stream.write(response.as_ref()).unwrap();
}

fn handle_data(incomming_data: &[u8]) -> String {
    let data: [u8; 16] = *match incomming_data {
        b"D000000000000001" => b"D000000000000002",
        &_ => b"Der000005555ror1"
    };
    let response = str::from_utf8(&data).unwrap().parse().unwrap();
    return response;
}


fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;

    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(true)
    }

    #[test]
    #[should_panic]
    fn it_works_not() {
        assert!(false);
    }
}
