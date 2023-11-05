fn main() {
    println!("Hello, world!");
}


fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
       1 
}

#[test]
fn test_arrows(){

    assert_eq!(2, find_min_arrow_shots(vec![vec![10,16],vec![2,8],vec![1,6],vec![7,12]]));
    assert_eq!(4, find_min_arrow_shots(vec![vec![1,2],vec![3,4],vec![5,6],vec![7,8]]));
    assert_eq!(2, find_min_arrow_shots(vec![vec![1,2],vec![2,3],vec![3,4],vec![4,5]]));
}

