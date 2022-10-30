#![allow(unused)]

use std::net;
use std::net::{TcpStream, AddrParseError};
use std::io::{Empty, Error, Read, Write};
use std::str;
use std::{thread, time};
use std::fmt::Error as OtherError;
use std::io;

fn communication() //-> io::Result<()>
{
    let stdin = io::stdin();
    let server_ip = "127.0.0.1:80";
    use std::net::{Shutdown, TcpStream};
    let mut buff_res = [0 as u8; 48];
    let client_id = "90000001";
    let empfanger_id = "90000002";
    let mut message = String::new();
    //let mut message = "So weiter gehts because the blub is on the way to the moon and back say the kid";

    while true {
        stdin.read_line(&mut message);
        if message.len() > 240 {
            continue;
        } else { break; }
    }


    let mut stream = TcpStream::connect(server_ip)
        .expect("Couldn't connect to the server...");
    println!("Connectet");

    let buff_send = (client_id.to_owned() + empfanger_id + &message);

    println!("Der Client sendet: {}", buff_send);
    stream.write(buff_send.as_ref());
    stream.read(&mut buff_res).unwrap();
    println!("Der Server sendet: {}", str::from_utf8(&buff_res).unwrap());
    if str::from_utf8(&buff_res).unwrap() != buff_send {
        println!("Beim senden ist was schiefegangen")
    }


    stream.shutdown(Shutdown::Both).expect("shutdown call failed");
    thread::sleep(time::Duration::from_millis(500));
}

fn main() {
    communication();
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