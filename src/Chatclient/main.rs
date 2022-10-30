#![allow(unused)]
use std::net;
use std::net::{TcpStream, AddrParseError};
use std::io::{Empty, Error, Read, Write};
use std::str;
use std::{thread, time};
use std::fmt::Error as OtherError;


fn communication() {
    let server_ip = "127.0.0.1:80";
    use std::net::{Shutdown, TcpStream};
    let mut buffer = [0 as u8; 48];
    let client_id = "Clientid";
    let empfanger_id = "Empfanid";
    let message = "Hallo Boy i bims";
    let key = "The komische key";
    let mut stream = TcpStream::connect(server_ip)
        .expect("Couldn't connect to the server...");
    println!("Connectet");


    let mut buff_temp = [0 as u8; 48];
    let hjadsg = "id";
    let buff_temp2 = (client_id.to_owned()+empfanger_id+message+key);
    println!("Der Client sendet: {}",buff_temp2);

    stream.write(buff_temp2.as_ref());
/*    stream.write(client_id).unwrap();
    stream.write(empfanger_id).unwrap();
    stream.write(message).unwrap();
    stream.write(key).unwrap();*/
    stream.read(&mut buffer).unwrap();
    println!("Der Server sendet: {}", str::from_utf8(&buffer).unwrap());
    //let response: String = format!("{:?}{}{}{}", str::from_utf8(client_id).unwrap(), &empfanger_id, &message, &key);
    //let echo_key = str::from_utf8(&buff[0..8]).unwrap();
    //stream.write(response.as_ref()).unwrap();


    stream.shutdown(Shutdown::Both).expect("shutdown call failed");
    //thread::sleep(time::Duration::from_millis(5000));
    let s = "Hat geklappt";
    println!("{}",s);
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