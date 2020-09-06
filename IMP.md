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

* Strings in rust are not so simple !


In rust Strings are UTF-8 encoded text. So this works: `String::from("नमस्ते");`
They are actaully a wrapper over a Vec<u8>.

Many of the same operations available with Vec<T> are available with String as well, like the new function to create a string.

```

// Creating a new string
let mut s = String::new();

// Get a string from a string literal(&str) or some preexisting data
// to_string() works on anything that implements the Display trait.
let s = "initial contents".to_string();

// Also, we can do... 
let s = String::from("initial contents"); // this has exactly same outcome as "initial contents".to_string()

-------------------------------------
// Appending

let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("s2 is {}", s2);

// Here the push-str method takes a slice(&str) so the ownership os s2 is not lost

// To push a single characterr we can use...
let mut s = String::from("lo");
s.push('l');
--------------------------------------

// Concating using `+` operator

let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2;

/* 

The `+` operator uses the `fn add(self, s: &str) -> String` method.
add() takes ownership of the first string, so in the example ownsership of `s1` is lost after `s1 + &s2`. But ownership for s2 is not lost since its borrowed.
Also, the 2nd parameter to add() is a slice(&str) but in the example we passed a reference to a string (&String).
Here rust uses automatic `deref coercoin` which turns &s2 into &s2[..] thus giving us a slice

*/
------------------------------

// Using format! macro, we get a string without losing ownership of any of the parameters

let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);
```

* Indexing into string

In rust a string is a series of bytes. A ut8 chracter may take 1 byte, 2byte...

So index a string directly is not possible, although index via range is possible but if we end up at a invalid char boundary our code will panicand crash...
`thread 'main' panicked at 'byte index 1 is not a char boundary;`

If we do `len` on string it gives us a the number of bytes,so...

String::from("Hola"); // Length is 4 since each letter is one byte

String::from("Здравствуйте"); // Length is 24 although there are 12 chracters since each character here is 2 bytes

There are 3 ways to read a string, for a hindi string “नमस्ते” ...

- Reading each byte. (vector of u8) Eg: [224, 164, 168, ... 135] (18 bytes)

- Reading as unicode scalar values or as rusts `char` type.
  Eg: ['न', 'म', 'स', '्', 'त', 'े']

- Reading a grapheme clusters, Eg: ["न", "म", "स्", "ते"]

Also, index into string directly is again not possible since indexing should be a O(1) operation but for stings Rust would have to walk through the contents from the beginning to the index to determine how many valid characters there were.


```
let s1 = String::from("hello");
let h = s1[0]; // ERROR ! `std::ops::Index<{integer}>` is not implemented for `std::string::String`


let hello = "Здравствуйте";
&hello[0..2]; // Reading first 2 bytes gives `З`
&hello[2..4]; // Reading 2nd 2 bytes gives `д`
&hello[4..5]; // thread 'main' panicked at 'byte index 5 is not a char boundary;
    

// Iterating on string...

for b in "नमस्ते".bytes() {
    println!("{}", b);  // [224, 164, 168, ... 135]
}

for c in "नमस्ते".chars() {
    println!("{}", c);   // ['न', 'म', 'स', '्', 'त', 'े']
}
```
Getting grapheme clusters from strings is complex, so this functionality is not provided by the standard library. 
Use this, to get this functionality: https://crates.io/crates/unicode-segmentation


* HashMap<K, V>

A hash map stores key value pairs, where both thew keys and values must be of homogenous types(all of the keys must have the same type, and all of the values must have the same type).

Just like vectors, hash maps store their data on the heap.

By default, a hashmap uses a “cryptographically strong” hash function, which provides security but might be slow for some applications, you can provide your own hash function(A hasher is a type that implements the BuildHasher trait.), many implementations are available on crates.io

Unlike strings and vectors a Hash map not automatically available in scope we need to import it like `use std::collections::HashMap`

```
use std::collections::HashMap;

let mut scores = HashMap::new(); // There is no built-in macro, like for vectors `vec![1, 2, 3]`

// Here the type (keys of type String and values of type i32) is automatically understood by rust as we insert values

scores.insert(String::from("Blue"), 10); // Adding keys and values
scores.insert(String::from("Yellow"), 50);



// Create a hash map using iterators and the collect method on a vector of tuples


let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

// Here zip method to create a vector of tuples
// Rust can infer the types that the hash map contains based on the types of the data in the vectors.
let mut scores: HashMap<_, _> =
    teams.into_iter().zip(initial_scores.into_iter()).collect();
```

