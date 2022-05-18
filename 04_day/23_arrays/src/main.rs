fn main() {
    family_array();
}


/*
 * Arrays are a collection of elements.
 * Different from tuples, arrays can contain different types.
 * And they do not have a fixed amount of values.
 */

fn family_array(){
    let arr1 = [1, 2, 3, 4, 5];
    let arr2: [u8; 5] = [1, 2, 3, 4, 5];

    println!("{}", arr1[0]);
    println!("{}", arr2[0]);
}