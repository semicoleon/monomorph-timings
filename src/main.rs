use seq_macro::seq; // seq-macro = "0.3.2"
use std::fmt::Debug;

fn action<T: Debug>(value: T) {
    #[cfg(feature = "polymorphic")]
    action_inner(&value);
    #[cfg(not(feature = "polymorphic"))]
    seq!(x in 0..1_000 {
        println!("{}: {value:?}", x);
    });
}

#[cfg(feature = "polymorphic")]
fn action_inner(value: &dyn Debug) {
    seq!(x in 0..1_000 {
        println!("{}: {value:?}", x);
    });
}

fn usize() {
    action(100usize);
}

fn str() {
    action("str");
}

fn string() {
    action("String".to_string());
}

fn vec_usize() {
    action(vec![1usize, 2, 3]);
}

fn main() {
    usize();
    str();
    string();
    vec_usize();
}
