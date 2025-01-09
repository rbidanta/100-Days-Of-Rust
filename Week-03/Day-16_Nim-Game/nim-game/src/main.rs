
fn can_i_win(heap_size: u32) -> bool {
    if heap_size%4 == 0 {
        return false;
    }
    true
}


fn main() {
    println!("Can I win: {}", can_i_win(5) );
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(can_i_win(4), false);
    }


    #[test]
    fn test_two() {
        assert_eq!(can_i_win(6), true);
    }

    #[test]
    fn test_three() {
        assert_eq!(can_i_win(28), false);
    }

    #[test]
    fn test_four() {
        assert_eq!(can_i_win(100), false);
    }

    #[test]
    fn test_five() {
        assert_eq!(can_i_win(101), true);
    }
}
