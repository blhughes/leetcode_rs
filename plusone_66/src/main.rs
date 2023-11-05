fn main() {
    println!("Hello, world!");
    let a = vec![9,9,9];
    println!("{:?}", plus_one(a));
}

fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut d = digits.clone();
    d.reverse();
    let mut carry: bool = true;
    for x in d.iter_mut(){
        if carry && *x  == 9 {
            *x = 0;
            carry = true;
            continue;
        }
        if carry { 
            *x +=1;
            carry =false;
        }
    }
    if carry {
        d.push(1);
    }
    d.reverse();
    d
}
    
