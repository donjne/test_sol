// A question was thrown during the course of this bootcamp and it is:

// What's the use of rust immutability if shadowing 

use std::error::Error;

// Example hash function, replace with your actual hashing function
fn hash_function(password: &str) -> String {
    // Placeholder hashing, you should use a proper hashing algorithm
    format!("hashed: {}", password)
}

fn authenticate(form: &LoginForm) -> Result<(), Box<dyn Error>> {
    let password: &str = &form.password;
    let confirm_password: &str = &form.confirm_password;
    let password = hash_function(password);

    if password == *confirm_password {
        println!("Success");
        Ok(())
    } else {
        Err("Passwords do not match".into())
    }
}

// Example struct for login form
struct LoginForm {
    password: String,
    confirm_password: String,
}

fn main() {
    // Example usage
    let form = LoginForm {
        password: String::from("password123"),
        confirm_password: String::from("password123"),
    };
    if let Err(e) = authenticate(&form) {
        println!("Error: {}", e);
    }
}
