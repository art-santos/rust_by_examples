use std::io;

fn main() {
    let mut family_size: String = String::new();
    let mut generations: String = String::new();

    println!("How big is this family now?");

    io::stdin().read_line(&mut family_size)
    .expect("Failed to read line");

    println!("How many generations will this family live?");

    io::stdin().read_line(&mut generations)
    .expect("Failed to read line");

    let family_size: i32 = family_size.trim().parse()
    .expect("Please type a number!");

    let generations: i32 = generations.trim().parse()
    .expect("Please type a number!");

    family_function(family_size, generations);

}


/**
 * That's just a brief reminder of how to declarate and type a function
 * @func family_function shall receive 
 * @arg family_size
 * @arg generations
 * then it shall return the size of the
 * genealogic tree in a certain amount of time
 */
fn family_function(family_size: i32, generations: i32){
    /*
    according to https://worldpopulationreview.com/country-rankings/total-fertility-rate
    the average reproduction rate between humans is 2.4 sons per couple.
    */
    let average_reproduction_rate: f64 = 2.4;
    /*
    according to https://www.ancestry.ca/learn/learningcenters/default.aspx?section=lib_Generation#:~:text=As%20a%20matter%20of%20common,it%20varies%20case%20by%20case.
    each generation takes about 25 to 38 years to mature and have children
    */
    let average_generation_lifespan: i32 = 33;

    let total_family_population: f64 = (family_size as f64) * average_reproduction_rate;

    let total_population_overtime: f64 = total_family_population * (generations as f64);

    let total_time: i32 = generations * average_generation_lifespan;

    println!(
    "It's going to take {} years to the family population to reach {} members",
    total_time,
    total_population_overtime
    );
}