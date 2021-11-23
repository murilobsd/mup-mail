#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Address(pub String);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
