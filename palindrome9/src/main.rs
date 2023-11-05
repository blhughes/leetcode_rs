fn main() {

    println!("{:?}", is_palindrome(121));
}


 fn is_palindrome(x: i32) -> bool {
    let _s: &str = &x.to_string();
    _s.chars().collect::<String>() == _s.chars().rev().collect::<String>() 
    }
