fn main() {
    // print_labeled_measurement(10, 'h');

    // let x = five();
    // let x = plus_one(5);
    let x = sum(5, 3);

    println!("five: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn sum(first_num: i32, second_num: i32) -> i32 {
    let label_printer = print_labeled_measurement;
    label_printer(10, 'h');
    return first_num + second_num;
}

fn five() -> i32 {
    5 //dont have a comma at the end
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
