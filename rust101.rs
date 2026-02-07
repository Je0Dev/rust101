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

// same type
    enum IpAddrKind {
        V4,
        V6,
    }

 let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
fn route(ip_kind: IpAddrKind) {}
route(IpAddrKind::V4);
route(IpAddrKind::V6);


    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,//ip type
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,//ip type
        address: String::from("::1"),
    };

    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

        enum IpAddr {
        V4(u8, u8, u8, u8),//0-255,4 nums
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    struct Ipv4Addr {
    // ipv4 addr
}

struct Ipv6Addr {
    // ipv6 addr
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
struct QuitMessage; // unit struct
struct MoveMessage {//normal struct
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

    impl Message {
        fn call(&self) {
            // method body
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();//call body from Message

    //A null is a value that is currently invalid or absent for some reason!

enum Option<T> {//generic->data any type
    None,
    Some(T),
}
    let some_number = Some(5);//num
    let some_char = Some('e');//char

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y;//we get error-cant add diff types bro

enum Coin {
    Penny,//for case 1
    Nickel,//for case 2
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {//it enum list
    match coin {
        Coin::Penny => 1,//case 1
        Coin::Nickel => 5,//case 2
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {//arm:pattern & code
        Coin::Penny => {
            println!("Lucky penny!");
            1//returns val of block 1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

#[derive(Debug)] //inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),//bind
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}
value_in_cents(Coin::Quarter(UsState::Alaska))//coin=Quarter(Alaska)


    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,//x=None-<program stops & returns None
            Some(i) => Some(i + 1),//Some(6)
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
         _ => reroll(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
        fn reroll() {}
    fn move_player(num_spaces: u8) {}

//_ is a special pattern that matches any value 
// and does not bind to that value

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),//if roll other than 3,7 nothing happens now
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
//or even better
    let config_max = Some(3u8);
    if let Some(max) = config_max {//if let works like match
        println!("The maximum is configured to be {max}");//runs for pattern mathching!
    }

        let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}!"),
        _ => count += 1,//else
    }
    let mut count = 0;
    if let Coin::Quarter(state) = coin {//if matchie-matchie 
        println!("State quarter from {state:?}!");
    } else {//if not
        count += 1;
    }


    impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            
        }
    }
}
fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}
fn describe_state_quarter(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state //if matching return state
    } else {
        return None;
    };

    if state.existed_in(1900) {//if found
        Some(format!("{state:?} is pretty old, for America!"))
    } else {//else 
        Some(format!("{state:?} is relatively new."))
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {//if match->outer scope
        return None;//if DONT match->here
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}


//  Module System:
//  Packages: A Cargo feature that lets you build, test, and share crates
// Crates: A tree of modules that produces a library or executable
// Modules and use: Let you control the organization, scope, and privacy of paths
// Paths: A way of naming an item, such as a struct, function, or module

//A crate->smallest amount of code that the Rust compiler considers at a time
//Binary crates-> are programs you can compile to an executable that you can run, such as a command line program or a server.
//Library Crates-> define functionality intended to be shared with multiple projects.

// A package is a bundle of one or more crates that provides a set of functionality. 
// A package contains a Cargo.toml file that describes how to build those crates. 
// Cargo is actually a package that contains the binary crate for the command line tool 
// you’ve been using to build your code. 
// The Cargo package also contains a library crate that the binary crate depends on.

// $ cargo new my-project
//      Created binary (application) `my-project` package
// $ ls my-project
// Cargo.toml
// src
// $ ls my-project/src
// main.rs


// Start from the crate root: 
// When compiling a crate, the compiler first looks in the crate root file 
// (usually src/lib.rs for a library crate and src/main.rs for a binary crate) for code to compile.
// Declaring modules: 
// In the crate root file, you can declare new modules; 
// say you declare a “garden” module with mod garden;. 
// The compiler will look for the module’s code in these places:
//     Inline, within curly brackets that replace the semicolon following mod garden
//     In the file src/garden.rs
//     In the file src/garden/mod.rs

// Declaring submodules: 
// In any file other than the crate root, 
// you can declare submodules. For example, you might declare mod vegetables; 
// in src/garden.rs. 
// The compiler will look for the submodule’s code within the directory named 
// for the parent module in these places:
//     Inline, directly following mod vegetables, within curly brackets instead of the semicolon
//     In the file src/garden/vegetables.rs
//     In the file src/garden/vegetables/mod.rs

// Paths to code in modules: 
// Once a module is part of your crate, you can refer to code in that module 
// from anywhere else in that same crate, as long as the 
// privacy rules allow, using the path to the code. 
// For example, an Asparagus type in the garden vegetables module would be found at 
// crate::garden::vegetables::Asparagus.

// Private vs. public: 
// Code within a module is private from its parent modules by default. 
// To make a module public, declare it with pub mod instead of mod. 
// To make items within a public module public as well, use pub before their declarations.
// The use keyword: 
// Within a scope, the use keyword creates shortcuts to items to reduce repetition of long paths. 
// In any scope that can refer to crate::garden::vegetables::Asparagus, 
// you can create a shortcut with use crate::garden::vegetables::Asparagus;, 
// and from then on you only need to write Asparagus to make use of that type in the scope.

// backyard
// ├── Cargo.lock
// ├── Cargo.toml
// └── src
//     ├── garden
//     │   └── vegetables.rs
//     ├── garden.rs
//     └── main.rs
use crate::garden::vegetables::Asparagus;//src/main.rs

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}
pub mod vegetables;//src/garden.rs-included vegetables.rs
#[derive(Debug)]
pub struct Asparagus {}//src/garden/vegetables.rs

// Modules also allow us to control the privacy of items because code within a 
// module is private by default. 
// Private items are internal implementation details not available for outside use


// cargo new restaurant --lib
mod front_of_house {//src/lib.rs-crate roots
    mod hosting {
        //modules
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        //modules again
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
// crate
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment

//  If module A is contained inside module B, 
//  we say that module A is the child of module B and that module B is the parent of module A


// An absolute path is the full path starting from a crate root; 
// for code from an external crate, the absolute path begins with the crate name, 
// and for code from the current crate, it starts with the literal crate

// A relative path starts from the current module and uses self, 
// super, or an identifier in the current module.

// one or more identifiers separated by double colons (::)

mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path- root(start)->front->hosting->waitlist
    crate::front_of_house::hosting::add_to_waitlist();
 ///front_of_house/hosting/add_to_waitlist-> run add_to_waitlist
    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

//  In Rust, all items (functions, methods, structs, enums, modules, and constants) 
//  are private to parent modules by default. 
//  If you want to make an item like a function or struct private, you put it in a module

//compiling & running above->issue

//child modules can see the context in which they’re defined(ancestor modules)
//office managers can see and do everything in the restaurant they operate(back office?we know dude)

// Rust does give you the option to expose inner parts of child modules’ 
// code to outer ancestor modules by using the pub keyword to make an item public

mod front_of_house {
    pub mod hosting {//public
        fn add_to_waitlist() {} //can reference not access
    }
}//contents of hosting are still private-> doesn’t make its contents public =( pitty

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }//we good now,right?access parent module of hosting->access parent module add_to_watchlist
}

// The module tree should be defined in src/lib.rs. Then, any public items 
// can be used in the binary crate by starting paths with the name of the package. 
// The binary crate becomes a user of the library crate just like a completely external 
// crate would use the library crate: It can only use the public API. 
// This helps you design a good API; 
// not only are you the author, but you’re also a client!

//src/lib.rs
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();//..->goto parent dir(ref parent module here)
    }//access wherever it is

    fn cook_order() {}
}

//src/lib.rs
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast.
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Change our mind about what bread we'd like.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal.
    // meal.seasonal_fruit = String::from("blueberries");
}
//src/lib.rs
mod back_of_house {
    pub enum Appetizer {//public
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

//src/lib.rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;//into scope-symbolic link filesys

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

mod customer {//use only creates the shortcut for the particular scope in which the use occurs;
    //so it doesnt compile here(out of scope)
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}

// Specifying the parent module when calling the function makes it clear 
// that the function isn’t 
// locally defined while still minimizing repetition of the full path
use crate::front_of_house::hosting::add_to_waitlist;

//in other words we dont know where add_to_watchlist is defined
pub fn eat_at_restaurant() {
    add_to_waitlist();
}

//scope binary crate src/main.rs
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

//src/lib.rs
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}

//better way
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}

