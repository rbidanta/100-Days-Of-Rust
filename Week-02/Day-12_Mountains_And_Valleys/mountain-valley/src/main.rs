

fn is_mountain(input: &[i32]) -> bool {

    let mut inc = true;
    let mut peak: usize = 0;
    for i in 1..input.len() {

        // Keep tack of peak
        if input[i] > input[i-1] {
            peak = i;
            // If there are multiple peaks then its not a strict mountain
            // So return false
            if !inc {
                return false
            }
        }

        // Switch the slope direction when descending from peak
        if input[i] < input[i-1] {
            inc = false;
        }
    }

    // Edge cases as mountain cannot be on the border
    if peak == 0 || peak == input.len() - 1 {
        return false;
    }

    true
}


fn is_valley(input: &[i32]) -> bool {

    let mut dec = true;
    let mut trough = 0;
    for i in 1..input.len() {
        // Keep tack of trough

        if input[i] < input[i-1] {
            trough = i;
            // If there are multiple troughs then its not a strict valley
            // So return false
            if !dec {
                return false
            }
        }

        // Switch the slope direction when ascending from valley
        if input[i] > input[i-1] {
            dec = false;
        }
    }

    // Edge cases as valley cannot be on the border
    if trough == 0 || trough == input.len() - 1 {
        return false;
    }

    true
}


fn mountain_or_valley(input: &[i32]) -> String {

    if is_mountain(&input) {
        return "mountain".to_string();
    }

    if is_valley(&input) {
        return "valley".to_string();
    }

    return "neither".to_string();
}



fn main() {
    let input = [1, 3, 5, 4, 3, 2];
    println!("{}",mountain_or_valley(&input));
    let input = [10, 9, 8, 7, 2, 3, 4, 5];
    println!("{}", mountain_or_valley(&input));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mountain_one(){
        let  input = [1, 3, 5, 4, 3, 2];
        assert_eq!(mountain_or_valley(&input), "mountain");
    }

    #[test]
    fn test_mountain_two(){
        let  input = [-1, 0, -1];
        assert_eq!(mountain_or_valley(&input), "mountain");
    }

    #[test]
    fn test_mountain_three(){
        let  input = [-1, -1, 0, -1, -1];
        assert_eq!(mountain_or_valley(&input), "mountain");
    }

    #[test]
    fn test_valley_one(){
        let  input = [10, 9, 8, 7, 2, 3, 4, 5];
        assert_eq!(mountain_or_valley(&input), "valley");
    }


    #[test]
    fn test_valley_two(){
        let  input = [350, 100, 200, 400, 700];
        assert_eq!(mountain_or_valley(&input), "valley");
    }

    #[test]
    fn test_neither_one(){
        let  input = [1, 2, 3, 2, 4, 1];
        assert_eq!(mountain_or_valley(&input), "neither");
    }

    #[test]
    fn test_neither_two(){
        let  input = [5, 4, 3, 2, 1];
        assert_eq!(mountain_or_valley(&input), "neither");
    }

    #[test]
    fn test_neither_three(){
        let  input = [0, -1, -1, 0, -1, -1];
        assert_eq!(mountain_or_valley(&input), "neither");
    }

    #[test]
    fn test_neither_four(){
        let  input = [1, 2, 3, 4, 5, 6];
        assert_eq!(mountain_or_valley(&input), "neither");
    }

}