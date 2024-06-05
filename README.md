# 30 DAYS OF SOLANA BOOTCAMP - OWERRI

This markdown file is dedicated to the 30 days of Solana bootcamp held in Owerri.

## References and Borrowing

A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; that data is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.

```rust
fn main() {

let s1 = String::from("hello");

let len = calculate_length(&s1);

println!("The length of '{}' is {}.", s1, len);

}

fn calculate_length(s: &String) -> usize {

s.len()

}
```

### Assignment: Make your research about the Drop Trait in rust

We call the action of creating a reference borrowing. As in real life, if a person owns something, you can borrow it from them. When you’re done, you have to give it back. You don’t own it.

```rust
fn main() {

let s = String::from("hello");

change(&mut s);

}

fn change(some_string: &mut String) {

some_string.push_str(", world");


}
```

## Dangling References

In languages with pointers, it’s easy to erroneously create a dangling pointer—a pointer that references a location in memory that may have been given to someone else—by freeing some memory while preserving a pointer to that memory. In Rust, by contrast, the compiler guarantees that references will never be dangling references: if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.

Let’s try to create a dangling reference to see how Rust prevents them with a compile-time error:

```rust
fn main() {

let reference_to_nothing = dangle();

}

fn dangle() -> &String {

let s = String::from("hello");

&s

}
```

## The Rules of References

Let’s recap what we’ve discussed about references:

1. At any given time, you can have either one mutable reference or any number of immutable references.
2. References must always be valid.

## Strings

In rust, we have various types of strings. Let's look at them:

String slices

```rust
let s = "Hello, world!";
let s = "Hello, world!";
&str String
```

The type of s here is &str : it’s a slice pointing to that specific point of the binary. This is also why string literals are immutable; &str is an immutable reference.

Owned strings

```rust
let my_string = String::from("hello world");
```

This type of string takes ownership of the value.

## Structs

A struct, or structure, is a custom data type that lets you package together and name multiple related values that make up a meaningful group. If you’re familiar with an object-oriented language, a struct is like an object’s data attributes.

To define a struct, we enter the keyword struct and name the entire struct. A struct’s name should describe the significance of the pieces of data being grouped together. Then, inside curly brackets, we define the names and types of the pieces of data, which we call fields.

```rust
struct User {
active: bool, //Struct field
username: String,
email: String,
sign_in_count: u64,
}
```

To use a struct, we would create an instance like this:

```rust
fn main() {
let user1 = User {
active: true,
username: String::from("someusername123"),
email: String::from("someone@example.com"),
};
println!("{}", user1.username)
}
```

### Assignment: Learn how to update a struct using the struct update syntax

Tuple Structs
Rust also supports structs that look similar to tuples, called tuple structs. Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields; rather, they just have the types of the fields. Tuple structs are useful when you want to give the whole tuple a name and make the tuple a different type from other tuples, and when naming each field as in a regular struct would be verbose or redundant.

```rust
struct Color(&str, &str, &str);
struct Point(i32, i32, i32);

fn main() {
let color = Color("red", "blue", "green");
let origin = Point(0, 0, 0);
println!("{}", color.2)
}
```

Unit like structs without any fields
You can also define structs that don’t have any fields! These are called unit-like structs because they behave similarly to () , the unit type.

```rust
struct AlwaysEqual;

fn main() {
let subject = AlwaysEqual;
}
```

Ownership of Structs
You cannot use references in your struct fields without lifetimes. See an example

```rust
struct User {
active: bool,
username: String,
email: &str,
sign_in_count: u64,

}

fn main() {

let user1 = User {
active: true,
username: String::from("emmanuel"),
email: "someone@example.com",
sign_in_count: 1,
};

}
```

### Assignment: Make a research about lifetimes eg. &'a str

## Methods

Methods are similar to functions: we declare them with the fn keyword and a name, they can have parameters and a return value, and they contain some code that’s run when the method is called from somewhere else.

```rust
#[derive(Debug)]



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

## ASSOCIATED FUNCTIONS

```rust
fn main() {
    me();
}

