fn main() {
    println!("Hello, world!");
    let s1: String = "flower".to_string();
    let s2: String = "flow".to_string();
    let s3: String = "flight".to_string();
    let v: Vec<String> = vec!(s1,s2,s3); 
    println!("{:?}",longest_common_prefix(v));
}


fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut s: String = String::new();
    for i in 0..200 {
    let c = &strs[0].chars().nth(i).unwrap();
    for vs in &strs {
        if vs.chars().nth(i).unwrap() != *c{
            return s;
        }
    }
    s.push(*c)
    }
    s

}
