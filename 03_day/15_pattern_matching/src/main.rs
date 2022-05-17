fn main() {
    family_matches();
    more_family_matches();
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

/*
 * In the case bellow we're going to create a decoupled struture so we can
 * use the same function to match different types of data.
 * For doing so, i've created a new enum
 * and then a function to print the desired log according to
 * the type of data.
 */

fn more_family_matches(){
    enum FamilyMembers{
        Dad,
        Mom,
        Children,
    }

    fn print_family_member(family_members: FamilyMembers){

        match family_members {
            FamilyMembers::Dad => println!("Dad"),
            FamilyMembers::Mom => println!("Mom"),
            FamilyMembers::Children => println!("Children"),
        }
    }

    print_family_member(FamilyMembers::Dad);
    print_family_member(FamilyMembers::Mom);
    print_family_member(FamilyMembers::Children);
}