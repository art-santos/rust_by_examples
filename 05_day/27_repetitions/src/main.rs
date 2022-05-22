fn main() {
    multiple_patterns();
}

/*
 * In a macro pattern, it's also possible to match with an
 * unlimited number of different patterns by using the operators
 * + and *.
 * + - matches 1 time
 * * matches 0, 1 or more times (most used)
 */

fn multiple_patterns(){
    #[macro_export]
    macro_rules! hash {
        ($( $key:expr => $value:expr ),*) => {
        let mut hashmap = ::std::collections::HashMap::new();
        $(hashmap.insert($key, $value);)*
        hashmap
        };
    }

    let hashmap = {
        let mut hashmap = ::std::collections::HashMap::new();
        hashmap.insert("one", 1);
        hashmap.insert("two", 2);
        hashmap
    };
    
}