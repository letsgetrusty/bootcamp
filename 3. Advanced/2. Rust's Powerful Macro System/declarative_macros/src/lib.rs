// macro_rules! name {
//     rule0 ;
//     rule1 ;
//     // …
//     ruleN ;
// }

// (matcher) => { expansion aka transcriber }

#[macro_export]
macro_rules! hello {
    () => {
        println!("Hello World!");
    };
}

#[macro_export]
macro_rules! map {
    // $ [identifier] : [fragment-specifier]
    ($key:ty, $val:ty) => {
        {
            let map: HashMap<$key, $val> = HashMap::new();
            map
        }
    };
    // $ ( ... ) sep rep
    ($($key:expr => $val:expr),*) => {
        {
            let mut map = HashMap::new();
            $( map.insert($key, $val); )*
            map
        }
    };
}