struct Rectangle {
width: u32,
height: u32,
}


impl Rectangle {
fn new(size: u32) -> Self {
// Struct -> Impl -> Associated function
Self {
width: size,
height: size,
}
}
}
let y = Rectangle{
    width: Rectangle::new(78),
    height: Rectangle::new(56)
}
```

## Automatic Referencing and Dereferencing

Rust has a feature called automatic referencing and dereferencing. Calling methods is one of the few places in

Rust that has this behavior.

### Assignment: Explore the concept of automatomatic referencing and dereferencing

Here’s how it works: when you call a method with object.something() , Rust automatically adds in & , &mut , or * so object matches the signature of the method.

In other words, the following are the same:
p1.distance(&p2);
(&p1).distance(&p2);

## Associated functions

All functions defined within an impl block are called associated functions because they’re associated with the type named after the impl . We can define associated functions that don’t have self as their first parameter (and thus are not methods) because they don’t need an instance of the type to work with. We’ve already used one function like this: the String::from function that’s defined on the String type.

Associated functions that aren’t methods are often used for constructors that will return a new instance of the struct. These are often called new , but new isn’t a special name and isn’t built into the language.

## Enums

We’ll look at enumerations, also referred to as enums. Enums allow you to define a type by enumerating its possible variants. First we’ll define and use an enum to show how an enum can encode meaning along with data. Next, we’ll explore a particularly useful enum, called Option , which expresses that a value can be either something or nothing.

```rust
enum IpAddrKind {
V4(String),// first variant
V6(String),// second variant
}

// Primitve Data Types
// Strings
// Integers
// Floats
// Unit

//Custom Data Types
// Enums or Enumeration

struct IpAddr {
    kind: IpAddrKind,
    value: String,
}
let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
```

In another example, we import the std::net crate to create an actual IP address

```rust
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr}; // standard library

// To create ipv4 and ipv6 addresses

pub enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

// Creating actual ip addresses

let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
let localhost_v6 = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));

Ipv4Addr::new(127, 0, 0, 1)

struct Ipv4Addr {}

impl Ipv4Addr {
    fn new() {}
}

// new 

assert_eq!("127.0.0.1".parse(), Ok(localhost_v4));
assert_eq!("::1".parse(), Ok(localhost_v6));

assert_eq!(localhost_v4.is_ipv6(), false); //actual result, what you expect
assert_eq!(localhost_v4.is_ipv4(), true);
```

## The Option Enum

Option, which is another enum defined by the
standard library. The Option type encodes the very common scenario in which a value
could be something or it could be nothing.

For example, if you request the first item in a non-empty list, you would get a value. If you
request the first item in an empty list, you would get nothing. Expressing this concept in
terms of the type system means the compiler can check whether you’ve handled all the
cases you should be handling; this functionality can prevent bugs that are extremely
common in other programming languages.

Programming language design is often thought of in terms of which features you include,
but the features you exclude are important too. Rust doesn’t have the null feature that many
other languages have. Null is a value that means there is no value there. In languages with
null, variables can always be in one of two states: null or not-null.

Tony Hoare, the inventor of null, in his 2009 presentation “Null References: The Billion Dollar Mistake,” said this:
I call it my billion-dollar mistake. At that time, I was designing the first comprehensive
type system for references in an object-oriented language. My goal was to ensure that
all use of references should be absolutely safe, with checking performed automatically
by the compiler. But I couldn’t resist the temptation to put in a null reference, simply
because it was so easy to implement. This has led to innumerable errors,
vulnerabilities, and system crashes, which have probably caused a billion dollars of
pain and damage in the last forty years.

```rust
enum Option<T> {
None,
Some(T),
}
```

```rust
let some_number = Some(5);
let some_char = Some('e');
let absent_number: Option<i32> = None;
```

## Match

Think of a match expression as being like a coin-sorting machine: coins slide down a track with variously sized holes along it, and each coin falls through the first hole it encounters that it fits into.

```rust
enum Coin {
Penny,
Nickel,
Dime,
Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
match coin {
Coin::Penny => 1,
Coin::Nickel => 5,
Coin::Dime => 10,
Coin::Quarter => 25,
}
}
```

Let's see another example:

```rust

