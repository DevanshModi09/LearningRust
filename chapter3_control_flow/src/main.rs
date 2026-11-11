fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    loop {
        print!("hellow");
        break;
    } //Infinite Loop;
    main2();
    /*
        if number {
            println!("number was there");
        } This does not work in rust unlike , javascript and ruby;
    */
}

fn main2() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
    for element in a {
        println!("the value is: {element}");
    }
}