//re-exporting because we’re bringing an item into scope but also making 
// that item available for others to bring into their scope(we good peaople i mean)

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

//write our code with one structure but expose a different structure.
//customers dont think "front of house” and “back of house".
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}


//Cargo.toml
rand = "0.8.5"

use rand::Rng;//into scope
use std::cmp::Ordering;
use std::io;
//or even better
use std::{cmp::Ordering, io};
use std::io;//general
use std::io::Write;//specific
//or even better
use std::io::{self, Write};//boom!
use std::collections::*;//all public items defined in a path into scope->there it is

//if the dependency adds a definition with the same name 
// as a definition of yours in the same scope->problemio 
fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}

//src/lib.rs
mod front_of_house;

pub use crate::front_of_house::hosting;//access

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
//src/front_of_house.rs
pub mod hosting {
    pub fn add_to_waitlist() {}//public
}

//src/front_of_house.rs
pub mod hosting;
//src/front_of_house/hostin.rs
pub fn add_to_waitlist() {}


// src/front_of_house.rs (what we covered)
// src/front_of_house/mod.rs (older style, still supported path)

// src/front_of_house/hosting.rs (what we covered)
// src/front_of_house/hosting/mod.rs (older style, still supported path)



//collections can contain multiple values
//collections point to is stored on the heap, which means the amount 
// of data does not need to be known at compile time and can grow or shrink as the program runs

