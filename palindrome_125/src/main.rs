fn main() {
    println!("Hello, world!");
}


fn is_palindrome(s: String) -> bool {
    let mut s2 = String::new();
    let mut v1:Vec<char> = Vec::new();
    let mut v2:Vec<char> = Vec::new();
    for seg in s.split_ascii_whitespace(){
        s2 = s2+ &seg.to_lowercase();
    }
    s2.split_ascii_whitespace
    for c in s2.chars(){
        v1.push(c);
        v2.push(c);
    }
    v1.reverse();
    if v1 == v2{
        return true
    }
    false
}
