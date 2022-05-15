fn main() {
    family_clone();
}

/*
*Alternativly to references we can use the 'clone' method.
*/

fn family_clone(){
    #[derive(Clone, Debug)]
    struct FamilyStructure{
        dad: i32,
        mom: i32,
        child: i32,
    }

    let f1 = FamilyStructure {dad: 1, mom: 1, child: 2,};
    let f2 = f1.clone();
    println!("{} {} {}", f1.dad, f1.mom, f1.child);
    println!("{} {} {}", f2.dad, f2.mom, f2.child);
}