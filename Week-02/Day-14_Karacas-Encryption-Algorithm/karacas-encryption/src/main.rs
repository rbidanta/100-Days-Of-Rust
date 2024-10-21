use std::collections::HashMap;

fn is_vowel(ch: &char) -> bool {

    if ['a', 'e', 'i', 'o', 'u'].contains(ch) {
        return true;
    }
    false
}

fn encrypt(input: &str) -> String {

    let vowels = HashMap::from([
        ('a', '0'),
        ('e', '1'),
        ('i', '2'),
        ('o', '2'),
        ('u', '3'),
    ]);

    let mut res: Vec<char> = vec![];

    let chars: Vec<char> = input.chars().collect();

    let mut it = input.len();

    while it > 0  {
        let ch = chars.get(it-1).unwrap();
        if is_vowel(&ch) {
            res.push(*vowels.get(&ch).unwrap());
        } else {
            res.push(*ch);
        }
        it -= 1;
    } 

    println!("{:?}", res);

    let chars_as_string: Vec<String> = res.iter().map(|ch| ch.to_string()).collect();

    chars_as_string.join("") + "aca"

}


fn main() {
    let data = "Hello, world!";
    println!("Result: {}",encrypt(data));
    println!("Original Data: {}", data);
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_one() {
        let data = "banana";
        assert_eq!(encrypt(data), "0n0n0baca");
    }

    #[test]
    fn test_two() {
        let data = "karaca";
        assert_eq!(encrypt(data), "0c0r0kaca");
    }

    #[test]
    fn test_three() {
        let data = "burak";
        assert_eq!(encrypt(data), "k0r3baca");
    }

    #[test]
    fn test_four() {
        let data = "alpaca";
        assert_eq!(encrypt(data), "0c0pl0aca");
    }
}


