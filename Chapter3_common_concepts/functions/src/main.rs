fn main() {
    divisible_by(23);
}

fn divisible_by(x: i32) {
    match x % 2 {
        0 => {
            print!("Yes its even");
        }
        _ => {
            println!("No, its not divisible and its odd");
        }
    }
}
