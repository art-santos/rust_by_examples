use std::io;

/**
 * The intention is to create a code that iterates throw a loop
 * and prints a message depending on the number of family members
 */

fn main() {
    /*
     * @param family_size is going to be the size that
     * we're going to use to control the flow control.
     * DETAIL:
     * this variable has to be mutable
     */
    let mut family_size = String::new();
    /*
     * @function println! is going to be used to ask
     * how big we want the family to be.
     */
    println!("How many family members are there?");
    /*
     * @io::stdin is going to read the user input
     * and pass it to the family_size variable
     */
    io::stdin().read_line(&mut family_size)
    .expect("Failed to read line");

    /*
     *@func trim() and @func parse() are going to
     * be used to convert the string into a number i32
     * DETAIL:
     * used i64 to allow bigger numbers so you can see the power
     * of Rust compared to JS, for example.
     */
    let family_size: i64 = family_size.trim().parse()
    .expect("Please type a number!");
    /*
    *@func family_generator is going to be used to log
    *if the family needs or not to have more children
    *until it reaches the desired number.
    */
    family_generator(family_size);
}

fn family_generator(family_size: i64) {
    /*
    *@variable s is going to used to store the
    *number of time we've iterated throw our function.
    *That's going to be used as our r
    */
    let mut s: i64 = 0;
    /*
    * Here's the loop. While it is true, it will keep iterating
    * If it becomes bigger than our desired number then it's going to
    *print in the console the message that we want.
    */
    while s <= family_size {
        if s < family_size {
            println!("{} children - let's have more!", s);
        }else{
            println!("{} children - time to stop!", s);
        }
        s += 1;
    }
}