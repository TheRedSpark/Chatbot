use mysql::params;
use mysql::prelude::Queryable;

pub mod variables;

#[derive(Debug, PartialEq, Eq)]
struct Message {
    sender_id: i32,
    retriever_id: i32,
    message_data: String,
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


