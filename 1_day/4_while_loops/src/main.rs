use std::io;


fn main() {
    let mut family_size = String::new();

    println!("How many family members are there?");

    io::stdin().read_line(&mut family_size)
    .expect("Failed to read line");

    let family_size: i64 = family_size.trim().parse()
    .expect("Please type a number!");

    family_generator(family_size);
}

fn family_generator(family_size: i64) {
    let mut s: i64 = 0;

    while s <= family_size {
        if s < family_size {
            println!("{} children - let's have more!", s);
        }else{
            println!("{} children - time to stop!", s);
        }
        s += 1;
    }
}