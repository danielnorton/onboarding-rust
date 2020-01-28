use onboarding_rust::week1::exercise1::parity;

#[test]
fn test_week1_exercise1_001() {

    let v1: Vec<i64> = vec![1,2,3,4,5,6,7,8,9];
    let t1: Vec<i64> = vec![2,4,6,8,1,3,5,7,9];
    assert_eq!(t1, parity(&v1).clone());
}

#[test]
fn test_week1_exercise1_002() {

    let v1: Vec<i64> = vec![5,7,3,1,5,8,9,6,45,3];
    let t1: Vec<i64> = vec![8,6,5,7,3,1,5,9,45,3];
    assert_eq!(t1, parity(&v1).clone());
}

#[test]
fn test_week1_exercise1_003() {

    let v1: Vec<i64> = vec![];
    let t1: Vec<i64> = vec![];
    assert_eq!(t1, parity(&v1).clone());
}
