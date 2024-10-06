
fn find_nemo(sentence: &str) -> String {

    let words: Vec<&str> = sentence.split(' ').collect();

    let mut found_position = 0; 

    for word in words.iter() {
        if *word == "Nemo" {
            break;
        }
        found_position+=1;
    }

    if found_position < (words.len()) {
        format!("I found Nemo at {:?}!", found_position+1)
    }else{
        format!("I can't find Nemo :(")
    }
    
}

fn main() {
    let given_sentence = String::from("I am finding Nemo !");
    println!("{}",find_nemo(&given_sentence));
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_finding_nemo() {
        let sentence = String::from("I am finding Nemo !");
        assert_eq!(find_nemo(&sentence), "I found Nemo at 4!");
    }

    #[test]
    fn test_not_found_nemo() {
        let sentence = String::from("Did you find Nemo?");
        assert_eq!(find_nemo(&sentence), "I can't find Nemo :(");
    }

    #[test]
    fn test_not_found_nemo_when_letters_do_not_match() {
        let sentence = String::from("I am finding NeMo !");
        assert_eq!(find_nemo(&sentence), "I can't find Nemo :(");
    }

    #[test]
    fn test_multiple_nemo_in_sentence() {
        let sentence = String::from("Nemo finding Nemo !");
        assert_eq!(find_nemo(&sentence), "I found Nemo at 1!");
    }

}


