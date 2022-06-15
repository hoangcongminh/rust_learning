fn main() {
    let decimal = 100_000;
    let hexa = 0xff_2e121;
    let octa = 0o77_1_23_412_21;
    let binary = 0b1111_0000_1111_0000;

    println!("Decimal: {}", decimal);
    println!("Hexa: {}", hexa);
    println!("Octa: {}", octa);
    println!("Binary: 0000_1111_0000{}", binary);

    //floating point types
    let x = 2.0;
    println!("Floating point: {} ", x);
    let x: f32 = 3.0;
    println!("Floating point: {} ", x);

    //operator
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    println!("sum: {} ", sum);
    println!("difference: {} ", difference);
    println!("product: {} ", product);
    println!("quotient: {} ", quotient);
    println!("floored:  {} ", floored);
    println!("remainder: {} ", remainder);

    //boolean type
    let t = true;
    let f: bool = false;

    println!("t: {}, f: {}", t, f);

    //character type
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    let sentence = "This is a 😻";
    let cat = match sentence.chars().nth(10) {
        Some(cat) => cat,
        None => 'n',
    };

    println!("{}\n{}\n{}\n", c, z, heart_eyed_cat);
    println!("{}\n{}", sentence, cat);

    //tuple
    let tup: (i32, f64, u8) = (200, 6.4, 1);
    let (x, y, z) = tup;

    println!("tuple: {}, {}, {}", x, y, z);
    println!("tuple: {}, {}, {}", tup.0, tup.1, tup.2);

    //array
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5]; //[3, 3, 3, 3, 3]

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    println!("Num: {}, Feb: {}", a[3], months[1]);
}
