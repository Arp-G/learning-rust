POINTS TO REMEMBER:


------------------------------------------------------------ BOROWING ------------------------------------------------------------


* We can have any number of immutable references to a varaible

* We can have at most one mutable refereence to a variable in a perticular scope

* We can't have a immutable as well as a mutable reference to a varaible in teh same scope

* Rust automatically detects that the scopr of a mutable reference has ended when the reference is not longer used later
and thus is allows defining a mutable reference to the varaible since the immutable reference is no longer in scope

```
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
```

* Rust we not allow us to have dangling pointers to varaibles which have gone out of scope, the code wont compile.
A reference can only exists as long the varaible is there in memory, if the varaible goes out of scope and we try to
use the reference to it, rust will detect this at compile time.


------------------------------------------------------------ The Slice type ------------------------------------------------------------

* Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.

* A string literal is nothing but a slice (a pointer) to a binary stored in memory, 
A slice type contains the starting memory location of the slice, and its length 
(This is another reason why a string literal is immutable since its a slice which is a immutable reference)

```
let s = "Hello, world!"; // The type of s here is &str (slice)
```

* A string literal is actual of type slice, eg: let s = "hello  world";

* When we use slices it means we are creating an immutable reference to our varaible (or a reference to a part of the varaible)
  This means we cant create a mutable reference (as we can't have a mutable and immutable reference togther in same scope)

```
  fn main() {
    let mut s = String::from("Hello world !");
    
    let sl = &s[1..2]; // Creates a immutable reference to s
    
    let x = & mut s; // BUG ! Creates a mutable reference to s
    
    println!("{}", sl); // Without this print statement the code would compile since then the immutable reference "sl" would go out of scope since its never used 
}
```

* We also have general slice types like over arrays...

```
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];
```

------------------------------------------------------------ STRUCTS ------------------------------------------------------------

* Defining and copying:

```
// define
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// use, if property and value is same use shorthand like this
User {
  email,
  username,
  active: true,
  sign_in_count: 1,
}

// shorthand for copying ans specifying only properties that have changed
User {
  email,
  username,
  active: true,
  sign_in_count: 1,
}
```

* Tuple Structs

```
fn main() {
    struct Color(i32, i32, i32);
    let black = Color(0, 2, 4);

    let Color(a, b, c) = black; // destructuring a tuple struct

    println!("{}", black.1); //2
    println!("{} {} {}", a, b, c); // 0 2 4
}
```

* printing structs

to print our struct must implement `std::fmt::Display` or `std::fmt::Debug`...

We can do that later by...

```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// some code...

println!("{:?}", rect1); // { width: 30, height: 50 }

println!("{:#?}", rect1); // using {:#?} prints in separate lines
```

* Unit struct, we can define a struct without any properties like:

```
struct QuitMessage;
```

* Struct methods

We can define methods(not fucntions) tied to a struct, it will receive self (instance of the struct) as the first argument.
In the method we can receive self as `&self` or `& mut self` if we need to mutate it. We can also take owner shif of the struct
by simplying receiveing  only sellf instaed of its reference but this is not usually done.

Also, a struct may have multiple impl blocks, its perfectly valid.

Eg:

```
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

* Associated functions, inside impl blocks we can have methods which receive self as the first arg, also we can have a
  associated function, which are like normal fucntion (they don't receive self), often these are used as contructors.
  To calll an associated function we use, struct::func().

  Associated functions let you namespace functionality that is particular to your struct without having an instance available.

  Eg:

```
  impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
  } // Can be called using Rectangle::square(3);
```

------------------------------------------------------------ ENUMS ------------------------------------------------------------

* You can define an enum in rust, and enum can also optioanlly have values associated with it (you need to specify the type of these values) when defining
  the enum.

Example:

```
enum Message {
    Quit,                       // has no data associated with it at all.
    Move { x: i32, y: i32 },    // includes an anonymous struct inside it
    Write(String),              // includes a single String
    ChangeColor(i32, i32, i32), // includes three i32 values
}

// Like Structs you can also have methods on Enum, which receive the enum instance as the first argument
impl Message {
    fn call(&self) {
        match self {
            Message::Write(value) => println!("value: {}", value), // Destructure the value from the enum and print it
            _ => println!("Something else"),
        }
    }
}

let m = Message::Write(String::from("hello")); // Create a new enum of of type `Message::Write` with value "hello"
m.call(); // invoke the call() method on the enum instance
```

* The Option<T> enum in rust is used to avoid problems with null values

Since Option<T> is very usefull its already present in scope and so are its variants: you can use Some and None, you don't need to get it.
You can directly without the Option:: prefix.

```
enum Option<T> { // <T> here is a generic term, that can be anything
    Some(T),
    None,
}

let some_number = Some(5);
let some_string = Some("a string");
let absent_number: Option<i32> = None;
```

If you have a need where a varaible might contain nothing/null at a perticular option you can use the Option<T> enum to wrap it.
Doing so, elimintaes errors which occur due to null values.
Bacause using Option<T> instead of T (where T can be any type), the compiler won’t let us use an Option<T> value as if it were DEFINITELY a valid value. 

Thus, Everywhere that a value has a type that isn’t an Option<T>, you can safely assume that the value isn’t null.

For example, this code won’t compile because it’s trying to add an i8 to an Option<i8>

```
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y; // ERROR ! no implementation for `i8 + std::option::Option<i8>
```

If a varaible might contain None, using Options rust forces us to define what to do in case its null rather than crashing our code..

Eg:

```
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5); // Option::Some(5)
    let six = plus_one(five);
    let none = plus_one(None); // Now adding 1 to null doesn't lead to an error since we have handled that case
```

Rust will ensure that we have handled all possible cases in our match clause, for example if we excluded the `None` case in the above example rust would say...

ERROR!
`ensure that all possible cases are being handled, possibly by adding wildcards or more match arms`

So...

Matches in Rust are exhaustive: we must exhaust every last possibility in order for the code to be valid. Especially in the case of Option<T>, when Rust prevents us from forgetting to explicitly handle the None case, it protects us from assuming that we have a value when we might have null, thus making error due to null impossible.



* if let

If you have a situation in which your program has logic that is too verbose to express using a match, you can use "if let".

"if let" is like syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values.
You can add a else block to handle all other cases for a "if let" statement, here the else blocks works like the "_" wildcard for match statements.

Eg:

```
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
```

Thus, When enum values have data inside them, you can use match or if let to extract and use those values, depending on how many cases you need to handle.
