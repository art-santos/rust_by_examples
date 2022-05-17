fn main() {
    family_trait();
}
/*
 *  Traits are a way to create and implement default methods
 * In js we would call them as the default modules that comes
 * with the language. For example: reduce, sort, toUpperCase.
 * Just like in js, you both create and edit the default methods
 * as you need.
 */


fn family_trait(){
//Firstly, we need to create the trait, thats going to contain the method
//that we want to implement.

//That's the structure that's going to be implemented by the methods.
    struct FamilyMember {
        name: String,
        age: u8,
    }
//That's the trait we're going to use.
//It can grow undefined containing as much modules as we want to
    trait FamilyMemberTrait {
        fn get_name(&self);
        fn get_age(&self);
    }

//That's the implementation of the trait. Creating methods that uses FamilyMember as a basis of its types.
    impl FamilyMemberTrait for FamilyMember {
        fn get_name(&self){
            println!("{}", self.name);
        }

        fn get_age(&self) {
            println!("{}", self.age);
        }
    }

//Here we're simply creating objects and associating them with some variables.
    let scott = FamilyMember {
        name: String::from("Scott"),
        age: 30,
    };

    let rex = FamilyMember {
        name: String::from("Rex"),
        age: 3,
    };

//Here, let's simply call the methods we've created.
//We could call .get_age, for example.

    //scott.get_age();
    scott.get_name();
    rex.get_name();
}