pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn split_string(s: &str, delimiter: char) -> Vec<String> {
    s.split(delimiter).map(|x|  x.to_string()).collect()
}

pub fn string_slice_experiment() {
    let s1: &str = "Hello";
    let s2: &str = "Hello World";

    println!("Address of s1: {:p}", s1); // s1의 메모리 주소 0x...bf
    println!("Address of s2: {:p}", s2); // s2에서 "Hello"의 주소 0x...c4
    println!("Address of s2: {:p}", &s2[0..5]); // s2에서 "Hello"의 주소 0x...c4
    // diffrent. 컴파일러 최적화에 따름
  
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

    #[test]
    fn test_replace_all() {
        string_slice_experiment();
        assert!(true)
    }
}