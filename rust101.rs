//$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
//$ rustc --version, echo $PATH
// rustup update,rustup self uninstall
//rustc main.rs,./main

//cargo --version:Rust’s build system and package manager
//cargo new <proj>,cd <proj>,cargo build,./target/debug/<proj>,cargo run/check

use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}


let x = 5;//  let mut x = 5; CAN CHANGE
println!("The value of x is: {x}");
x = 6;
println!("The value of x is: {x}");//ERROR

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

//global
  let x = 5;
    let x = x + 1;

    {//scope starts
        let x = x * 2;//inner
        println!("The value of x in the inner scope is: {x}");
    }//scope finishes

    //global
    println!("The value of x is: {x}");


    let spaces = "   ";
    let spaces = spaces.len();
// let mut spaces = " "; ERROR->CANT mutate variables type


let guess: u32 = "42".parse().expect("Not a number!");


// 8-bit	i8	u8
// 16-bit	i16	u16
// 32-bit	i32	u32
// 64-bit	i64	u64
// 128-bit	i128	u128
// Architecture-dependent	isize	usize

// Number literals	Example
// Decimal	98_222
// Hex	0xff
// Octal	0o77
// Binary	0b1111_0000
// Byte (u8 only)	b'A'

 let x = 2.0; // f64 default!

    let y: f32 = 3.0; // f32

     // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

 let t = true;

let f: bool = false; // explicit type annotation
 let c = 'z';
    let z: char = 'ℤ'; //  explicit type annotation again
    let heart_eyed_cat = '😻';


let tup: (i32, f64, u8) = (500, 6.4, 1);//fixed!
 let (x, y, z) = tup;
  println!("The value of y is: {y}");//6.4

let x: (i32, f64, u8) = (500, 6.4, 1);
   
     let five_hundred = x.0;//500

    let six_point_four = x.1;//6.4

    let one = x.2;//1

let a = [1, 2, 3, 4, 5];//fixed!
let first = a[0];//1
    let second = a[1];//2
    let third=a[2];//3
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
let a: [i32; 5] = [1, 2, 3, 4, 5];//5=# el
let a = [3; 5];//a = [3, 3, 3, 3, 3]; 

use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();//input:10

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    //index out of bounds!!!
    println!("The value of the element at index {index} is: {element}");
}


fn main() {
    //call
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");//5
}

