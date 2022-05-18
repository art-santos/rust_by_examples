use std::ops::Sub;


fn main() {
    family_default_methods_traits();
}


/*
 *Traits can also contain default methods. 
 *What is helpfull, since we're going to save the developer
 *To write a few lines of code.
 *
 */


/*
 * Specifically for this file, i've imported the std::ops::Sub trait.
 * That's used for making substraction operations. 
 */
fn subtract<T:Sub<Output = T> + Copy> (num1: T, num2: T) -> T {
    return num1 - num2;
}

fn family_default_methods_traits(){
/**
 * Lets create a trait again. This time, the diference is
 * that we're going to implement default methods, and not just methods
 * Basically, they're going to work the same way, no matter in
 * what part of the code you call each one of them.
 */

    trait FamilySize{
        fn get_size(&self) -> usize;
        fn set_size(&mut self, param: usize) -> usize;
        fn is_set(&self, index: usize) -> bool;
        fn remove_size(&mut self, param: usize) -> usize;

        fn toggle_size(&mut self, param: usize) -> usize {
            if self.get_size() == 0 {
                self.set_size(param)
            } else {
                self.remove_size(param)
            }
        }
    }

    /**
     * Now, we're going to create a struct that implements the trait.
     * 
     * As you can see, the default method we've creted doesn't need to be 
     * implemented for us to work with it.
     */

    impl FamilySize for usize {
        fn get_size(&self) -> usize {
            *self
        }
        fn set_size(&mut self, param: usize) -> usize {
            *self = param;
            *self
        }
        fn is_set(&self, index: usize) -> bool {
            *self == index
        }
        fn remove_size(&mut self, param: usize) -> usize {
            *self = subtract(*self, param);
            *self
        }
    }

    /**
     * Now, let's check if every function is working, specially then ones
     * whose function is to associate new numbers to a ready done mutable
     * variable.
     */

    let mut family_size = 0;

    println!("{}", family_size.get_size());
    println!("{}", family_size.set_size(5));
    println!("{}", family_size.is_set(5));
    println!("{}", family_size.remove_size(3));
    println!("{}", family_size.toggle_size(2));
}