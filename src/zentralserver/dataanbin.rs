use mysql::params;
use mysql::prelude::Queryable;
use std::str::from_utf8;
use chrono;
use chrono::Utc;

pub mod variables;

#[derive(Debug, PartialEq, Eq)]
struct Message {
    sender_id: i32,
    retriever_id: i32,
    message_data: String,
}

struct Log {
    sender_id: i32,
    secondary_info:i32,
    command:String,
    data:String,

}

pub(crate) fn string_builder() -> String {
    let mysql_ipaddr = variables::mysql_ip("remote".to_string());
    let mysql_user = variables::mysql_user("test".to_string());
    let mysql_database = variables::mysql_database("test".to_string());
    let mysql_passwort = variables::mysql_passwort("remote".to_string());
    //println!("{}{}{}{}", mysql_database, mysql_user, mysql_ipaddr, mysql_passwort);
    let url: &str = &*format!("mysql://{mysql_user}:{mysql_passwort}@{mysql_ipaddr}:3306/{mysql_database}");
    return url.to_string();
}


pub(crate) fn server_logging(sender_id: i32, secondary_information: i32, command:i32, message_data: String)-> std::result::Result<(), Box<dyn std::error::Error>> {
    let pool = mysql::Pool::new(&*string_builder())?;
    let now = Utc::now();
    let stamp = now.naive_utc();
    let mut conn = pool.get_conn()?;
    conn.exec_drop(
        "insert into log (zeit, sender_id, secondary_info,command, message_data) values (:zeit, :sender_id, :secondary_info, :command, :message_data)",
        params! {
            "zeit" => stamp.to_string().to_owned(),
            "sender_id" => sender_id,
            "secondary_info" => secondary_information,
            "command" => command,
            "message_data" => &message_data,

        },
    ).expect("TODO: panic message beim loggin");
    println!("Yay! LOG");
    Ok(())
}

pub(crate) fn datenbank_putter(sender_id: i32, retriever_id: i32, message_data: String) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let pool = mysql::Pool::new(&*string_builder())?;
    let mut conn = pool.get_conn()?;

    conn.exec_drop(
        "insert into Message (sender_id, retriever_id, message_data) values (:sender_id, :retriever_id, :message_data)",
        params! {
            "sender_id" => sender_id,
            "retriever_id" => retriever_id,
            "message_data" => &message_data,
        },
    ).expect("TODO: panic message beim insert");
    println!("Yay!");
    Ok(())
}

pub(crate) fn datenbank_getter() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let pool = mysql::Pool::new(&*string_builder())?;
    let mut conn = pool.get_conn()?;
    let selected_payments = conn
        .query_map(
            "SELECT customer_id, amount, account_name from payment",
            |(sender_id, retriever_id, message_data)| {
                Message { sender_id, retriever_id, message_data }
            },
        )?;
    println!("Get from the Datenbank");
    println!("{:?}", selected_payments);
    println!("Yay!");

    Ok(())
}


