pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn split_string(s: &str, delimiter: char) -> Vec<String> {
    s.split(delimiter).map(|x|  x.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("hello"), "olleh");
    }

    #[test]
    fn test_split_string() {
        assert_eq!(split_string("a,b", ','), ["a", "b"])
    }
}