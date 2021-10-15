fn main() {

    // 不可变变量x
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // shadow
    let x = 6;
    println!("The value of x is: {}", x);

    // 可变变量y
    let mut y = 0;
    println!("The value of y is: {}", y);
    y = 2147483647;
    println!("The value of y is: {}", y);

    // 常量Z
    const Z:i64 = 100_000;
    println!("The value of z is: {}", Z);
}