//A vector allows you to store a variable number of values next to each other.
//A string is a collection of characters. We’ve mentioned the String type previously, 
// but in this chapter, we’ll talk about it in depth.
//A hash map allows you to associate a value with a specific key. 
// It’s a particular implementation of the more general data structure called a map.

let v: Vec<i32> = Vec::new();

let v = vec![1, 2, 3];

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

        let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];//3
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100];//problem
    let does_not_exist = v.get(100);//None 


     let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    //issue i guess =( but why though?
    println!("The first element is: {first}");


    // vectors put the values next to each other in memory, 
    // adding a new element onto the end of the vector might 
    // require allocating new memory and copying the old elements to the new space
    //,if there isn’t enough room to put all the elements next to each other 
    // where the vector is currently stored. In that case, 
    // the reference to the first element would be pointing to deallocated memory

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;//dereference->get val i
    //     The reference to the vector that the 
    //     for loop holds prevents simultaneous modification of the whole vector
    //      this seems weird i guess...
     }

        enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
//what types will be in the vector at compile time so that it 
// knows exactly how much memory on the heap will be needed to store each element

    {
        let v = vec![1, 2, 3, 4];
        // do stuff with v
    } // <- v goes out of scope and is freed here/integers it holds will be cleaned up
    //The borrow checker ensures that any references to contents of a vector are only used while the vector itself is valid.


    //String or the string slice &str types(slices)->UTF-8 encoded

    // wrapper around a vector of bytes with some extra guarantees, restrictions, and capabilities.
        let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // The method also works on a literal directly:
    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    //multilingua stuff
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

        let mut s = String::from("foo");
    s.push_str("bar");//append str slice

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);//append
    println!("s2 is {s2}");//foobar!

    let mut s = String::from("lo");
    s.push('l');//lol

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    //takes ownership of s1, appends a copy of the contents of s2, and then returns ownership of the result
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used!!!

    // + fn add(self, s: &str) -> String { ...
//We can only add a string slice to a String; we can’t add two String values together
// &s2 in the call to add is that the compiler can coerce the &String argument into a &str!
// we call the add method, Rust uses a deref coercion, which here turns &s2 into &s2[..](dafuq y mean)
//add doest take ownership of the s parameter, s2 will still be a valid String after this operation!

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    //references so that this call doesn’t take ownership of any of its parameters.
    let s = s1 + "-" + &s2 + "-" + &s3;//String with the contents


        let s1 = String::from("hi");
    let h = s1[0];//problem-indexing not possible

        let hello = String::from("Hola");//4
        let hello = "Здравствуйте";
        let answer = &hello[0];//1st byte of 3->208,2nd byte->151
        //If &"hi"[0] were valid code that returned the byte value, it would return 104, not h!
    
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]//18 bytes
//Rust would have to walk through the contents from the beginning to 
//the index to determine how many valid characters there were
['न', 'म', 'स', '्', 'त', 'े']// fourth and sixth are not letters-diacritics(dont make sense on their own, so why bother)

let hello = "Здравствуйте";

let s = &hello[0..4];//first 4 bytes,s=Зд 
&hello[0..1]//problem

for c in "Зд".chars() {
    println!("{c}");
}//or
for b in "Зд".bytes() {
    println!("{b}");
}//gives: 208 151 208 180

//more of the complexity of strings;prevents you from having to handle errors involving non-ASCII characters


//HashMap<K, V> stores a mapping of keys of type K to values of type V using a 
// hashing function, which determines how it places these keys and values into memory

// in a game, you could keep track of each team’s score 
// in a hash map in which each key is a team’s name and the values are each team’s score
//you have the name?you have the score!

    use std::collections::HashMap;
//All of the keys must have the same type, and all of the values must have the same type
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    //get->ret Option<&V>
    let score = scores.get(&team_name).copied().unwrap_or(0);

    //Yellow: 50 Blue: 10
     for (key, value) in &scores {
        println!("{key}: {value}");
    }

     let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);//moved into hash map
    // field_name and field_value are invalid at this point!

    //  each unique key can only have one value associated with it at a time(1 identifier lets say)
    //  not vice versa(n values for 1 key):
    // both the Blue team and the Yellow team could have the value 10 stored in the scores hash map

