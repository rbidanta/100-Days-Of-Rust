
fn validate_ip_section(section: &str) -> bool {


    let num: i32 = section.parse().expect("Invalid Number");

    let fist_char = section.chars().next().expect("Empty Char");

    if (fist_char == '0' && section.len() > 1) || num > 255 {
        return false
    }
    true
}


fn generate_all_ips(input: &str, sections: usize, temp: &mut Vec<String>, res: &mut Vec<String>) {

    if sections == 0 && input.len() == 0 {
        res.push(temp.join("."));
        return;
    }

    if sections == 0 || input.len() == 0 {
        return;
    }

    let mut n = 3;
    if input.len() < 3 {
        n = input.len();
    }

    for i in 1..=n {
        let section = &input[0..i];
        if !validate_ip_section(section) {
            return;
        }
        temp.push(section.to_string());
        generate_all_ips(&input[i..], sections-1, temp, res);
        temp.pop();
    }
}

fn solve(input: &str) -> Vec<String> {

    let mut res = vec![];
    let mut temp: Vec<String> = vec![];
    generate_all_ips(input, 4, &mut temp, &mut res);

    res
}


fn main() {
    let input = "25525511135";
    println!("{:?}", solve(input));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(solve("25525511135"), ["255.255.11.135","255.255.111.35"]);
    }

    #[test]
    fn test_two() {
        assert_eq!(solve("0000"), ["0.0.0.0"]);
    }

    #[test]
    fn test_three() {
        assert_eq!(solve("101023"), ["1.0.10.23", "1.0.102.3", "10.1.0.23" ,"10.10.2.3",  "101.0.2.3"]);
    }

    #[test]
    fn test_four() {
        assert_eq!(solve("010010"), ["0.10.0.10", "0.100.1.0"]);
    }

    #[test]
    fn test_five() {
        assert_eq!(solve("1111"), ["1.1.1.1"]);
    }
}