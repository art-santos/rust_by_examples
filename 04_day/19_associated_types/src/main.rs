fn main(){
    trait Graph {
        type N;
        type E;
    
        fn has_edge(&self, &Self::N, &Self::N) -> bool;
        fn edges(&self, &Self::N) -> Vec<Self::E>;
        
        fn distance<G: Graph>(graph: &G, start: &G::N, end: &G::N) -> u32 { ... }

    }
}