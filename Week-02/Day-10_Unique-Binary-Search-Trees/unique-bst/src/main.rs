use std::collections::HashMap;

fn count_bsts(n: i32, track: &mut HashMap<i32, i32>) -> i32 {
    if let Some(value) = track.get(&n){
        return *value;
    }

    let mut res = 0;
    for i in 1..=n {
        res += count_bsts(i - 1, track) * count_bsts(n - i, track)
    }
    track.insert(n, res);
    res
}

fn calculate(n: i32) -> i32 {
    let mut memo = HashMap::new();
    memo.insert(0, 1);
    memo.insert(1, 1);
    count_bsts(n, &mut memo)

}

fn main() {
    
    println!("{}",calculate(4));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_1_node() {
        assert_eq!(calculate(1), 1);
    }

    #[test]
    fn test_2_node() {
        assert_eq!(calculate(2), 2);
    }

    #[test]
    fn test_3_node() {
        assert_eq!(calculate(3), 5);
    }

    #[test]
    fn test_4_node() {
        assert_eq!(calculate(4), 14);
    }

    #[test]
    fn test_19_node() {
        assert_eq!(calculate(19), 1767263190);
    }
}

