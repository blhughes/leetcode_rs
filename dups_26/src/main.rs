fn main() {
    println!("Hello, world!");
    let mut x = vec![3,1,2,2,3];
    remove_element(&mut x, 3);
    println!("{:?}", x);

}

fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    
    //for (i,v) in nums.iter().enumerate() {
    for i in (0..nums.len()).rev() {

        if nums[i] == val {
            nums.remove(i);
        }

    }
    nums.len() as i32

}

