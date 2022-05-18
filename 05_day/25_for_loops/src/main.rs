fn main() {
   family_loop();
}
/*
 * For loops are very common in basically every programming language
 * a for..loop consists of three parts:
 * a condition that need to be meet in order to continue the loop
 * or reached in order to stop the loop.
 * ??A data structure that will be iterated over (but not necessarily).
 * A return for the data structure iteration
 */

fn family_loop(){
    let mut array = [1, 2, 3, 4, 5];
    for element in array.iter_mut() {
        *element += 1;
    }
    println!("{:?}", array);

    //We can also use this to print all items we want

    for x in 0..1000001 {
        if x == 1000000 {
                println!("{}", x);
        }
    }
}