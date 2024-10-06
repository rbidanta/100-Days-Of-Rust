
mod convert;

use std::io::{self, Write};

use crate::convert::age_in_days::*;

fn main() {
    loop {

        print!("Input age or press q to exit: ");
        let _ = io::stdout().flush().expect("Unable to run the program!!");
        let mut input = String::new();
        
        io::stdin()
        .read_line(&mut input)
        .expect("Unable to read input.");

        // Need to trim because console add return carraige as additonal bytes to the input
        if input.trim() == "q" {
            break;
        }
        
        let age_in_years: u32 = input.trim().parse().expect("Positive Integer expected");

        println!("Age {} years equals {} days",age_in_years, calculate_age_in_days(age_in_years))

    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_age_convert() {
        assert_eq!(calculate_age_in_days(0), 0);
        assert_eq!(calculate_age_in_days(1), 365);
        assert_eq!(calculate_age_in_days(5), 1825);
    }
}