
fn count_progress(miles: &[i32]) -> i32 {

    let mut progress_days = 0;

    for i in 1..miles.len() {
        if miles[i] > miles[i-1] {
            progress_days +=1;
        }
    }
    progress_days
}

fn main() {
    let miles = [3, 4, 1, 2];
    println!("Progress Days: {}", count_progress(&miles));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_increasing_progress() {
        let miles = [2, 3, 4, 5];
        assert_eq!(count_progress(&miles), 3);
    }

    #[test]
    fn test_decreasing_progress() {
        let miles = [22, 13, 11, 5];
        assert_eq!(count_progress(&miles), 0);
    }

    #[test]
    fn test_few_progress() {
        let miles = [10, 11, 12, 9, 10];
        assert_eq!(count_progress(&miles), 3);
    }

    #[test]
    fn test_one_progress() {
        let miles = [6, 5, 4, 3, 2, 9];
        assert_eq!(count_progress(&miles), 1);
    }

}
