fn main() {
    family_move();
}

/*
 *In Rust, values are moved instead of copied.
    The language does  that so it can avoid unnecessary copies.
    This is called "move semantics".
 */

 fn family_move(){
    struct FamilyStructure{
        dad: i32,
        mom: i32,
        child: i32,
    }

    let f1 = FamilyStructure {dad: 1, mom: 1, child: 2,};
    
    //Doing this will cause a compiler error. cause the data has been moved already;
    //let f2 = f1;
    //println!("{}", f1);
    
    //to avoid moving the value we must use the prefix '&'
    let f2 = &f1;

    println!("{:#?} {} {}", f2.dad, f2.mom, f2.child);
}