fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
let first = &v[0];
// first = 1
println!("The first element is: {first}");

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
    ];

    {
    let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here
let data: String = "initial contents".to_string();

let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");
let s = s1 + "-" + &s2 + "-" + &s3;
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
    }
    