fn plus_one(x: Option<i32>) -> Option<i32> {
match x {
None => None,
Some(i) => Some(i + 1),
}
}
let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

Let’s examine the first execution of plus_one in more detail. When we call plus_one(five) ,
the variable x in the body of plus_one will have the value Some(5) . We then compare that
against each match arm:
The Some(5) value doesn’t match the pattern None , so we continue to the next arm:
Does Some(5) match Some(i) ? It does! We have the same variant. The i binds to the value
contained in Some , so i takes the value 5 . The code in the match arm is then executed, so
we add 1 to the value of i and create a new Some value with our total 6 inside.

### Matches Are Exhaustive

There’s one other aspect of match we need to discuss: the arms’ patterns must cover all
possibilities. Consider this version of our plus_one function, which has a bug and won’t
compile:
We didn’t handle the None case, so this code will cause a bug. Luckily, it’s a bug Rust knows
how to catch. If we try to compile this code, we’ll get this error:
Rust knows that we didn’t cover every possible case, and even knows which pattern we
forgot! Matches in Rust are exhaustive: we must exhaust every last possibility in order for the
code to be valid. Especially in the case of `Option<T>` , when Rust prevents us from forgetting
to explicitly handle the None case, it protects us from assuming that we have a value when
we might have null, thus making the billion-dollar mistake discussed earlier impossible.

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
match x {
Some(i) => Some(i + 1),
}
}
```

## Catch all pattern

Rust also has a pattern we can use when we want a catch-all but don’t want to use the value
in the catch-all pattern: _ is a special pattern that matches any value and does not bind to
that value. This tells Rust we aren’t going to use the value, so Rust won’t warn us about an
unused variable.

```rust
let dice_roll = 9;

match dice_roll {
3 => add_fancy_hat(),
7 => remove_fancy_hat(),
_ => reroll(),

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}

}
```

## if let control flow syntax

The if let syntax lets you combine if and let into a less verbose way to handle values
that match one pattern while ignoring the rest.

Using a match expression we would express this code as:

```rust
let config_max = Some(3u8);
match config_max {
Some(max) => println!("The maximum is configured to be {}", max),
_ => (),
}
```

Instead we can write it in a shorter way as:

```rust
let config_max = Some(3u8);
if let Some(max) = config_max {
println!("The maximum is configured to be {}", max);
} 
```

### Assignment: The Fibonacci sequence begins with [0,1]. For n>1, the n'th Fibonacci number is calculated recursively as the sum of the n-1'th and n-2'th Fibonacci numbers. Write a function fib(n) that calculates the n'th Fibonacci number

```rust
fn fib(n: u32) -> u32 {
if n < 2 {
// The base case.
todo!("Implement this")
} else {
// The recursive case.
todo!("Implement this")
}
}

