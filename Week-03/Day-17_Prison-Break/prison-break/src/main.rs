

fn freed_prisoners(cells: &[i32]) -> i32 {

    if cells.is_empty() || cells[0] == 0 {
        return 0;
    }

    cells.iter().fold(0, |freed_count, &cell| {

        if (freed_count % 2 == 0 && cell == 1) || (freed_count % 2 == 1 && cell == 0) {
            freed_count + 1
        }else{
            freed_count
        }
    })
}


fn main() {
    println!("Hello, world!");
    let prison_cells = [1, 1, 0, 0, 0, 1, 0];

    println!("Number of prisoners freed {}", freed_prisoners(&prison_cells));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let cells = [1, 1, 0, 0, 0, 1, 0];
        assert_eq!(freed_prisoners(&cells), 4)
    }

    #[test]
    fn test_two() {
        let cells = [1, 1, 1];
        assert_eq!(freed_prisoners(&cells), 1)
    }

    #[test]
    fn test_three() {
        let cells = [0, 0, 0];
        assert_eq!(freed_prisoners(&cells), 0)
    }

    #[test]
    fn test_four() {
        let cells = [0, 1, 1, 1];
        assert_eq!(freed_prisoners(&cells), 0)
    }
}
