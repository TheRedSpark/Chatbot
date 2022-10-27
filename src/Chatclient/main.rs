fn main() {
    println!("Hello, Chatclient!");
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