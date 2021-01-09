
fn main() {
    println!("'F'ahrenheit or 'C'elcius");

    let mut response = String::new();

    std::io::stdin().read_line(&mut response)
            .expect("Failed to recieve response!");


    println!("Whats the temperature?");
    let mut temp = String::new();
    std::io::stdin().read_line(&mut temp)
        .expect("Failed to recieve response!");

    let temp: f32 = match temp.trim().parse() {
        Ok(num) => num,
        Err(error) => { println!("Oh noes: {}", error); 0.0},
    };

    println!("{}", -64 * (5/9));
    if response.trim() == "F" {
        println!("{}", (temp - 32.0) * (5.0/9.0))
    } else {
        println!("{}", ((temp * 9.0) / 5.0) + 32.0)
    }
}
