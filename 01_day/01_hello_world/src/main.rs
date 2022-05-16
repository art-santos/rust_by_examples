fn main() {
    greet();
    greet_family();
    greet_family_typed();
    greet_family_imutable();
}


//The brackts are replaced by the variable;
fn greet(){
    let name = "Rust";
    println!("Hello, {}!", name);
}

/*  
*When we do want to increase the number
*of variables displayed, we can use the comma operator.
*We declare them like this.
*
*Detail:
*For function names that have more than two words the
*compiler recomends us to use snake_case.
*/
fn greet_family(){
    let dad = "Dad";
    let mom = "Mother";
    println!("Hello, {} and {}!", dad, mom);
}

/*  
*Rust is a statically typed language.
*This means that we can't assign a value to a variable
*that doesn't match the type of the variable.
* the syntax is:
* &_type_ 
*/
fn greet_family_typed(){
    let dad: &str = "Dad";
    let mom: &str = "Mother";
    println!("Hello, {} and {}!", dad, mom);
}

/*  
*In rust we can't simply assign a value to a variable
*cause all variable are immutable by default.
*We need to tell the compiler that we want to change
*the value of the variable.
*We do this by using the mut keyword.
*/
fn greet_family_imutable(){
    // let dad: &str = "Dad";
    // dad = "Marcus";
    let mut dad: &str = "Dad";
    dad = "Marcus";
    println!("Hello, {}!", dad);
}