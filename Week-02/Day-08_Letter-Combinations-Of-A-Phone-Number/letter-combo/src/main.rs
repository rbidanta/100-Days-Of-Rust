
fn combine_letters(digits: &Vec<char>, mapping: &Vec<&str>, position: usize, res: &mut Vec<String>, curr: String ) {

    if position == digits.len() {
        res.push(curr);
        return;
    }

    let num = digits[position];
    let part = mapping[num.to_digit(10).unwrap() as usize];


    for c in part.chars() {
        combine_letters(digits, mapping, position + 1, res, curr.to_string() + &c.to_string());
    }
}

fn letter_combinations(input: String) ->  Vec<String> {

    let mapping = vec!["", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
    let mut res = vec![];
    let digits: Vec<char> = input.chars().collect();
    combine_letters(&digits, &mapping, 0, &mut res, "".to_string());
    res
}


fn letter_combinations_queue(input: String) -> Vec<String> {

    let mapping = vec!["", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
    let mut res = vec![];
    let mut queue: Vec<String> = vec!["".to_string()];
    let digits: Vec<char> = input.chars().collect();
    while queue.len() > 0 {

        let first = queue.remove(0);

        if first.len() == input.len() {
            res.push(first.to_string());
        } else { 
            let digit_alphabets = mapping[digits[first.len()].to_digit(10).unwrap() as usize];
            for ch in digit_alphabets.chars() {
                let part = first.to_string() + &ch.to_string();
                queue.push(part);
            }
        }
    }

    res
}

fn main() {
    let input = "23";
    assert_eq!(letter_combinations_queue(input.to_string()), letter_combinations(input.to_string()));
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_two_digit_number() {
        let input = "23";
        assert_eq!(letter_combinations(input.to_string()), ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]);
        assert_eq!(letter_combinations_queue(input.to_string()), ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]);
    }

    #[test]
    fn test_no_number() {
        let input = "";
        assert_eq!(letter_combinations(input.to_string()), [""]);
        assert_eq!(letter_combinations_queue(input.to_string()), [""]);
    }

    #[test]
    fn test_one_number() {
        let input = "2";
        assert_eq!(letter_combinations(input.to_string()), ["a", "b", "c"]);
        assert_eq!(letter_combinations_queue(input.to_string()), ["a", "b", "c"]);
    }


    #[test]
    fn test_three_number() {
        let input = "234";
        assert_eq!(letter_combinations(input.to_string()), ["adg", "adh", "adi", "aeg", "aeh", "aei", "afg", "afh", "afi", "bdg", "bdh", "bdi", "beg", "beh", "bei", "bfg", "bfh", "bfi", "cdg", "cdh", "cdi", "ceg", "ceh", "cei", "cfg", "cfh", "cfi"]);
        assert_eq!(letter_combinations_queue(input.to_string()), ["adg", "adh", "adi", "aeg", "aeh", "aei", "afg", "afh", "afi", "bdg", "bdh", "bdi", "beg", "beh", "bei", "bfg", "bfh", "bfi", "cdg", "cdh", "cdi", "ceg", "ceh", "cei", "cfg", "cfh", "cfi"]);

    }


}
