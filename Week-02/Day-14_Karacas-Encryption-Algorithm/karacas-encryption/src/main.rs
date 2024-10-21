use std::collections::HashMap;

fn is_vowel(ch: &char) -> bool {
    "aeiou".contains(*ch)
}

fn encrypt(input: &str) -> String {

    let vowels = HashMap::from([
        ('a', '0'),
        ('e', '1'),
        ('i', '2'),
        ('o', '2'),
        ('u', '3'),
    ]);

    let mut res = "".to_string();

    let chars = input.chars().rev();

    for ch in chars {
        if is_vowel(&ch) {
            res.push(*vowels.get(&ch).unwrap());
        }else{
            res.push(ch);
        }
    }

    res + "aca"

}


fn main() {
    let data = "in a quiet little town nestled between rolling hills and lush green forests, life moved at a gentle pace. the sun rose each morning, casting a warm golden glow over the quaint houses and cobblestone streets. children played in the park, their laughter echoing through the air as they chased butterflies and climbed trees. nearby, an old man sat on a bench, feeding the birds with crumbs from his lunch. he often shared stories of his youth with anyone who would listen, tales of adventure and mischief that captivated the hearts of those around him.
as the seasons changed, so did the town. spring brought vibrant flowers that painted the landscape in hues of pink and yellow, while summer filled the days with sunshine and warmth. autumn transformed the trees into a tapestry of red and orange, and winter blanketed everything in soft white snow. each season held its own charm, drawing visitors from afar who sought to experience the magic of this idyllic place.
the townspeople were friendly and welcoming, always ready to lend a helping hand or share a smile. they gathered for festivals and celebrations, where music filled the air and delicious food was shared among friends. it was a community built on love, kindness, and the simple joys of life.";
    println!("Result: {}",encrypt(data));
    println!("Original Data: {}", data);
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_one() {
        let data = "banana";
        assert_eq!(encrypt(data), "0n0n0baca");
    }

    #[test]
    fn test_two() {
        let data = "karaca";
        assert_eq!(encrypt(data), "0c0r0kaca");
    }

    #[test]
    fn test_three() {
        let data = "burak";
        assert_eq!(encrypt(data), "k0r3baca");
    }

    #[test]
    fn test_four() {
        let data = "alpaca";
        assert_eq!(encrypt(data), "0c0pl0aca");
    }
}


