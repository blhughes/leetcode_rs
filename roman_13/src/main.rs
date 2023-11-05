
fn main() {
    println!("Hello, world!");
    println!("{}", roman_to_int("MCMXCIV".to_string()));
}


fn roman_to_int(s: String) -> i32 {
    println!("{}",s);
    let mut v: Vec<i32> = Vec::new();

    for x in s.chars(){
        let r: Option<i32>= match x {
        'I' => Some(1),
        'V' => Some(5),
        'X' => Some(10),
        'L' => Some(50),
        'C' => Some(100),
        'D' => Some(500),
        'M' => Some(1000),
        _ => None,
        };
        v.push(r.unwrap());
    }
    let mut acc: i32 = 0;

    for (i,x) in (&v).into_iter().enumerate(){
        let mut next: i32 = 0;
        if i+1 < v.len() {
            next = v[i+1];
        }
        if x < &next{
            acc -= x;
        }else{
            acc += x;
        }
    }

    acc
}
