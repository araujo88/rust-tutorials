fn main() {
    /////////////////////// constants ///////////////////////

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let mut x = 3;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!("{THREE_HOURS_IN_SECONDS}");

    /////////////////////// shadowing ///////////////////////

    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    let spaces = "   "; 
    let spaces = spaces.len();

    println!("{spaces}");

    /////////////////////// data types ///////////////////////

    let guess: u32 = "42".parse().expect("Not a number!"); // must include type annotation!!
    // let guess = "42".parse().expect("Not a number!"); // will throw error

    println!("{guess}");

    let test: u32 = 1_000;

    println!("{test}");

    let z: f64 = 3.1415; // f32

    println!("{z}");

    let emoji = '🗿'; // char type

    println!("{}", emoji);

    // Tuples

    let tup: (i64, f64, i64) = (500, 6.4, 1);

    let (_a, b, _c) = tup;

    println!("The value of y is: {b}");

    let tup2: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = tup2.0;

    let six_point_four = tup2.1;

    let one = tup2.2;

    println!("{}", five_hundred);
    println!("{}", six_point_four);
    println!("{}", one);

    // arrays

    let array1: [i32; 5] = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let array2 = [3; 5];

    println!("{}", months[0]);

    println!("{:?}", months); // prints all elements of array

    println!("{}", array1[5]);
}