fn main() {
let n = 20;
println!("fib({n}) = {}", fib(n));
}
```

## Packages, Crates and Modules

Rust has a number of features that allow you to manage your code’s organization, including
which details are exposed, which details are private, and what names are in each scope in
your programs. These features, sometimes collectively referred to as the module system,
include:
Packages: A Cargo feature that lets you build, test, and share crates
Crates: A tree of modules that produces a library or executable
Modules and use: Let you control the organization, scope, and privacy of paths
Paths: A way of naming an item, such as a struct, function, or module

## Packages and Crates

A crate is the smallest amount of code that the Rust compiler considers at a time. Even if you
run rustc rather than cargo and pass a single source code file (as we did all the way back
in the “Writing and Running a Rust Program” section of Chapter 1), the compiler considers
that file to be a crate. Crates can contain modules, and the modules may be defined in other
files that get compiled with the crate, as we’ll see in the coming sections.
A crate can come in one of two forms: a binary crate or a library crate. Binary crates are
programs you can compile to an executable that you can run, such as a command-line
program or a server. Each must have a function called main that defines what happens
when the executable runs. All the crates we’ve created so far have been binary crates.
Library crates don’t have a main function, and they don’t compile to an executable. Instead,
they define functionality intended to be shared with multiple projects.

## The Crate root

Most of the time when Rustaceans say “crate”, they mean library crate, and they use “crate”
interchangeably with the general programming concept of a “library".

The crate root is a source file that the Rust compiler starts from and makes up the root
module of your crate

## Packages

A package is a bundle of one or more crates that provides a set of functionality. A package
contains a Cargo.toml file that describes how to build those crates. Cargo is actually a
package that contains the binary crate for the command-line tool you’ve been using to build
your code. The Cargo package also contains a library crate that the binary crate depends on.
Other projects can depend on the Cargo library crate to use the same logic the Cargo
command-line tool uses.
A package can contain as many binary crates as you like, but at most only one library crate.
A package must contain at least one crate, whether that’s a library or binary crate.

```shell
$ cargo new my-project
Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
lib.rs
```

Here, we have a package that only contains src/main.rs, meaning it only contains a binary
crate named my-project . If a package contains src/main.rs and src/lib.rs, it has two crates: a
binary and a library, both with the same name as the package. A package can have multiple
binary crates by placing files in the src/bin directory: each file will be a separate binary crate.

## Modules to Control Scope and Privacy

Modules Cheat Sheet:
Here we provide a quick reference on how modules, paths, the use keyword, and the pub
keyword work in the compiler, and how most developers organize their code. We’ll be going
through examples of each of these rules throughout this chapter, but this is a great place to
refer to as a reminder of how modules work.
Start from the crate root: When compiling a crate, the compiler first looks in the
crate root file (usually src/lib.rs for a library crate or src/main.rs for a binary crate) for
code to compile.

Declaring modules: In the crate root file, you can declare new modules; say, you
declare a “garden” module with mod garden; . The compiler will look for the module’s
code in these places:
Inline, within curly brackets that replace the semicolon following mod garden
In the file src/garden.rs
In the file src/garden/mod.rs

Declaring submodules: In any file other than the crate root, you can declare
submodules. For example, you might declare mod vegetables; in src/garden.rs. The
compiler will look for the submodule’s code within the directory named for the parent
module in these places:
Inline, directly following mod vegetables , within curly brackets instead of the
semicolon
In the file src/garden/vegetables.rs
In the file src/garden/vegetables/mod.rs
Paths to code in modules: Once a module is part of your crate, you can refer to code
in that module from anywhere else in that same crate, as long as the privacy rules
allow, using the path to the code. For example, an Asparagus type in the garden
vegetables module would be found at crate::garden::vegetables::Asparagus .

Private vs public: Code within a module is private from its parent modules by default.
To make a module public, declare it with pub mod instead of mod . To make items
within a public module public as well, use pub before their declarations.

```rust
//main.rs

use crate::garden::vegetables::Asparagus;
pub mod garden;
fn main() {
let plant = Asparagus {};
println!("I'm growing {:?}!", plant);
}
```

The pub mod garden; line tells the compiler to include the code it finds in src/garden.rs.

```rust
//garden.rs
pub mod vegetables;
```

Here, pub mod vegetables; means the code in src/garden/vegetables.rs is included too. That
code is:

```rust
//vegetables.rs
#[derive(Debug)]
pub struct Asparagus {}
```

## Grouping related modules

```rust
pub mod front_of_house {
pub mod hosting {
fn add_to_waitlist() {}
fn seat_at_table() {}
}
pub mod serving {
fn take_order() {}
fn serve_order() {}
fn take_payment() {}
}
}

