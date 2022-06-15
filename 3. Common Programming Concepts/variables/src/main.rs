fn main() {
    //variables and mutability
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 7;
    println!("The value of x is: {}", x);

    //constants
    const THREE_HOURS_IN_SECOND: u32 = 60 * 60 * 3;

    println!("Const value: {}", THREE_HOURS_IN_SECOND);

    //shadowing
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is:  {}", x);
    }

    println!("The value of x is:  {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
}
