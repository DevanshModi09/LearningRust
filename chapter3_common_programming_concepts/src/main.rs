fn main() {
    let x = 5;
    let mut y = 23;

    //Tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    let five_hundred = tup.0;
    let six_point_four = tup.1;

    //Array  (all the elements will be of the same type);
    let a = [1, 24, 53, 54, 6, 35];
    // [i32:5];
    // [Type of array:No of elements];
    let b = [3; 5];

    //Functions (In rust we can declare functions inside functions)
}

fn another_function(x: i32) -> i32 {
    println!("The value of x is : {x}");
    //Statements vs Expressions
    //Statements do not return anything (they usually have a ; at the end);
    let y = { x + 1 };
    x //This is an Expression (automatically returns if at last of the funciton);
}

fn five() -> i32 {
    5
}



fn plus_one(x: i32) -> i32 {
    x + 1;
    //This will give an error because we are not returning anything;
    //But the function signature expects to return a i32;
};

