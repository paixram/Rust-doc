
// usar crate si queremos usar otro modulo que no sea de lib y de main

mod hash {
    pub struct fhash<T, V> {
        _type: T,
        _value: V,
    }

    pub enum Hashtx {
        values {v: Vec<fhash<String, u32>>},
    }

    impl Hashtx {
        fn new() -> Hashtx {
            Hashtx::values{v: Vec::new()}
        }
    }
}
