fn main() {
    //branches
    let number = 3;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, 2");
    }

    let x = if true { 5 } else { 4 };

    println!("value: {}", x);

    //loop
    let mut count = 0;
    'counting_up: loop {
        println!("count: {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining: {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    //return value from loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        };
    };
    println!("The result is: {}", result);

    //while
    let mut number = 5;

    while number != 0 {
        println!("{}", number);
        number -= 1;
    }

    let a = [1, 2, 3, 4, 5];
    let mut index = 0;

    while index < a.len() {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    //for
    for element in a {
        println!("the value is: {}", element);
    }

    println!("Fibo 20: {}", fibonacci(10));

    for number in (1..4).rev() {
        println!("{}", number);
    }
}

fn fibonacci(n: i32) -> i128 {
    if n == 0 || n == 1 {
        return 1;
    } else {
        let mut num1 = 1;
        let mut num2 = 2;

        for _ in 2..n {
            let num3 = num1 + num2;
            num1 = num2;
            num2 = num3;
        }

        return num2;
    }
}
