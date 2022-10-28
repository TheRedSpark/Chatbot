#![allow(unused)]

// https://github.com/rust-lang/rustlings  Wichtig
use std::net;
use std::net::{TcpStream, AddrParseError};
use std::io::{Empty, Error, Read, Write};
//use std::simd::u8x16;
use std::str;
use std::{thread, time};
use std::fmt::Error as OtherError;

fn communication() {
    use std::net::{Shutdown, TcpStream};
    let mut buf = [0 as u8; 48];
    let client_id = b"c0000001";
    let empfanger_id = b"jejdkztb";
    let message = b"Hallo Boy i bims";
    let key = b"C000000000000001";
    let mut stream = TcpStream::connect("127.0.0.1:80")
        .expect("Couldn't connect to the server...");
    println!("Connectet");
    stream.write(key).unwrap();
    stream.write(client_id).unwrap();
    stream.write(empfanger_id).unwrap();
    stream.write(message).unwrap();
    stream.read(&mut buf).unwrap();
    println!("Der Befehl lautet: {}", str::from_utf8(&buf).unwrap());
    stream.shutdown(Shutdown::Both).expect("shutdown call failed");
    thread::sleep(time::Duration::from_millis(5000));
    let s = "Hat geklappt";
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