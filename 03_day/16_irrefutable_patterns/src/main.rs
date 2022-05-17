fn main() {
    irrefutable_family();
}
/*
 * Is this pattern about Jordan Peterson? I don't think so.
 * An irrefutable pattern when there's only one and always correct way
 * to match this. Sounds confusing? Let's see an example. 
 */


fn irrefutable_family(){
    let family = (String::from("Dad"), String::from("Mom"));
    let (dad, mom) = family;

    println!("{} {}", dad, mom);
}