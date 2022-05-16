use std::io;

fn main() {
//First function that uses only static data
    family_control();

//Second function that uses dynamic data entered by the user
    let mut your_family_size = String::new();
// The variable is always going to be an string when we get an input from the user

//Here we're going to print in the console a statement requiring the user input
    println!("How many family members are there?");

//Here we're going to get the user input
    io::stdin().read_line(&mut your_family_size)
    .expect("Failed to read line");

//Here we're going to convert the string to an integer
    let your_family_size: i32 = your_family_size.trim().parse()
    .expect("Please type a number!");

//Here we're going to pass it to the function and wait for it to send us the results
    family_control_helper(your_family_size);

}

fn family_control() {
    let family_members: [&str; 4] = ["Dad", "Mom", "Sister", "Brother"];
    let family_size: usize = family_members.len();

    println!("There are {} family members", family_size);

//Here's a flow control using if and else
    if family_size > 3 {
        println!("Take birth control!");
    } else {
        println!("Have more children!");
    }
}

fn family_control_helper(family_size: i32){
    if family_size > 3 {
        println!("Take birth control!");
    } else {
        println!("Have more children!");
    }
}