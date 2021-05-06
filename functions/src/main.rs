fn main() {
    let x = puls_one(5);
    println!("The value of x is: {}",x);
}

fn puls_one(x: i32) -> i32 {
    x+1
}

fn another_function(x: i32,y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}