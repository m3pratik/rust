
fn divide( a: i16, b: i16) -> Option<i16> {

    if b == 0 {
        None
    }else
    {
        Some(a/b)
    }

}


fn main() {
    println!("Let's Devide: ");

    let result1 = divide(8,0    );

    match result1 {
        Some(result1) => println!("Result is {}", result1),
        None => println!("Divide by Zero is not possible, Beta"),
    }
}
