fn main() {
    family_macros();
}

/*
 * Macro rules are a way to avoid code duplication 
 * by generating code at compile time.
 * 
 * 
 */

fn family_macros(){
    trait BitSet {
        fn clear(&mut self, index: usize);
        fn is_set(&self, index: usize) -> bool;
        fn set(&mut self, index: usize);
        fn toggle(&mut self, index: usize) {
        if self.is_set(index) {
        self.clear(index);
        } else {
        self.set(index);
        }
        }
    }

    macro_rules! int_bitset {
        ($ty: ty) => {
            impl BitSet for $ty {
                fn clear(&mut self, index: usize){
                    *self &= ! (1<<index)
                }

                fn is_set(&self, index: usize) -> bool{
                    (*self >> index) & 1 == 1
                }

                fn set(&mut self, index: usize){
                    *self |= 1 << index;
                }
            }
        }
    }

    int_bitset! (i32);
    int_bitset! (u8);
    int_bitset! (u64);
}