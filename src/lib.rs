#[macro_export]
#[allow(dead_code)]
macro_rules! readln {
    () => {};
    ($arg:expr) => { std::io::stdin().read_line($arg) };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_readln() {
        let mut buffer: String = String::new();
        readln!(&mut buffer);
        assert!(buffer.len() != 0)
    }
}
