fn main() {
    family_matches();
}

/**
 * When we use enums, like in the code we've did before
 * we can have a faster way to know what kind of data
 * we are dealing with.
 * 
 * This is called pattern matching. 
 * To do this we basically use the match keyword, inform
 * the variable that we want to find pairs with, and then 
 * we inform the Enum and the value that we want to match.
 * After this we use a callback function to return a function
 * when we find a match for this variable.
 */


fn family_matches(){
    enum FamilyMembers {
        Human(String),
        Animal(String),
    }

    let scott = FamilyMembers::Human(String::from("Scott"));
    let rex = FamilyMembers::Animal(String::from("Rex"));

    //Instead of using a if...else statement, we can pass the variable as reference, then find what pattern it fits.
    match scott {
        FamilyMembers::Human(name) => println!(" - {} -", name),
        FamilyMembers::Animal(name) => println!("{}", name),
    }

    println!("\n");

    match rex {
        FamilyMembers::Human(name) => println!(" - {} - ", name),
        FamilyMembers::Animal(name) => println!("{}", name),
    }
}