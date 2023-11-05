fn main() {
    println!("Hello, world!");
    let s: String = "({[]})".to_string();
    is_valid(s);
}


fn is_valid(s: String) -> bool {
    
    let mut v: Vec<char> = vec![];
    for c in s.chars(){
        println!("{}",c);
        match c {
            '(' | '{' | '[' => {
                v.push(c);
            }
            ')' | '}' | ']' => {
                let popped: Option<char> = v.pop();
                match popped {
                    Some(popped) => {

                        if popped != char::from_u32(c as u32 - 1).unwrap() | popped != char::from_u32(c as u32 - 2).unwrap(){
                            return false
                        }
                    }
                None => return false
                }
            }
            _ => ()
        }
    }
    println!("{:?}",v);
    if v.len() == 0 {
        return true
    }else{
        return false}
    }
