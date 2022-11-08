use std::collections::HashMap;
use std::mem::size_of;

enum E {
    A(f64),
    B(HashMap<String, String>),
    C(Result<Vec<u8>, String>),
}


macro_rules! show_size {
    // 如果 show_size!(header) 則會進入下面的 macro
    (header) => {
        println!(
            "{:<24} {:>4}   {}  {}",
            "Type", "T", "Option<T>", "Result<T, io::Error>"
        );

        println!("{}", "-".repeat(64));
    };
    
    // 否則進入計算 容器 的記憶體計算
    ($t:ty) => {
        println!("{:<24} {:4}   {:8}  {:12}",
            stringify!($t),
            size_of::<$t>(),
            size_of::<Option<$t>>(),
            size_of::<Result<$t, std::io::Error>>()
        )
    };
}

fn main() {
    show_size!(header);
    show_size!(u8);
    show_size!(f64);
    show_size!(&u8);
    show_size!(Box<u8>);
    show_size!(&[u8]);
    show_size!(String);
    show_size!(Vec<u8>);
    show_size!(HashMap<String, String>);
    show_size!(E);
}
