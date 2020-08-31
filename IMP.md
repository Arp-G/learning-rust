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
  associated function, which are like normal fucntion (they don't receive self), often these are used as contructors, which give a instance of that struct.
  To call an associated function we use, struct::func().

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


----------------------------------------------------------- PACKAGES, CRATES & MODULES -----------------------------------------------------------


* A package is one or more crates that provide a set of functionality. A package contains a Cargo.toml file that describes how to build those   crates.

* There can be 2 types of crates, library crates and binary crates. You can have max one library crate but any number of binary crates in a package

A binary crate is an executable project that has a main() method. A library crate is a group of components that can be reused in other projects. 
Unlike a binary crate, a library crate does not have an entry point (main() method).

* By deafult when we create a new package using `cargo new package-name`, Cargo follows a convention that src/main.rs is the crate root of a binary crate with the same name as the package. 

* Likewise, Cargo knows that if the package directory contains src/lib.rs, the package contains a library crate with the same name as the package, and src/lib.rs is its crate root.

* We can create a new library named restaurant by running cargo new --lib restaurant, this will create a new library which has its own `src/lib.rs`
Now we might have a folder structure like:

├───resturant
│   └───src
|       |__ lib.rs
├───src
|   |__ main.rs
|
|__ Cargo.toml

* Modules let us organize code within a crate into groups for readability and easy reuse.

We define a module by starting with the mod, modules can contain other modules or functions, structs, enums, constants, traits, etc.

Eg:

```
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
    }
}

```

*  src/main.rs and src/lib.rs are called crate roots. The reason for their name is that the contents of either of these two files form a module named crate at the root of the crate’s module structure, known as the module tree. The entire module tree is rooted under the implicit module named crate

crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     └── serving
         ├── take_order
         ├── serve_order


* By default the code we write is implicitley inside the crate root module, everthing defined inside a module is private by default unless explicitly made public
using the `pub` keyword. Modules can be accessed via a abosulte (starting from the crate root module) or relative path.

You can access a module if its public or if its a sibling of the current module.

```
// `front_of_house` is not public
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    // The front_of_house module isn’t public, but because the eat_at_restaurant function is defined in the same module(the implicit crate module) as front_of_house 
    // (that is, eat_at_restaurant and front_of_house are siblings), we can refer to front_of_house from eat_at_restaurant.
    front_of_house::hosting::add_to_waitlist();
}
```

* We can construct relative paths that begin in the parent module by using super at the start of the path.

```
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); // This is same as `crate::serve_order()` since the parent module of `back_of_house` is the implicit `crate` root module
    }

    fn cook_order() {}
}
```

* a struct inside a module is by default private and can be made public using the `pub` keyword.

HOWEVER, even if the struct is made public the properties of the struct remain private and have to be explicitly made public.

```
mod back_of_house {

    pub struct Breakfast {
        pub toast: String,      // This property is public
        seasonal_fruit: String, // This property is private
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast { // A public associated function which acts like a constructor to make "Breakfast" instances.
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

let mut meal = back_of_house::Breakfast::summer("Rye");

meal.toast = String::from("Wheat");                 // OK !

meal.seasonal_fruit = String::from("blueberries"); // ERROR ! cannot access private field `seasonal_fruit` of the `Breakfast` struct

```

* Unlike structs where we need to mae the invidual properties of the struct public for enums 
 if we declare and enum as public it means all of its variants are then public.

 ```
 mod back_of_house {
    pub enum Appetizer {
        Soup,  // Here we don't need to explicitly declare `Soup` and `Salad` as public. 
        Salad, // Since `Appetizer` is public we can directly use `back_of_house::Appetizer::Soup`
    }
}
```


* The `use` keyword is used to bring some other PUBLIC code into current scope, it can be thoughh of as an alias
and helps us to avoid writing completed path names again and again.

To import items relative to the current and parent modules, use the `self::` and `super::` prefixes, respectively.

The rules of visibility also applies qually when brining items into scope via "use"

```

/*
use crate::front_of_house::hosting;
use self::front_of_house::hosting;  // From the current module bring "front_of_house::hosting" into scope
use std::collections::HashMap;


use std::fmt::Result;
use std::io::Result as IoResult; // import as to avoid name collisions

/*
Re-exporting Names with "pub use"

When we bring a name into scope with the use keyword, the name available in the new scope is private. 
To enable the code that calls our code to refer to that name as if it had been defined in that code’s scope, we can combine pub and use. 
*/

pub use crate::front_of_house::hosting;


use std::{cmp::Ordering, io}; // // Bring multiple items from same std module

/*
Intead of this....

use std::io;
use std::io::Write;

We can write...

*/
use std::io::{self, Write};


// To bring all public items defined in a path into scope, we can specify that path followed by *
use std::collections::*;

```


* Keeping modules in separarte files...

Using a semicolon after `mod filename;` rather than using a block tells Rust to load the contents of the 
file.

Eg: 

To include the code from hello.rs in your main.rs, use mod hello;. It gets expanded to the code that is in hello.rs.

use is just for alias, while mod pulls in the file. You would use use, for example, to be able to call the print_hello function without having to prefix with the namespace


----------------------------------------------------------- VECTORS, STRINGS and HASHMAPS -----------------------------------------------------------

* A vector stores homogenous values contigously in memeory, its stored in the heap so elements can be added or removed from it at runtime.
Vectors can only store values of the same type.

```
let v: Vec<i32> = Vec::new(); // creating a vector using Vec::new() requires a type annotaion

let v = vec![1, 2, 3]; // Here we don't need type annotation when using the vec! macro

let v = vec![]; // however this won't work, type annotations needed for `std::vec::Vec<T>`

------------------------------------------------------
// To add elements to a vector it must be mutable...
let mut v = Vec::new();
v.push(5);
v.push(6);
v.pop();

------------------------------------------------------
// Reading from a Vector, there are 2 ways...

let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2]; // Get the 3rd element, this works but the code will crash if we try to access a invalid or out of bound index

// using the get method we obtain a Option<T> so even if the index is invalid it returns None and therefore this is safe
match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
------------------------------------------------------

/* 
You can't have a mutable reference to a vector item while inserting/deleting from the vector
adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, 
if there isn’t enough room to put all the elements next to each other where the vector currently is. 
In that case, the reference to the first element would be pointing to deallocated memory. 
The borrowing rules prevent programs from ending up in that situation.
*/

let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0]; // immutable borrow occurs here

v.push(6); // the push() needs a mutable borrow here, ERROR ! can't have both immutable and mutable borrow at same scope.

------------------------------------------------------

// Iterating on a vector

let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}

// Iterate over mutable references to each element in a mutable vector in order to make changes to all the elements.
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;      // To change the value that the mutable reference refers to, we 
                   // have to use the dereference operator (*) to get to the value in i before we can use the += operator. 
}

// Update a value at index

let mut v = vec![1, 2, 3, 4, 5, 6];
let got = std::mem::replace(&mut v[3], 42); // [1, 2, 3, 42, 5, 6]

------------------------------------------------------
// You can store different type of values ina vector using a Enum

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

```