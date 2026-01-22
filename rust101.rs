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