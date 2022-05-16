fn main() {
    family_methods()
}

/*
*Inside a struct we can have functions that are associated in two ways:
*Functions are associated to a type generally, while methods are associated to an instance of a type.
*are associated functions that are called on a particular instance of a type
*/

fn family_methods(){
    struct Family {
        dad: i32,
        mom: i32,
    }
{/*
  *For doing so, first we need to use the keywork impl
  *that basically is an interface for all associated 
  *functions and methods.
  */
}
    impl Family {

/*
*@func orgin 
*@func new_family
*Are associated functions. That's because they are associated with a type.
*In this case: Family.
*This functions are generally used like constructors in js.    
*/

        fn origin() -> Family {
            Family {
                dad: 0,
                mom: 0,
            }
        }

        fn new_family(dad: i32, mom: i32) -> Family {
            Family { dad, mom }
        }
/*
 *@func family_methods
 *is a method, not an associated function.
 *This is because it is associated with an instance of a type.
 *For this we use the instance &Self, that can also be declared as
 *self: &Self. in this case Self it's going to be associated with the method
 *we're implementing. If we were implementing a method called
 *Clock, Self would be associated with Clock.
 *In this case, self is associated with Family.
 */

        fn family_method(&self) {
            println!("dad: {}", self.dad);
            println!("mom: {}", self.mom);
        }
    }

/*
 * @variable no_family is associated with the initial values.
 * @variable family is associated with the family contructor, thus
 * it needs to pass
 * @param dad -> i32 
 * @param mom -> i32
 * to return the structure Family.
 */

    let no_family = Family::origin();
    let family = Family::new_family(1, 2);

    /*
     *@method family_methods
     *is a method and we are going to use this method
     *(pretty much like what we do in JS classes)
     *to return the values we've defined in the struct. 
     *but associated with the data we've defined in each
     *variable.
     */

    no_family.family_method();
    family.family_method();
    
}

/*
*no_family.family_methods()
*@return -> Family
*{dad: 0, mom: 0}
*
*family.family_methods()
* @return -> Family
* {dad: 1, mom: 2}
*/