use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::str;
use std::str::from_utf8;
use crate::dataanbin::datenbank_putter;
use crate::dataanbin::server_logging;

mod dataanbin;

fn authentication_confirm() {
    dataanbin::datenbank_getter().expect("Das ist schiefgegangen");
}

fn handle_client(mut stream: TcpStream) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut buff = [0 as u8; 256];
    stream.read(&mut buff).unwrap();
    let sender_id_str = str::from_utf8(&buff[0..8]).unwrap();
    let sender_id: i32 = sender_id_str.parse().unwrap();
    let second_inf_str = str::from_utf8(&buff[8..16]).unwrap();
    let second_inf: i32 = second_inf_str.parse().unwrap();
    let command_str = str::from_utf8(&buff[16..20]).unwrap();
    let command: i32 = command_str.parse().unwrap();
    let data_strem = str::from_utf8(&buff[20..]).unwrap();
    println!("Client {} ist verbunden", sender_id);
    println!("Secondärinformationen sind: {}", second_inf);
    println!("Der Befehl lautet: {}", handle_command(command));
    println!("Data is: {}", data_strem);
    //let data = handle_data(data.as_bytes());
    let response: String = format!("{}{}{}", sender_id, second_inf, data_strem);
    stream.write(response.as_ref()).unwrap();
    //server_logging(sender_id, second_inf, command, data_strem.to_owned()).expect("TODO: panic message beim logging in der main");
    datenbank_putter(sender_id, second_inf, data_strem.to_string()).expect("Fehler bei der Eingabe der Datenbank");
    Ok(())
}

fn handle_command(command: i32) -> String {
    let response = match command {
        9001 => "Incomming Message",
        9002 => "Authentification",
        _ => "The Problem"
    };
    return response.to_owned();
}


fn main() -> std::io::Result<()> {
    println!("Starten vom lissener");
    let listener = TcpListener::bind("127.0.0.1:1111")?;
    println!("Server gestartet");
    for stream in listener.incoming() {
        handle_client(stream?).expect("panic: handle_client Funktionsfehler");
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
