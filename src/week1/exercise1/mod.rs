pub fn parity(source: &Vec<i64>) -> Vec<i64> {
    
    // Given an array A of non-negative integers, return an array 
    // consisting of all the even elements of A, followed by all 
    // the odd elements of A.

    let even: Vec<i64> = source.iter().cloned().filter(|x| *x % 2 == 0).collect();
    let odd: Vec<i64> = source.iter().cloned().filter(|x| *x % 2 == 1).collect();

    return [&even[..], &odd[..]].concat();
}
