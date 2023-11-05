
fn main() {
    println!("Hello, world!");
    let g = vec![1,2,3,4,5];
    let c = vec![3,4,5,1,2];
    println!("{}", can_complete_circuit(g, c));
    
}


fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    

        let c:Vec<i32> = gas.iter().zip(cost.iter()).map( | (j,k) | j-k ).collect();
        if c.iter().sum::<i32>() < 0 {
            return -1
        }
        if c.len() == 1{
            return 0
        }
        let mut jump:usize = 0;
        for i in 0..gas.len(){
            if i < jump { continue} 
            if c[i] <=0 {
                continue;
            }
            let (x,y) = c.split_at(i);
            let tank = [y,x].concat();
            let mut sum: i32 = 0;
            for (a,j) in tank.iter().enumerate() {
                sum += j;
                if sum < 0 {
                    jump = i + a;
                    break
                }
            }
            if sum >= 0{
                return i as i32;
            }
    }
    -1
    }

#[test]
fn test_circuit() {

let gas = vec![1,2,3,4,5];
let cost = vec![3,4,5,1,2];
    assert_eq!(3, can_complete_circuit(gas,cost));
    assert_eq!(0, can_complete_circuit(vec![2],vec![2]));
    assert_eq!(-1, can_complete_circuit(vec![2],vec![3]));


}

#[test]
fn test_c2(){
    let mut gas: Vec<i32> = Vec::new();
    let mut cost: Vec<i32> = Vec::new();

    for _i in 0..99999 {
        gas.push(0);
}
    gas.push(2);
    for _i in 0..99998 {
        cost.push(0);
    }   
    cost.push(1);
    cost.push(0);
    assert_eq!(99999, can_complete_circuit(gas,cost));
}

#[test]
fn test_c3(){
    let mut gas: Vec<i32> = Vec::new();
    let mut cost: Vec<i32> = Vec::new();

    for _i in 0..99998 {
        gas.push(1);
}
    gas.push(0);
    gas.push(1000);
    println!("{}", gas.len() );
    cost.push(0);
    for _i in 1..100000 {
        cost.push(0);
    }   
    for i in 1..10 {
        cost[(9999 * i) + (i-1)] = 10000;
    }
    cost[99998] = 10000;
    assert_eq!(99999 , can_complete_circuit(gas,cost));
}

    

