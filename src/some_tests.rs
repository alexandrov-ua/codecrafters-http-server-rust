#![allow(dead_code)]

use std::error::Error;


fn some_foo() -> Result<i32, Box<dyn Error>> {
    let mut it = 1..2;
    Ok(it.next().ok_or_else(|| Box::new(std::io::Error::new(std::io::ErrorKind::Other, "No value found")))?)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_some_foo() {
        let res = some_foo().unwrap();
        assert_eq!(res, 1);
    }
}