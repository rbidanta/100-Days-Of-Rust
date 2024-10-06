fn count_skewers(skewers: &[&str]) -> Vec<u32> {
    let mut counts = [0u32; 2];
    
    for skewer in skewers.iter() {
        counts[skewer.contains("x") as usize] += 1;
    }

    counts.to_vec()
}

fn main() {
    let skewers = [
        "--oooo-ooo--",
        "--xx--x--xx--",
        "--o---o--oo--",
        "--xx--x--ox--",
        "--xx--x--ox--"
    ];

    println!("{:?}", count_skewers(&skewers));

}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_mixed_skewers() {
        let skewers = [
            "--oooo-ooo--",
            "--xx--x--xx--",
            "--o---o--oo--",
            "--xx--x--ox--",
            "--xx--x--ox--"
        ];
        assert_eq!(count_skewers(&skewers), [2, 3]);
    }

    #[test]
    fn test_variable_length_skewers() {
        let skewers = [
            "--oooo-ooo--",
            "--xxxxxxxx--",
            "--o---",
            "-o-----o---x--",
            "--o---o-----",
        ] ;
        assert_eq!(count_skewers(&skewers), [3, 2]);
    }

    #[test]
    fn test_vegeterian_skewers() {
        let skewers = [
            "--oooo-ooo--",
            "--o--oo---",
            "--o---",
            "-o-----o-----",
            "--o---o-----",
        ] ;
        assert_eq!(count_skewers(&skewers), [5, 0]);
    }

    #[test]
    fn test_non_vegeterian_skewers() {
        let skewers = [
            "--xxxx-xxx--",
            "--x--xx---",
            "--x---",
            "-x-----x-----",
            "--x---x-----",
        ] ;
        assert_eq!(count_skewers(&skewers), [0, 5]);
    }

}
