fn main() {
    family_tuple();
    family_tuple_variables();
}

/*
*Tuples are basically arrays with a fixed size
*and predefined types.
*
*In rust tuples can be used both as a way to creating an array of values
*either a way to create variables associated with values.
*/

/* 1-
*Doing with this way, what we're doing is basically create an array and
*Associating values of predefined types to specific indexes. 
*/
fn family_tuple(){
    let family: (i32, i32, i32) = (1, 2, 3);
    println!("dad: {}, mom: {}, children: {}", family.0, family.1, family.2);
}

/* 2-
 * Doing it this way, what we're doing is basically creating variables with no
 * predefined types. And then associating it to different values.
 * That's a good way to create lots of variables at the same time.
 */

fn family_tuple_variables(){
    let (dad, mom, children) = (1, 2, 3);
    println!("dad: {}, mom: {}, children: {}", dad, mom, children);
}