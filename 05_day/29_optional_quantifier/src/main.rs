fn main() {
    println!("Hello, world!");
}

/*
 * In the macro rules system, there's no way to specify that
 * a pattern is opitional.
 * 
 * So, for specifing an optoinal quantifier in macros, we must use a , for this.
 *   ($($key:expr => $value:expr ),* $(,)* ).
 */