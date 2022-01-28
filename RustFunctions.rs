use std::io;

fn div (x: f32, y: f32) -> f32 {
    x / y                               // this is an expression, it allows the function to return a value
}

fn main() {
    let mut x = String::new();
    println!("Enter a floating point number:");
    io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line");
    let x: f32 = x.trim().parse().expect("Please type a floating point number");   
    let mut y = String::new();
    println!("Enter a second floating point number:");
    io::stdin()
        .read_line(&mut y)
        .expect("Failed to read line");
    let y: f32 = y.trim().parse().expect("Please enter another floating point number");
    let sum = x + y;
    let diff = x - y;
    let prod = x * y;
    println!("The sum is: {}", sum);
    println!("The difference is: {}", diff);
    println!("The product is: {}", prod);
    println!("The division is: {}", div(x,y));
}