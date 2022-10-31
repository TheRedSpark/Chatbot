use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::str;
use crate::dataanbin::datenbank_putter;

mod dataanbin;

fn authentication_confirm() {
    dataanbin::datenbank_getter().expect("Das ist schiefgegangen");
}

fn handle_client(mut stream: TcpStream) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let mut buff = [0 as u8; 256];
    stream.read(&mut buff).unwrap();
    let sender_id_str = str::from_utf8(&buff[0..8]).unwrap();
    let sender_id: i32 = sender_id_str.parse().unwrap();
    let client_id_str = str::from_utf8(&buff[8..16]).unwrap();
    let client_id: i32 = client_id_str.parse().unwrap();
    let message = str::from_utf8(&buff[16..]).unwrap();
    println!("Echo is :{}", sender_id);
    println!("Client {} ist verbunden", client_id);
    println!("Data is: {}", message);
    //let data = handle_data(data.as_bytes());
    let response: String = format!("{}{}{}", sender_id, client_id, message);
    stream.write(response.as_ref()).unwrap();
    datenbank_putter(sender_id, client_id, message.to_string()).expect("Fehler bei der Eingabe der Datenbank");
    Ok(())
}

/*fn handle_data(incomming_data: &[u8]) -> String {
    let data: [u8; 16] = *match incomming_data {
        b"D000000000000001" => b"D000000000000002",
        &_ => b"Der000005555ror1"
    };
    let response = str::from_utf8(&data).unwrap().parse().unwrap();
    return response;
}*/


fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;
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
