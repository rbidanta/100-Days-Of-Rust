fn merge_sorted<'a>(nums1: &'a mut [i32], nums2: &mut [i32], mut m: usize, mut n: usize) -> &'a [i32] {
    let mut total = m + n;
    loop {
        if nums1[m-1] >= nums2[n-1] {
            nums1[total-1] = nums1[m-1];
            total -= 1;
            m -= 1;
        } else {
            nums1[total-1] = nums2[n-1];
            total -= 1;
            n -= 1;
        } 

        if m == 0 || n == 0 {
            break;
        }
    }

    while n != 0 {
        nums1[total-1] = nums2[n-1];
        total -= 1;
        n -= 1;
    }

    nums1
}

fn main() {
    
    let mut nums1 = [1, 2, 3, 0, 0, 0];
    let mut nums2 = [2, 5, 6];

    merge_sorted(&mut nums1, &mut nums2, 3, 3);

    println!("{:?}", nums1);

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num1_less_than_nums2() {
        let mut nums1 = [1, 2, 3, 0, 0, 0];
        let mut nums2 = [4, 5, 6];
        assert_eq!(merge_sorted(&mut nums1, &mut nums2, 3, 3), [1, 2, 3, 4, 5, 6]);

    }


    #[test]
    fn test_num1_greater_than_nums2() {
        let mut nums1 = [4, 5, 6, 0, 0, 0];
        let mut nums2 = [1, 2, 2];
        assert_eq!(merge_sorted(&mut nums1, &mut nums2, 3, 3), [1, 2, 2, 4, 5, 6]);
    }


    #[test]
    fn test_num1_with_negative_nums() {
        let mut nums1 = [-5, -4, 6, 0, 0, 0];
        let mut nums2 = [1, 2, 2];
        assert_eq!(merge_sorted(&mut nums1, &mut nums2, 3, 3), [-5, -4, 1, 2, 2, 6]);
    }

    #[test]
    fn test_num2_with_negative_nums() {
        let mut nums1 = [-7, -5, 0, 0, 0, 0];
        let mut nums2 = [-4, -3, 6];
        assert_eq!(merge_sorted(&mut nums1, &mut nums2, 3, 3), [-7, -5, -4, -3, 0, 6]);
    }


    #[test]
    fn test_2() {
        let mut nums1 = [-5, -3, 0, 0, 0];
        let mut nums2 = [-4, -2, 1];
        assert_eq!(merge_sorted(&mut nums1, &mut nums2, 2, 3), [-5, -4, -3, -2, 1]);
    }

    #[test]
    fn test_3() {
        let mut nums1 = [1, 3, 5, 0, 0];
        let mut nums2 = [-2, 0];
        assert_eq!(merge_sorted(&mut nums1, &mut nums2, 3, 2), [-2, 0, 1, 3, 5]);
    }


}
