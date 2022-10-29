use mysql::prelude::Queryable;

pub mod variables;

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
    print!("{}", stringbuilder());
    let pool = mysql::Pool::new(&*stringbuilder())?;
    //let pool = Pool::new(url)?;

    let mut conn = pool.get_conn()?;

    // Let's create a table for payments.
    conn.query_drop(
        r"CREATE TABLE payment (
            customer_id int not null,
            amount int not null,
            account_name text
        )")?;

    println!("Yay!");

    Ok(())
}