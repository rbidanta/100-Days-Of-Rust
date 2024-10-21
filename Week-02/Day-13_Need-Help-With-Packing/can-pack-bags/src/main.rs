


fn solve(nums: &[i32], bags: i32) -> bool {

    let mut total_weight = 0;

    for num in nums {
        total_weight += num;
    }

    let num_bags_required = (total_weight as f32) / 10.0;

    if num_bags_required.ceil() as i32 <= bags {
        return true;
    }
    false
}




fn main() {

    let nums = [2, 7, 1, 3, 3, 4, 7, 4, 1, 8, 2];
    let bags = 4;
    println!("{}", solve(&nums, bags));
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_one() {
        let nums = [2, 1, 2, 5, 4, 3, 6, 1, 1, 9, 3, 2];
        let bags = 4;
        assert_eq!(solve(&nums, bags), true);
    }


    #[test]
    fn test_two() {
        let nums = [2, 7, 1, 3, 3, 4, 7, 4, 1, 8, 2];
        let bags = 4;
        assert_eq!(solve(&nums, bags), false);
    }

}