mod less {
    use super::*
}
```

Modules can also hold definitions for other items, such as structs, enums, constants, traits, and as.

## Paths

// Assignment: make a deep research on paths

A path can be in two forms:

1. An absolute path is the full path starting from a crate root; for code from an external crate, the absolute path begins with the crate name, and for code from the current
crate, it starts with the literal crate .

2. A relative path starts from the current module and uses self , super , or an identifier in the current module

```rust
mod my {
    fn function() {
        println!("called `my::function()`");
    }
    
    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }
    
    pub fn indirect_call() {
        // Let's access all the functions named `function` from this scope!
        print!("called `my::indirect_call()`, that\n> ");
        
        // The `self` keyword refers to the current module scope - in this case `my`.
        // Calling `self::function()` and calling `function()` directly both give
        // the same result, because they refer to the same function.
        self::function();
        function();
        
        // We can also use `self` to access another module inside `my`:
        self::cool::function();
        
        // The `super` keyword refers to the parent scope (outside the `my` module).
        super::function();
        
        // This will bind to the `cool::function` in the *crate* scope.
        // In this case the crate scope is the outermost scope.
        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}
```

## Super

We can construct relative paths that begin in the parent module, rather than the current
module or the crate root, by using super at the start of the path. This is like starting a
filesystem path with the .. syntax.

```rust
fn deliver_order() {}

mod back_of_house {
fn fix_incorrect_order() {
cook_order();
super::deliver_order();
}
fn cook_order() {}
}
```

## Use keyword

```rust
mod front_of_house {
pub mod hosting {
pub fn add_to_waitlist() {}
}
}
use crate::front_of_house::hosting::add_to_waitlist;
pub fn eat_at_restaurant() {
add_to_waitlist();
}
```

## Bringing other items with use

On the other hand, when bringing in structs, enums, and other items with use , it’s idiomatic
to specify the full path.

```rust
use std::collections::HashMap;
fn main() {
let mut map = HashMap::new();
map.insert(1, 2);
}
```

## Providing names with the as keyword

There's a solution for bringing two types of the same name into the same scope.

```rust
use std::fmt::Result;
use std::io::Result as IoResult;
fn function1() -> Result {
// --snip--
}
fn function2() -> IoResult<()> {
// --snip--
}
```

## Common collections

Rust’s standard library includes a number of very useful data structures called collections.
Most other data types represent one specific value, but collections can contain multiple
values. Unlike the built-in array and tuple types, the data these collections point to is stored
on the heap, which means the amount of data does not need to be known at compile time
and can grow or shrink as the program runs.

We’ll discuss three collections that are used very
often in Rust programs:

1. A vector allows you to store a variable number of values next to each other.

2. A string is a collection of characters. We’ve mentioned the String type previously, but
in this chapter we’ll talk about it in depth.

3. A hash map allows you to associate a value with a particular key. It’s a particular
implementation of the more general data structure called a map.

## Vectors

The first collection type we’ll look at is `Vec<T>` , also known as a vector. Vectors allow you to
store more than one value in a single data structure that puts all the values next to each
other in memory. Vectors can only store values of the same type. They are useful when you
have a list of items, such as the lines of text in a file or the prices of items in a shopping cart.

## Creating a new vector

```rust
let v: Vec<i32> = Vec::new();

//Creating a new vector containing values
let v = vec![1, 2, 3]; 
```

## Updating a vector

```rust
let mut v = Vec::new(); //nothing yet
v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

## Reading elements of vectors

There are two ways to reference a value stored in a vector: via indexing or using the get
method. In the following examples, we’ve annotated the types of the values that are
returned from these functions for extra clarity.

```rust
let v = vec![1, 2, 3, 4, 5];
let third: &i32 = &v[2];
println!("The third element is {third}");
//or
let third: Option<&i32> = v.get(2);
match third {
Some(third) => println!("The third element is {third}"),
None => println!("There is no third element."),
}
```

## Attempting to break a reference rule

```rust
let mut v = vec![1, 2, 3, 4, 5];
let first = &v[0];
// first = 1
v.push(6);
println!("The first element is: {first}");
```

## Iterating over a vector using a for loop

```rust
let v = vec![100, 32, 57];
for i in &v {
println!("{i}");
}
```

## Iterating over mutable references

```rust
let mut v = vec![100, 32, 57];
for i in &mut v {
*i += 50;
}
```

## Using an enum to store mutiple types