== IMPORTANT ==
HERE WE DID NOT HAVE TO SPECIFY THE TYPES <K,V> WHEN DEFINING THE HashMap LIKE
HashMap::new() since we added vavlues ot it later `scores.insert(String::from("Blue"), 10);` , so the compiler infered types that the HashMap has keys of type String and values of type i32.

----------

Ownership:

For datat types which implement the Copy trait like i32, values are copied into the hash map when inserting.

*IMPORTANT:* 
For owned values like String, the values will be moved and the hash map will be the owner of those values

If we insert references to values into the hash map, the values won’t be moved into the hash map, however the values that the references point to must be valid for at least as long as the hash map is valid.

Accessing values:

We use the get() method that gives us a `Option<&V>` that is a Option enum
which wraps up a reference to the value, if the key does not exists it gives us `None`.

```
let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
let team_name = String::from("Blue");
let score = scores.get(&team_name);  // Gives a Some(&10)


// Iterating over key value paires in a hash using for loop

for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

Updating Values:

By default inserting a value for a key that already exists will overwrtie its value.

ENTRY:

Hash maps have a special API for this called entry that takes the key you want to check as a parameter. The return value of the entry method is an enum called Entry that represents a value that might or might not exist. 

Example...

```
scores.entry(String::from("Yellow")).or_insert(50); 
```

`scores.entry` gives us a `Entry`, the `or_insert` method on Entry gives us a mutable reference for the corresponding value if the key exists or if there is no value then it inserts a value and then returns a mutable reference to it.

Since or_insert gives us a mutable reference we can use it to update the value inside the hash, this is usefull if we want to updating a Value Based on the Old Value...

```
// Count occurance of each word in a sentence

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1; // in order to assign to that value, we must first dereference count using the asterisk (*)
}
```

----------------------------------------------------------- ERROR HANDLING -----------------------------------------------------------

* The panic! macro makes our code pani, cWhen the panic! macro executes, your program will print a failure message, unwind and clean up the stack, and then quit.

  By default on panic our code will unwind which means Rust walks back up the stack and cleans up the data from each function it encounters, but this might be slow,
  alternative is to immediately abort, which ends the program without cleaning up. 
  Memory that the program was using will then need to be cleaned up by the operating system.

  You can change the behaviour on panic, like in order to `abort` instead of unwinding in release mode, add a profile for release mode in 'Cargo.toml' file like..
  (aborting instead of unwinding can make our release binary smaller)

  ```
  [profile.release]
  panic = 'abort'
  ```

We can call panic like...

```
fn main() {
    panic!("crash and burn"); // thread 'main' panicked at 'crash and burn', src/main.rs:2:5
}
```

When we try to do something illegal, like access a array index not available a rust core library method might call the panic! macro for us...
'thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', /rustc/5e1a799842ba6ed4a57e91f7ab9435947482f7d8/src/libcore/slice/mod.rs:2806:10'

We can see a complete backtrace when our code panics by running it like 'RUST_BACKTRACE=1 cargo run'

* Recovering from erros with Result<T, E>

The Result enum has two values, Ok(T) or Err(E) here T represents the type of the value that will be returned in a success case within the Ok variant, 
and E represents the type of the error that will be returned in a failure case within the Err variant. Just like the Options enum the Result enum
and its variants are automatically brought into scope and we can use them directly.

Examples...

```

// Using match

use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}


-------------------------------------

// Create a File if it does not exists

// io::Error struct provided by the standard library, has a kind() method that returns an io::ErrorKind value. 
// The enum io::ErrorKind has variants representing the different kinds of errors that might result from an io operation. 
// The variant we want to use is ErrorKind::NotFound here...

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}


---------------------------------------
// Shortcuts...

use std::fs::File;

