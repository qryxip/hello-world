fn main() {
    println!("{}", hello());
}

fn hello() -> &'static str {
    "Hello, world!"
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!("Hello, world!", crate::hello());
    }
}