//change the data in a hash map,?
//decide how to handle the case when a key already has a value assigned


    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);//overwriten

    println!("{scores:?}");

    //Hash maps have a special API for this called entry that takes the key you want to check as a parameter
 let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);//it doesnst->insert!
    scores.entry(String::from("Blue")).or_insert(50);//it does->so...

    println!("{scores:?}");




        use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");


    //recoverable errors,report the problem to user and retry the operation. 
    // Unrecoverable errors are always symptoms of bugs-immediate stop


    //Result<T,E>;recoverable errors and panic! macro that stops execution

    //By default, when a panic occurs, the program starts unwinding, which means Rust walks back up the stack and cleans up the data from each function it encounters


enum Result<T, E> {
    Ok(T),
    Err(E),
}

use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
}

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };
}

//or alternative

fn main() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}


//panick bro
  let greeting_file = File::open("hello.txt").unwrap();

 let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");



use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}


fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}


fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

//shortest yet
fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}


use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}

use std::net::IpAddr;

    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");


loop {

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100.");
            continue;
        }

        match guess.cmp(&secret_number) {
            // --snip--
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}


//generics: abstract stand-ins for concrete types or other properties


//lifetimes: a variety of generics that give the compiler information about how references relate to each other


fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");
}

//best  way to remove duplicating code(dont repeat-when not needed)
fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {result}");
}




fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {result}");
}


struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
       let wont_work = Point { x: 5, y: 4.0 };
}

//for all instances to be allowed be generic
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}


enum Option<T> {
    Some(T),
    None,
}
enum Result<T, E> {
    Ok(T),
    Err(E),
}


struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
}


impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}



struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

//Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled


//compiler uses different names than what we’re using here
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}

//A trait defines the functionality a particular type has and can share with other types. 

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

use aggregator::{SocialPost, Summary};

fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarize());
}
//1 new post: horse_ebooks: of course, as you probably already know, people.


pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
//we can call it
    println!("New article available! {}", article.summarize());


    pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarize());
// 1 new post: (Read more from @horse_ebooks...).

//traits as params
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
//trait bound
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify(item1: &impl Summary, item2: &impl Summary) {}//diff types
pub fn notify<T: Summary>(item1: &T, item2: &T) {}//same type



pub fn notify(item: &(impl Summary + Display)) {}
pub fn notify<T: Summary + Display>(item: &T) {}//valid as well on generic types



fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{}


fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    }
}



use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

//blanket implementation
impl<T: Display> ToString for T {}
let s = 3.to_string();//implements Display


//lifetimes
fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {r}");   //   |       |
                          // --+       |
}                         // ----------+


fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}//problem,sadly


&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
//The returned reference will be valid as long as both of the parameters are valid
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}


fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }
}
//The longest string is long string is long


fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}


fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {}
fn first_word<'a>(s: &'a str) -> &'a str {}


impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

let s: &'static str = "I have a static lifetime.";


use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}

//writing tests-important

//cargo new adder --lib,cd adder
//lib.rs
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

     #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}//cargo test->in proj


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }
  #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

}


