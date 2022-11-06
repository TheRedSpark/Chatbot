#![allow(unused)]

use std::net;
use std::net::{TcpStream, AddrParseError, Shutdown};
use std::io::{Empty, Error, Read, Write};
use std::str;
use std::{thread, time};
use std::fmt::Error as OtherError;
use std::io;

mod variables;

//fn get_credentials() {}

fn message_from_user() -> String {
    println!("Message start");
    let stdin = io::stdin();
    let mut message = String::new();
    println!("Bitte gib deine Nachricht an den Server an:");
    loop {
        stdin.read_line(&mut message);
        if message.len() > 240 {
            println!("Leider war deine Nachricht zu lang bitte versuche es nocheinmal und bitte \
            bleibe unter 240 Zeichen.");
            continue;
        } else { break; }
    }
    //message = "Testping".parse().unwrap();
    return message.to_owned();
}

fn communication(message: String) //-> io::Result<()>
{
    println!("Commonication Start");
    let mut buff_res = [0 as u8; 48];
    let server_ip = variables::mysql_ip("local".to_owned()).to_owned() + ":1111";
    println!("{:?}", server_ip);

    let client_id = "90000001";
    let empfanger_id = "90000002";
    let command = "9001";


    let mut stream = TcpStream::connect(server_ip)
        .expect("Couldn't connect to the server...");
    println!("Connectet");

    let buff_send = (client_id.to_owned() + empfanger_id + command + &message);

    println!("Der Client sendet: {}", buff_send);
    stream.write(buff_send.as_ref());
    stream.read(&mut buff_res).unwrap();
    println!("Der Server sendet: {}", str::from_utf8(&buff_res).unwrap());

    /*    if str::from_utf8(&buff_res).unwrap() != buff_send {
            println!("Beim senden ist was schiefegangen bitte versuche es erneut!")
        }*/


    stream.shutdown(Shutdown::Both).expect("shutdown call failed");
    thread::sleep(time::Duration::from_millis(500));
}

fn hauptmenu() -> i32 {
    loop {
        let mut terminal_input = String::new();
        io::stdin()
            .read_line(&mut terminal_input)
            .expect("Failed to read line");
        let desition: i32 = match terminal_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return desition;


    };
}


fn main() {
    println!("Anwendung gestartet");
    println!("Drücke 1 für den Nachrichtendienst");
    loop {

        match hauptmenu() {
            1 => communication(message_from_user()),
            2 => println!("TEst"),
            _ => println!("Das ist leider keine Richtige Option")
        }

    }



}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(true)
    }

    //assert_eq!(1,1);

    #[test]
    #[should_panic]
    fn it_works_not() {
        assert!(false);
    }
}