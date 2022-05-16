fn main() {
    family_struct()
}

/*
 * Structures work pretty much like interfaces in JS.
 * They can be created to make a more
 * visible sense in your code in general terms.
*/

fn family_struct() {
    struct Family {
        dad: String,
        mom: String,
    }

    let family_morisson = Family {
        dad: "John Morrison".to_string(),
        mom: "Jane Morrison".to_string(),
    };

    println!("Dad's Name: {} - Mom's name: {}", family_morisson.dad, family_morisson.mom);
    //println!("{:#?}", family_morisson);
}