
// constants
const MAX_POINTS: u32 = 100_000;

// important to log user
// error : the trait `std::fmt::Debug` is not implemented for `User`
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn format(&self) -> String {
        let b = format!("{}, {}", self.username, self.email);
        b
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

enum WebEvent {
    // An `enum` may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum`.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

// entry point
fn main() {
    let a = 10; // Types can be inferred by the compiler.
    let b: i32 = 20; // ..or declared by the programmer when creating variables

    let c = add(a, b); // calling a function
    println!("a + b = {}", c); // print console message (stdout) or console.log

    println!("add one to 3  = {}", add_one(3)); // add one to 3  = 4

    let foo = 5; // immutable
    println!("foo : {}", foo); // foo : 5

    // error
    // foo = 3;

    let mut bar = 15; // mutable
    println!("bar : {}", bar); // bar : 15

    bar = 25;
    println!("bar : {}", bar); // bar : 25

    let x = 5;

    if x == 5 {
        println!("x is five!");
    } else if x == 6 {
        println!("x is six!");
    } else {
        println!("x is not five or six :(");
    }

    let number = match x {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        _ => "something else",
    };
    println!("number : {}", number);

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!( "user1 : {:#?}", user1);

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    println!( "user2 : {:#?}", user2);
    println!( "user2 format : {:#?}", user2.format());

    // destructuring
    let User {
        email,
        username,
        sign_in_count,
        active
    } = user1;
    println!( "email : {:#?}", email);
    println!( "username : {:#?}", username);
    println!( "sign_in_count : {:#?}", sign_in_count);
    println!( "active : {:#?}", active);

    // string interpolation
    format!("Hello");                 // => "Hello"
    format!("Hello, {}!", "world");   // => "Hello, world!"
    format!("The number is {}", 1);   // => "The number is 1"
    format!("{:?}", (3, 4));          // => "(3, 4)"
    format!("{value}", value=4);      // => "4"
    format!("{} {}", 1, 2);           // => "1 2"
    format!("{:04}", 42);             // => "0042" with leading zeros

    let a = [1, 2, 3]; // a: [i32; 3]
    let mut m = [1, 2, 3]; // mut m: [i32; 3]

    let v = vec![1, 2, 3, 4, 5]; // v: Vec<i32>
    println!( "v : {:#?}", v);
    println!("The third element of v is {}", v[2]);

    // enum
    let pressed = WebEvent::KeyPress('x');
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    // iterators
    let numbers_iterator = [0,2,3,4,5].iter();

    let sum = numbers_iterator
        .fold(0, |total, next| total + next);
    println!("sum {}", sum);

    let squared: Vec<i32> = [0,2,3,4,5]
        .iter()
        .map(|&x| x * x)
        .collect();
    println!("squared : {:#?}", squared);

    let vector_result = (1..)            // Infinite range of integers
        .filter(|x| x % 2 != 0)   // Collect odd numbers
        .take(5)                  // Only take five numbers
        .map(|x| x * x)           // Square each number
        .collect::<Vec<usize>>(); // Return as a new Vec<usize>
    println!("{:?}", vector_result);     // Print result


    // string
    let sentence = "This is a sentence in Rust.";
    let words: Vec<&str> = sentence
        .split_whitespace()
        .collect();
    let words_containing_i: Vec<&str> = words
        .into_iter()
        .filter(|word| word.contains("i"))
        .collect();
    println!("{:?}", words_containing_i);
}

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 3;
/// let answer = add_one(arg);
///
/// println!("add one to 3  = {}", answer);
/// ```
fn add_one(x: i32) -> i32 {
    x + 1
}

// Types are required when defining functions
fn add(i: i32, j: i32) -> i32 {
    // Functions return the last expression’s result,
    // meaning return is not required (but be careful as adding
    // a semi-colon to this line would change the semantics
    // to return () rather than i32)
    i + j
}
