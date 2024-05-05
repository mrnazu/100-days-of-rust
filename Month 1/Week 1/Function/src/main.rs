fn main() {
    println!("Day 5: Functions");
    let z: i32 = my_func(22);
    println!("my_func returned: {}", z);
    // Functions are defined using the "fn" keywords followed by the name of function
}

fn my_func(x: i32)  -> i32 {
    println!("my_func called with: {}", x);
    let y: i32 = 10;
    y
}
