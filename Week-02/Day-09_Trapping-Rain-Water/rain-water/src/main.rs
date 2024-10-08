use std::cmp;


fn trap_rainwater(input: &[i32]) -> i32 {


    let mut stack: Vec<usize> = vec![];
    let mut res = 0;

    for i in 0..input.len() {
        while !stack.is_empty() && input[*stack.last().unwrap()] < input[i] {
            let last_greater = input[stack.pop().unwrap()];

            if stack.is_empty() {
                break;
            }

            let distance = i - stack.last().unwrap() - 1; 

            let mut water = cmp::min(input[*stack.last().unwrap()], input[i]);

            water -= last_greater;

            res += water * distance as i32;
        }

        stack.push(i);

        println!("{:?}", stack);

    }
    res
}

fn main() {
    let input = [0,1,0,2,1,0,1,3,2,1,2,1];
    println!("{:?}", trap_rainwater(&input));
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_case_one() {
        assert_eq!( trap_rainwater(&[0,1,0,2,1,0,1,3,2,1,2,1]), 6);
    }

    #[test]
    fn test_case_two() {
        assert_eq!( trap_rainwater(&[4,2,0,3,2,5]), 9);
    }

}



