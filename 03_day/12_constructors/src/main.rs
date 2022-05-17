fn main() {
    family_constructor();
}

/*
 * Rust does not provides us with contructors.
 * The most common way to do something similar 
 * is creating a method and associating it with the 
 * "new" keyword. Then, adding the syntatic sugar of
 * self: &Self to the method.
 * 
 * The difference between this and a noraml method is that
 * the normal method does not receive &self as a valid
 * parameter for referencing the impl.
 */

fn family_constructor() {
    struct FamilyConstructor {
        dad: i32,
        mom: i32,
        children: i32,
    }

    impl FamilyConstructor{
        fn new(dad: i32, mom: i32, children: i32) -> Self {
            Self { dad: dad, mom: mom, children: children }
        }
    }

    let family = FamilyConstructor::new(1, 2, 3);
    println!("dad: {}, mom: {}, children: {}", family.dad, family.mom, family.children);
}