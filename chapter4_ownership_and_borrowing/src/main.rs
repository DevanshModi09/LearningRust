fn main() {
    let s = String::from("Akshat");
    name(&s);

    let r1 = &s;
    let r2 = &s;

    //Slices
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but s no longer has any content that we
    // could meaningfully use with the value 5, so word is now totally invalid!
}

fn name(name: &String) {
    println!("{name}");
}
