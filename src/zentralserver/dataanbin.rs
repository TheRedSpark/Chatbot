use mysql::prelude::Queryable;

pub mod variables;

#[derive(Debug, PartialEq, Eq)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
}

pub(crate) fn stringbuilder() -> String {
    let mysql_ipaddr = variables::mysql_ip("remote".to_string());
    let mysql_user = variables::mysql_user("remote".to_string());
    let mysql_database = variables::mysql_database("test".to_string());
    let mysql_passwort = variables::mysql_passwort("remote".to_string());
    println!("{}{}{}{}", mysql_database, mysql_user, mysql_ipaddr, mysql_passwort);
    let url: &str = &*format!("mysql://{mysql_user}:{mysql_passwort}@{mysql_ipaddr}:3306/{mysql_database}");
    return url.to_string();
}

pub(crate) fn datenbananbindung() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let pool = mysql::Pool::new(&*stringbuilder())?;

    let mut conn = pool.get_conn()?;

    // Let's create a table for payments.

    let selected_payments = conn
        .query_map(
            "SELECT customer_id, amount, account_name from payment",
            |(customer_id, amount, account_name)| {
                Payment { customer_id, amount, account_name }
            },
        )?;

    println!("{:?}", selected_payments);

    println!("Yay!");

    Ok(())
}