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

pub fn replace_all(string_slice:&str, c: char) -> String {
    let collected: String = string_slice.chars().filter(|x| *x != c).collect();
    return collected;
}

pub fn sort(s: &str) -> String {
    let mut out = String::new();
    let mut deleted_idx: Vec<usize> = vec!(); 
    let mut smallest_char: char = 'z';
    let mut smallest_idx: usize = 0;
    s.chars().for_each(|_| {
        s.chars().enumerate().for_each(|(a_idx, a)| {
            if deleted_idx.contains(&a_idx) {
                return;
            }
            if a < smallest_char {
                smallest_idx = a_idx;
                smallest_char = a;
            }
        });
        deleted_idx.push(smallest_idx);
        out.push(smallest_char);
        smallest_char = 'z';
    });

    return out
}

pub fn include_char(s: &str, c: char) -> bool {
    let res = s.chars().find(|ch| *ch == c);
    match res {
        Some(_) => return true,
        None => false
    }
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
    fn test_string_slice_experiment() {
        string_slice_experiment();
        assert!(true)
    }

    #[test]
    fn test_replace_all() {
        assert_eq!(replace_all("hello", 'l'), String::from("heo"));
    }

    #[test]
    fn test_sort() {
        assert_eq!(sort("hello"), String::from("ehllo"));
    }

    #[test]
    fn test_include_char() {
        assert_eq!(include_char("hello", 'o'), true);
        assert_eq!(include_char("hello", 'z'), false)
    }
}