fn main() {

    // Unwraps and returns T from Result::Ok<T> that is the file handle in this case otherwise in case of Err it calls panic! macro
    let f = File::open("hello.txt").unwrap(); 

    // OR

    // Same as above but allows us to preovide a custom message incase of panic
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```

Propagating erros and `?` operator shortcut...
----------------------------------------------

Instead of calling panic! or handling the Result::Err we can return it to the caller in that case our fucntion must return a type like
`Result<String, io::Error>` and we can have `Err(e) => return Err(e)` in case of error.



If the `?` operator is placed after a Result, and the value of the Result is an Ok, the value inside the Ok will get returned from this expression,
and the program will continue. 
Otherwise the Err will be returned from the whole function as if we had used the return keyword so the error value gets propagated to the calling code.

Example...

```
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

```

Here if `File::open("hello.txt")` is success `?` returns the file handle T inside Result::Ok<T> over which we call `read_to_string` which loads up the content
of the file in the mutable string `s`. If at any point we recieve Result::Err<E> the `?` operator will resturn from the fucntion with Result::Err<E>.

This is common so there is shotcut method like `fs::read_to_string("hello.txt")` which does the same thing as above.

--------------------------------------------------------------
DIFFERENSE BETWEEN `return Err<E>` and using `?` operator...
--------------------------------------------------------------

Instead of directly returning a `Result::Err<E>` the `?` operator calls the `from()` function of the `From` trait in the standard library.
from() converts the error type received into the error type defined in the return type of the current function.

his is useful when a function returns one error type to represent all the ways a function might fail.
As long as each error type implements the from function to define how to convert itself to the returned error type, 
the ? operator takes care of the conversion automatically.

Finally the `?` operator can only be used in a function that returns Result or Option or another type that implements std::ops::Try.

The main() function either returns `()`, and conveniently, another valid return type is Result<T, E>. So we can do...

```
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> { // Box<dyn Error> type is called a trait object, it means “any kind of error.”
    let f = File::open("hello.txt")?;

    Ok(())
}
```

So, Using `?` in a main function is allowed only if the return type is `Result`.


----------------------------------------------------------- GENERICS -----------------------------------------------------------


* We can use generics in function signatures, structs or enums.

Exampls...

```
/*
This function largest is generic over some type T. This function has one parameter named list, which is a slice of values of type T. The largest function will return a reference to a value of the same type T.
*/

fn largest<T>(list: &[T]) -> &T {
    let mut largest = list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

---------------------
// Generic struct

struct Point<T, U> {
    x: T,
    y: U,
}

...
let both_float = Point { x: 1.0, y: 4.0 };
let integer_and_float = Point { x: 5, y: 4.0 };

---------------------
// Generic enums

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

--------------------
// Generics for 'impl'

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {

    // A simple getter
    fn x(&self) -> &T {
        &self.x
    }
}

```

It is possible to implment a method only for a specific generic type of the struct like, this method is only available for 'Point<f32>' structs 

```
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

```

Generics in impl and generics in method are different..

Here a new method is defined for 'Point<T, U>' so we had to specify the generic parameters as 'impl<T, U>'.

Next the method additionalty took a 'Point<V, W>' as a argument, which is of some other type <V,W> different from <T,U>
so we had to define the method like 'mixup<V, W>'.

```

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

...
let p1 = Point { x: 5, y: 10.4 };
let p2 = Point { x: "Hello", y: 'c' };

let p3 = p1.mixup(p2); // Point { x: 5, y: 'c' };
```

== PERFORMANCE OF GENERICS ==

Rust uses a technique called 'Monomorphization' which replaces all generics with concrete types at compile time and therefore there is absolutely NO COST at runtime.

Eg: 

If we create 2 Option<T> types like 'Some(5);' and 'Some(5.0);'

Rust will expands the generic definition of Option<T> into Option_i32 and Option_f64, thereby replacing the generic definition with the specific ones.


```

// Compile time Monomorphization, expanding generics:

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
```


====== TRAITS ======

Traits are similar to a feature often called interfaces in other languages, where you define a method and overrride it by implementing the interface.

Definign a trait:

```
pub trait Summary {
    fn summarize(&self) -> String;
}
```

Implementing a trait, syntax like: 'impl TRAIT for TYPE'

```
// Implementing the Summary trait for the NewsArticle struct
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// Now we can all summarize on an instance of NewsArticle like..
article.summarize()

```

 == Restriction when implementing traits ==

For implementing the trait on a type, either the type or the trait must be local to our crate. For example, we can implement the Display trait(external trait must be public) which is an external trait on our local type like 'NewsArticle'.

Similary we can implement our local trait like 'Summary' on an external type like String.

** However, WE CANNOT IMPLEMENT EXTRNAL TRAITS ON EXTERNAL TYPES.
Eg: We can't implement the external DIsplay trait on the external Vec<T> type.

Reason:

This restriction is part of a property of programs called coherence, and more specifically the orphan rule, 
so named because the parent type is not present. This rule ensures that other people’s code can’t break your code and vice versa. 
Without the rule, two crates could implement the same trait for the same type, and Rust wouldn’t know which implementation to use.


* We can have a default implementations for a trait, if a type implementing the trait does not specify its own implementation the default is used.

Eg:

```
pub trait Summary {
    fn summarize_author(&self) -> String;

    // Default implementation of summarize
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

```

In the above example if a type only defines 'summarize_author' when implementing the 'Summary' trait then its also gets the 'summarize'
method without having to implement it since there is a default implementation.


* Traits as parameters

When using generics in function signature we can use traits like..

```
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

'<T: Summary>' says thtat the function receives a generic type T that must implement the Summary trait.

An equivalent, shortcut to the above code is..

```
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

Sometimes the above shotcut cannot be used to express our intentions like...

```
pub fn notify<T: Summary>(item1: &T, item2: &T) { ... }

// is not the same as...

pub fn notify(item1: &impl Summary, item2: &impl Summary) {... }

// since it inforces that both item1, item2 must be of same generic type T
```


* Specifying Multiple Trait Bounds with the + Syntax

```
// Here, 'item' must implement both Display and Summary traits

pub fn notify<T: Summary + Display>(item: &T) { ... }

// or shotcut

pub fn notify(item: &(impl Summary + Display)) { ... }
```

Having multiple items having different traits might be confussing a cleaner synax is using the 'where' clause like...


```
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {...}

// The above can be written like...

fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{ ... }
```

* Returning Types that Implement Traits

We can also use the impl Trait syntax in the return position to return a value of some type that implements a trait

```
fn returns_summarizable() -> impl Summary { ... }
```

WARNING: When specifying the return type of a function implements a specific trait 
it DOES NOT mean you can conditionally return DIFFERENT TYPEs at just beacuse those 
types implement the trait. 

Your function can only return a specific type but specifying the return type as a 
trait only means that the retuned type must implement the trait.

So, this is INVALD ! Gives error like: '`if` and `else` have incompatible types'

```
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {...}
    } else {
        Tweet { ... }
    }
}
```
The above code wont compile, since it conditionally returns either a 'NewsArticle' or a 'Tweet' 
although both of them implement the 'SUmmary' trait.
While you can accept different types T you can only return a perticular type at all circumstances from the function.


* Conditionally implementing methods

By using a trait bound with an impl block that uses generic type parameters, we can implement methods conditionally for types that implement the specified traits.

Eg:

Here, we implement the 'cmp_display' method for only those types T of Pair<T> that implement both traits 'Display' and 'PartialOrd'

(This similar to how we implement a method for only a perticular type like Pair<f32>.)

```
...

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

```

Also, we can conditionally implement a trait for any type that implements another trait, this is called 'blanket'.

For example, the standard library implements the ToString trait on any type that implements the Display trait. 
The impl block in the standard library looks similar to this code:

```
impl<T: Display> ToString for T { // Implement the 'ToString' function for any Type T that implements the 'Display' trait
    // --snip--
}
```

Because the standard library has this blanket implementation, we can call the to_string method defined by the ToString trait on 
any type that implements the Display trait. 

For example, `let s = 3.to_string();`



Advantage over other languages:
-------------------------------
In dynamically typed languages, we would get an error at runtime if we called a method on a type which didn’t define the method. 

But Rust moves these errors to compile time so we’re forced to fix the problems before our code is even able to run. 
Additionally, we don’t have to write code that checks for behavior at runtime because we’ve already checked at compile time. 
Doing so improves performance without having to give up the flexibility of generics.
