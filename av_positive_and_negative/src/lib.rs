fn avg_positive(arr: &[i32]) -> f32 {
    let positive = arr.iter().filter(|&x| *x > 0).collect::<Vec<_>>();
    let positive_count = positive.len(); 
    let avg_positive = positive.iter()
                            .fold(0, |acc, &i| acc + i) as f32 / positive_count as f32;  
    avg_positive
}


#[test]
fn test_positive() {
    assert_eq!(3.6, avg_positive(&[9,-1,-1,-4,1,5,-4,2,-3,1]));
}

