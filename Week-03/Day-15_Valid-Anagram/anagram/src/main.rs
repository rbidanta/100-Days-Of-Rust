use std::collections::HashMap;



fn check_anagram(source: &str, test: &str) -> bool {

    if source.len() != test.len() {
        return false;
    }

    let mut chars: HashMap<char, i32> = HashMap::new();

    for ch in source.chars() {
        chars.entry(ch).and_modify(|count| *count+=1).or_insert(1);
    }

    for ch in test.chars() {
        if !chars.contains_key(&ch) {
            return false;
        }else{
            chars.entry(ch).and_modify(|count| *count-=1);
            let val = chars.get(&ch).unwrap();
            if *val == 0 {
                chars.remove(&ch);
            }
        }
    }

    chars.is_empty()
}


fn main() {
    let source = "anagram".to_string();
    let test = "nagaram".to_string();

    let is_anagram = check_anagram(&source, &test);
    println!("Is {} an anagram of {}: {}", test, source, is_anagram );
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_one() {
        assert_eq!(check_anagram("anagram", "nagaram"), true);
    }

    #[test]
    fn test_two() {
        assert_eq!(check_anagram("cat", "rat"), false);
    }

    #[test]
    fn test_three() {
        assert_eq!(check_anagram("astronomer", "moonstarer"), true);
    }

    #[test]
    fn test_four() {
        assert_eq!(check_anagram("listen", "silent"), true);
    }
}

