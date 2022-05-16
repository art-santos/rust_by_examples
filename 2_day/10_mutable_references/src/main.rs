fn main() {
    struct FamilyStructure{
        dad: i32,
        mom: i32,
        child: i32,
    }
    //The variable needs to be mutables
    let mut f1 = FamilyStructure {dad: 1, mom: 1, child: 2,};

    //if we're going to create a mutable @param
    //we need to use &mut and type the variable as mutable
    fn increment_family(f: &mut FamilyStructure){
        f.dad += 1;
        f.mom += 1;
        f.child += 1;
        
        println!("{} {} {}", f.dad, f.mom, f.child);
    }
    //We also need to be certain that we're passing a mutable variable as arg;
    increment_family(&mut f1);
}