fn main() {
    //call
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

 let x = (let y = 6);//can’t assign a let statement to another variable!

  let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");//4

fn five() -> i32 {
    5
}

fn main() {
    //call
    let x = five();//let x = 5;

    println!("The value of x is: {x}");//5
}

fn main() {
    let x = plus_one(5);//6
    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

 let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {//goes here
        println!("condition was false");
    }

 let number = 3;

    if number {//error->if number != 0 right!
        println!("number was three");
    }

let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };//num=true->5

    println!("The value of number is: {number}")
    let number = if condition { 5 } else { "six" };//issue-incompatible types

     loop {
        println!("again!");//infinite
    }


     let mut counter = 0;
    let result = loop {
        counter += 1;//incr by 1

        if counter == 10 {//incr=10
            break counter * 2;//10*2 & exit!(final val)
        }
    };

    println!("The result is {result}");//20

fn main() {
    let mut count = 0;//init
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;//exit inner
            }
            if count == 2 {
                break 'counting_up;//exit inner & outer!
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

 let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;//3,2,1
    }

    println!("LIFTOFF!!!");


    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {//seach arr
        println!("the value is: {}", a[index]);

        index += 1;
    }
     for element in a {//same way
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {//cool way i guess
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    // Ownership is a set of rules that govern 
    // how a Rust program manages memory. 

    //heap: When you enter, you state the number 
    // of people in your group, and the host finds 
    // an empty table that fits everyone and leads you there. 
    // If someone in your group comes late, 
    // they can ask where you’ve been seated to find you.


    // It’s most efficient to get all the
    //  orders at one table before moving 
    // on to the next table. Taking an order from table A, 
    // then an order from table B, then one from A again, 
    // and then one from 
    // B again would be a much slower process

    //stack:When you add more plates, 
    // you put them on top of the pile, 
    // and when you need a plate, you take one off the top. 
    // Adding or removing plates 
    // from the middle or bottom wouldn’t work as well

    //Each value in Rust has an owner.
    //There can only be one owner at a time
    //When the owner goes out of scope, the value will be dropped

     {// s not declared-not valid
        let s = "hello";   // s init-valid
    }//s dropped-not valid

     let mut s = String::from("hello");

    s.push_str(", world!"); //appnes to String

    println!("{s}");


      let s1 = String::from("hello");
    let s2 = s1;//copy ptr,len,cap of stack

    let s1 = String::from("hello");
    let s2 = s1;//s1 not valid

    println!("{s1}, world!");//problem

    let mut s = String::from("hello");//can change
    s = String::from("ahoy");//nothing referring to original value on heap!

    println!("{s}, world!");//ahoy,world it is then

    let s1 = String::from("hello");
    let s2 = s1.clone();//heap data copied

    println!("s1 = {s1}, s2 = {s2}");

    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");

    // types implement the Copy trait:
    // All the integer types, such as u32.
    // The Boolean type, bool, with values true and false.
    // All the floating-point types, such as f64.
    // The character type, char.
    // Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s); // s's value moves into the function...
    // ... and so is no longer valid here

    let x = 5;                // x comes into scope

    makes_copy(x);  // Because i32 implements the Copy trait,
    // x does NOT move into the function,
    // so it's okay to use x afterward!

} // Here, x goes out of scope, then s. However, because s's value was moved,
  // nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.


fn main() {
    let s1 = gives_ownership();        // gives_ownership moves its return
                                       // value into s1

    let s2 = String::from("hello");    // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
    // takes_and_gives_back, which also
    // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {       // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string       // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}


fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}


//A reference is like a pointer in that it’s 
// an address we can follow to access the data stored at that 
// address; that data is owned by some other variable. 
// Unlike a pointer, a reference is guaranteed to point 
// to a VALID value of 
// a particular type for the life of that reference

//s->s1->h
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because s does not have ownership of what
  // it refers to, the String is not dropped.


  //Just as variables are immutable by default, so are references. 
  // We’re not allowed to modify something we have 
  // a reference to(what we are borrowing-no stealing that means!!!)
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}


    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, 
    // so we can make a new reference with no problems.

    let r2 = &mut s;//like here for example


    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{r1}, {r2}, and {r3}");


    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.

    let r3 = &mut s; // no problem
    println!("{r3}");


 fn main() {
    let reference_to_nothing = dangle();
}
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope and is dropped, 
// so its memory goes away.
  // Danger!

//Ownership is moved out, and nothing is deallocated.
  fn no_dangle() -> String {
    let s = String::from("hello");

    s
  }


  //Slices let you reference a contiguous sequence of elements in a collection. 
  // A slice is a kind of reference, 
  // so it does not have ownership



  fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();//arr bytes

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    let hello = &s[0..5];//s->hello
    let world = &s[6..11];//w->world
 let slice = &s[0..2];
let slice = &s[..2];//equals


let len = s.len();
let slice = &s[3..len];
let slice = &s[3..];//equals as well


let len = s.len();

let slice = &s[0..len];
let slice = &s[..];//equals as third

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but s no longer has any content that we
    // could meaningfully use with the value 5, so word is now totally invalid!
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];//bit better than last time
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!
    //if we have an immutable reference to something, 
    // we cannot also take a mutable referenc

    println!("the first word is: {word}");
}


fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole.
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s.
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or
    // whole.
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}


let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];//&[i32]
assert_eq!(slice, &[2, 3]);//[1,2,3]==[2,3]



//hold multiple related values
//the pieces of a struct can be different types
struct User {
    //define fields
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    //access struct-instance
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    //change base email
    user1.email = String::from("anotheremail@example.com");


}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,//no need to write default
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    // or bit simpler

     let user2 = User {
        email: String::from("another@example.com"),
        ..user1
        //remaining fields not explicitly set 
        // should have the same value 
        // as the fields in the given instance
    };
}


struct Color(i32, i32, i32);//tuple 
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

struct AlwaysEqual;//unit-like

fn main() {
    let subject = AlwaysEqual;
}


fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

//or another version with tuples-pick your poisson
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}


//we learned structs so use this bro of course
#[derive(Debug)]//needed!
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    //debugging
    println!("rect1 is {rect1}");//wrong =( 
     println!("rect1 is {rect1:?}");//correct =)
    
    
     println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}



fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), //stderr instead of stdout,returns 
        // ownership of the expression’s value
        height: 50,
    };

    dbg!(&rect1);
    //reference to rect1 in the next call so dbg! doesnt take ownership
}




#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {//associated with Rectangle type
    fn area(&self) -> u32 {
        //object->something() == (*object).something()
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    //to compare instances
     fn can_hold(&self, other: &Rectangle) -> bool {
        //current object compared with other each time(2 params!)
        self.width > other.width && self.height > other.height
    }
     fn square(size: u32) -> Self {
        Self { //current
            width: size,
            height: size,
        }//alias for Rectangular type
    }
}

//Each struct is allowed to have multiple impl blocks!!!
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
//useful for generic types and traits
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// When you call a method with object.something(), 
// Rust automatically adds in &, &mut, or * 
// so that object matches the signature of the method


//p1.distance(&p2) == (&p1).distance(&p2)
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    //add more Rectangular instances
     let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };


    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

     if rect1.width() {
        println!("The rectangle has a nonzero width; 
        it is {}", rect1.width);
    }

    //true/false-boolean
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}


