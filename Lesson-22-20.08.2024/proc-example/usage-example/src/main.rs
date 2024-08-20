use map_macro::map;

fn main() {
    let my_map = map! {
        "a": 1,
        "b": 2,
        "c": 1 + 2,
    };

    println!("{my_map:?}");
}