pub fn add_two(a: u64) -> u64 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }
}

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{result}`"
        );
    }


pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]//check panics
    fn greater_than_100() {
        Guess::new(200);
    }
}


// --snip--

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {value}."
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {value}."
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}


//cargo test -- --test-threads=1 specific for binary
//cargo test -- --show-output successful

pub fn add_two(a: u64) -> u64 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_three_and_two() {
        let result = add_two(3);
        assert_eq!(result, 5);
    }

    #[test]//cargo test one_hundred
    fn one_hundred() {
        let result = add_two(100);
        assert_eq!(result, 102);
    }
}
//cargo test add multiple-regex


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}// cargo test -- --include-ignored all types


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}


pub fn add_two(a: u64) -> u64 {
    internal_adder(a, 2)
}

fn internal_adder(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        let result = internal_adder(2, 2);
        assert_eq!(result, 4);
    }
}


adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    |── common
    │   └── mod.rs
    └── integration_test.rs
    
        mod common;
        use adder::add_two;
        #[test]
    fn it_adds_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }//cargo test --test integration_test all of them







// Closures, a function-like construct you can store in a variable
// Iterators, a way of processing a series of elements


#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };


fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;


    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    let n = example_closure(5);



    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");


    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {list:?}");


     let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();


impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}



#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);

    println!("{list:#?}");//width/height: 3,5 7,12 10,1


       let mut sort_operations = vec![];
    let value = String::from("closure called");

    list.sort_by_key(|r| {
        sort_operations.push(value);
        r.width
    });
    println!("{list:#?}");//problem

       let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{list:#?}, sorted in {num_sort_operations} operations");
}


    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();
  for val in v1_iter {
        println!("Got: {val}");
    }

pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // methods with default implementations elided
}

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    //consume iterator
     #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }
    //produce iterators
     let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);


    #[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}

//release profiles are predefined, 
// customizable profiles with different configurations 
// that allow for more control over various options for compiling code

//cargo build (--release)

[profile.dev]
opt-level = 0//default
//relase?make it 3

// Cargo will apply more optimizations than the default, but not as many as in a release build.


//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
//cargo doc --open 
// builds  HTML for your current crate’s documentation 
// (as well as the documentation for all of your crate’s dependencies) 
// and open the result in a web browser



//! # Art
//!
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        // --snip--
    }
}


// use art::kinds::PrimaryColor;
// use art::utils::mix;

use art::PrimaryColor;
use art::mix;


fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}

//cargo login->api key(publish crates)

//metadata(publish phase)
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2024"
description = "A fun game."
license = "MIT OR Apache-2.0"
//cargo publish

//Yanking a version prevents new projects from depending 
// on that version while allowing all existing projects 
// that depend on it to continue. 
// yank means that all projects with a Cargo.lock will not break, 
// and any future Cargo.lock files 
// generated will not use the yanked version.

//cargo yank --vers 1.0.1 (--undo)


//A workspace is a set of packages that share the same Cargo.lock and output directory

//cargo new adder(bin crate)
[workspace]
resolver = "3"
members = ["adder"]//latest version of algorithm

//cargo new add_one --lib
members = ["adder", "add_one"]

├── Cargo.lock
├── Cargo.toml
├── add_one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target

//add_one/src/lib.rs
pub fn add_one(x: i32) -> i32 {
    x + 1
}

//adder/Cargo.toml
[dependencies]
add_one = { path = "../add_one" }

//adder/src/main.rs
fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", 
    add_one::add_one(num));
}

//add_one/Cargo.toml
[dependencies]
rand = "0.8.5"//external package-> use rand;


//add_one/src/lib.rs
pub fn add_one(x: i32) -> i32 {
    x + 1
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}
// test for 1 crate in a workspace from the top-level dir
//cargo test -p add_one



//cargo install ripgrep install binaries

//Smart pointers, are data structures that act like a pointer but also have additional metadata and capabilities

//heap
fn main() {
    let b = Box::new(5);
    println!("b = {b}");
}

enum List {
    Cons(i32, List),
    Nil,
}
use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}//infinite size

//better
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = 
    Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

fn main() {
    let x = 5;
    //y to be an instance of a box pointing to a copied value of x 
    // rather than a reference pointing to the value of x!
    let y = Box::new(x);
    //let y = &x; problem

    assert_eq!(5, x);
    assert_eq!(5, *y);//*(y.deref()) 
}


struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}//  let y = MyBox::new(x); error! 

use std::ops::Deref;//standard

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}//or even better

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    //    hello(&(*m)[..]); deref MyBox<Str> into Str

}
//Deref Coercion->

// From &T to &U when T: Deref<Target=U
// From &mut T to &mut U when T: DerefMut<Target=U>
// From &mut T to &U when T: Deref<Target=U>


struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping 
        CustomSmartPointer with data `{}`!", self.data);
    }//drop trait
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created");

//     
//      let c = CustomSmartPointer {
//         data: String::from("some data"),
//     };
//     println!("CustomSmartPointer created");
//     c.drop(); problem!
//     println!("CustomSmartPointer dropped before the end of main");
// }

  let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created");
    drop(c);//no problem
    println!("CustomSmartPointer dropped before the end of main");




    enum List {
    Cons(i32, Box<List>),
    Nil,
}


use crate::List::{Cons, Nil};

fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));
}//b->3->5->10->Nil c->4->5... a->5...


enum List {
    // Each Cons variant will now hold a value and an Rc<T> pointing to a List
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}

 let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
//     count after creating a = 1
// count after creating b = 2
// count after creating c = 3
// count after c goes out of scope = 2




pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: vec![],
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.len(), 1);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        // --snip--
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            let mut one_borrow = self.sent_messages.borrow_mut();
            let mut two_borrow = self.sent_messages.borrow_mut();

            one_borrow.push(String::from(message));
            two_borrow.push(String::from(message));
        }
    }

//multiple owners of mutable data
    #[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}