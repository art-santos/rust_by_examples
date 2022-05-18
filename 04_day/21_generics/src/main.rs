fn main() {
    println!("{}", family_generics(1, 0).0);
}


/*
 *Generics working by making a function more open to new
 *types. So, this way, we can avoid code duplication. 
 */


/**
 * What is happening?
 * After the function declaration, we do use the PartialOrd
 * trait to make the function more open to new types.
 * When we associate two different traits, that's called a trait bound
 * Now, we can associate this function with not just integers,
 * but floats, u8, i32, etc... The only condition is that both
 * types shall be equal. 
 */


fn family_generics<T: PartialOrd>(a: T, b: T) -> (T, T) {
    if a > b {
        (b, a)
    } else {
        (a, b)
    }
}