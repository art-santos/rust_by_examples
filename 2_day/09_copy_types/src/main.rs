fn main() {
    family_clone();
}

/*
*Some types are not moved when we reassing them.
*for example, integers are not moved when we reassing them.
*Since copy requires clone, we can reassing this latter on.
*/

fn family_clone(){
    #[derive(Clone, Copy)]
    struct FamilyStructure{
        dad: i32,
        mom: i32,
        child: i32,
    }

    let f1 = FamilyStructure {dad: 1, mom: 1, child: 2,};
    let f2 = f1;
    println!("{} {} {}", f1.dad, f1.mom, f1.child);
    println!("{} {} {}", f2.dad, f2.mom, f2.child);
}