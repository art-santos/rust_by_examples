fn main() {
    family_slice();
}

/*
 * If we want our array to not have a fixed size,
 *  than we need to use a slice type. A slice can be a 
 * part of a whole array.
 * 
 */

/*
basically, what this function does 
is creating implementing a array of a generic type as 
@param of the function.
@return, then, it's going to be the same type as the array.
notice that in this array we can only have a unique type.
*/

fn family_slice(){
    fn first<T>(slice: &[T]) -> &T {
        &slice[0]
    }

    let array = [10u8; 1000];
    //Doing this, we will define an array that starts at
    //index 2 of the array
    println!("{}", first(&array[2..]));
    //Doing this we're simply going to return an array.
    println!("{}", first(&array));
    //We can also do it this way
    println!("{}", first(&array[..]));
    //Or even this way
    println!("{}", first(&[1, 2, 3, 4, 5]));
}