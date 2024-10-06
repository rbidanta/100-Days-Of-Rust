
mod convert;
use crate::convert::age_in_days::*;

#[test]
fn test_age_convert() {
    assert_eq!(calculate_age_in_days(0), 0);
    assert_eq!(calculate_age_in_days(1), 365);
    assert_eq!(calculate_age_in_days(5), 1825);
}


