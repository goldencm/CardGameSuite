fn main() {
    println!("Count to what fib #?");
    let mut fib_num = String::new();
    std::io::stdin().read_line(&mut fib_num)
        .expect("Failed to recieve response!");

    let fib_num: u32 = match fib_num.trim().parse() {
        Ok(num) => num,
        Err(error) => { println!("Oh noes: {}", error); 0},
    };

    let mut count = 0;
    let mut fib_x = 0;
    let mut fib_y = 1;

    while count < fib_num + 1 {
        if count % 2 == 0 { 
            println!("{}", fib_x);
            fib_x += fib_y;
        } else {
            println!("{}", fib_y);
            fib_y += fib_x;
        }
        count += 1;
    }
}
