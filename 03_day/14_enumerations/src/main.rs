fn main() {
    family_enumarations();
    more_family_enumerations();
}

/*
 * Enumerators are very similar to structures, but with the difference
 * That the data structure, by default, is made to store values
 * and essentially limit what values can be associated with an
 * specif variable in Rust. 
 * 
 * Enums are going to be used, basically, to limit the values
 * that can be associated with a variable. In a dynamic way.
 * 
 * Let's say, for example, that you want just people called Julia
 * or Gary to enter your software.
 * 
 * Then you can create a enum to limit this persons to enter it.
 */

fn family_enumarations(){

    enum FamilyKinds {
        Human(String),
        Alien(String),
    }

    struct FamilyStructure {
        name: String,
        kind: FamilyKinds,
    }

    let family_1 = FamilyStructure {
        name: String::from("John"),
        kind: FamilyKinds::Human(String::from("Human")),
    };

    let family_2 = FamilyStructure {
        name: String::from("El"),
        kind: FamilyKinds::Alien(String::from("Alien")),
    };

    println!("{}", family_1.name);
    println!("{}", family_2.name);
    
}
