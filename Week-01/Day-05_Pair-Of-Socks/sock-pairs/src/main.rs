use std::collections::HashMap;

fn count_sock_pairs(sequence: &str) -> i32 {
    let mut sock_pairs: HashMap<String, i32> = HashMap::new();

    for ch in sequence.chars(){
        sock_pairs.entry(ch.to_string()).and_modify(|count| *count += 1).or_insert(1);
    }

    sock_pairs.values().map(|val| {
        if val % 2 == 0 {
            val/2
        } else {
            0i32
        }
    }).reduce(|acc, v| acc+v).unwrap_or_else(|| 0)
}


fn main() {
    let sequence = "AABBCBCB";
    println!("Number of pairs {}", count_sock_pairs(sequence));
}


#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn test_one_pair(){
        assert_eq!(count_sock_pairs("AA"), 1);
    }

    #[test]
    fn test_no_pair(){
        assert_eq!(count_sock_pairs("ABC"), 0);
    }

    #[test]
    fn test_multiple_pair(){
        assert_eq!(count_sock_pairs("AABBCC"), 3);
    }

    #[test]
    fn test_missing_pair(){
        assert_eq!(count_sock_pairs("ABABC"), 2);
    }

    #[test]
    fn test_no_socks(){
        assert_eq!(count_sock_pairs(""), 0);
    }

}