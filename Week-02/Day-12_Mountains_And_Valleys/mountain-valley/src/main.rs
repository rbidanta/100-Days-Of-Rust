

fn is_mountain(input: &[i32]) -> bool {

    let mut inc = true;
    let mut peak: usize = 0;
    for i in 1..input.len() {

        if input[i] > input[i-1] {
            peak = i;
        }

        if !inc && input[i] > input[i-1]{
            return false
        }
        if input[i] < input[i-1] {
            inc = false;
        }
    }

    if peak == 0 || peak == input.len() - 1 {
        return false;
    }

    true
}


fn is_valley(input: &[i32]) -> bool {

    let mut dec = true;
    let mut trough = 0;
    for i in 1..input.len() {

        if input[i] < input[i-1] {
            trough = i;
        }

        if !dec && input[i] < input[i-1]{
            return false
        }
        if input[i] > input[i-1] {
            if dec {
                trough = i - 1;
            }
            dec = false;
        }
    }

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