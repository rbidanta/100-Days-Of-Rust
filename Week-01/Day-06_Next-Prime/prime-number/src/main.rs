
fn next_prime(mut num: u32) -> u32 {

    while !is_prime(num) {
        num  += 1;
    }
    num
}

fn is_prime(num: u32) -> bool {

    if num <= 1 {
        return false;
    }

    if num <= 3 {
        return true;
    }

    if num % 2 == 0 || num % 3 == 0 {
        return false;
    }

    let mut i = 5;

    while i * i <= num {
        if num % i == 0 || num % (i + 2) == 0 {
            return false
        }
        i += 6;
    } 

    true

}

fn main() {
    let num = 23u32;
    println!("{}", next_prime(num));
}


#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn test_for_one() {
        assert_eq!(next_prime(1), 2);
    }

    #[test]
    fn test_for_zero() {
        assert_eq!(next_prime(0), 2);
    }

    #[test]
    fn test_for_five() {
        assert_eq!(next_prime(5), 5);
    }

    #[test]
    fn test_for_49() {
        assert_eq!(next_prime(49), 53);
    }

    #[test]
    fn test_for_12() {
        assert_eq!(next_prime(12), 13);
    }

    #[test]
    fn test_for_24() {
        assert_eq!(next_prime(24), 29);
    }

    #[test]
    fn test_for_11() {
        assert_eq!(next_prime(11), 11);
    }

}
