fn main() {
    println!("Hello, world!");
}

/**
 * Optional types are vey useful in the case of the absence of 
 * a value.
 */


fn family_option_type(){
    enum Option<T>{
        Some(T),
    }
    let x = Option::Some(1);
    let y = Option::Some('a');
}