Vectors can only store values that are the same type. This can be inconvenient; there are
definitely use cases for needing to store a list of items of different types. Fortunately, the
variants of an enum are defined under the same enum type, so when we need one type to
represent elements of different types, we can define and use an enum!

```rust
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

## Dropping a vector when it goes out of scope

```rust
{
let v = vec![1, 2, 3, 4];
// do stuff with v
} // <- v goes out of scope and is freed here
```

When the vector gets dropped, all of its contents are also dropped, meaning the integers it
holds will be cleaned up. The borrow checker ensures that any references to contents of a
vector are only used while the vector itself is valid.

## String

We’ll first define what we mean by the term string. Rust has only one string type in the core
language, which is the string slice str that is usually seen in its borrowed form &str .

String
The String type, which is provided by Rust’s standard library rather than coded into the
core language, is a growable, mutable, owned, UTF-8 encoded string type. When Rustaceans
refer to “strings” in Rust, they might be referring to either the String or the string slice
&str types, not just one of those types. Although this section is largely about String , both
types are used heavily in Rust’s standard library, and both String and string slices are UTF-8 encoded.

## Creating a new string

```rust
let mut s = String::new();

let data: String = "initial contents".to_string();
// the method also works on a literal directly:
let s = "initial contents".to_string();

```

## Updating a string

```rust
let mut s = String::from("foo");
s.push_str("bar");

let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("s2 is {s2}");
```

If the push_str method took ownership of s2 , we wouldn’t be able to print its value on the
last line. However, this code works as we’d expect.

## Concatenating strings

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
```

## Using the format macro

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");
let s = s1 + "-" + &s2 + "-" + &s3;
println!("{}", s1)
```

Instead we will rather do it this way:

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");
let s = format!("{s1}-{s2}-{s3}");
```

## Indexing into a string

Indexing into a string is often a bad idea because it’s not clear what the return type of the
string-indexing operation should be: a byte value, a character, a grapheme cluster, or a
string slice.

### Assignment: Learn how to iterate over a string

## Hashmaps

Storing keys associated with values in a hashmap:
Indexing into a string is often a bad idea because it’s not clear what the return type of the
string-indexing operation should be: a byte value, a character, a grapheme cluster, or a
string slice.

Hash maps are useful when you want to look up data not by using an index, as you can with
vectors, but by using a key that can be of any type. For example, in a game, you could keep
track of each team’s score in a hash map in which each key is a team’s name and the values
are each team’s score. Given a team name, you can retrieve its score.

Creating a new Hashmap
One way to create an empty hash map is using new and adding elements with insert

```rust
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

Accessing values in a hashmap
We can get a value out of the hash map by providing its key to the get method.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
let team_name = String::from("Blue");
let score = scores.get(&team_name).copied().unwrap_or(0);
```

Iterating over each key/value in a hashmap using a for loop

```rust
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
for (key, value) in &scores {
println!("{key}: {value}");
}
```

The code will print each pair in an arbitrary order:
Yellow: 50
Blue: 10

## Hashmaps and Ownerships

For types that implement the Copy trait, like i32 , the values are copied into the hash map.
For owned values like String , the values will be moved and the hash map will be the owner

```rust
use std::collections::HashMap;
let field_name = String::from("Favorite color");
let field_value = String::from("Blue");
let mut map = HashMap::new();
map.insert(field_name, field_value);



// field_name and field_value are invalid at this point, try using them and
// see what compiler error you get!
```

## Updating a hashmap

Overwriting a value

```rust
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);
```

Hashing functions
By default, HashMap uses a hashing function called SipHash that can provide resistance to
Denial of Service (DoS) attacks involving hash tables1
. This is not the fastest hashing
algorithm available, but the trade-off for better security that comes with the drop in
performance is worth it. If you profile your code and find that the default hash function is
too slow for your purposes, you can switch to another function by specifying a different
hasher. A hasher is a type that implements the BuildHasher trait.

### Assignment: Learn about Error handling in rust

### Next class: Generics, Traits, Lifetimes and more
