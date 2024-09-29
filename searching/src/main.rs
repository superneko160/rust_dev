// use searching::linear_search::linear_search;
use searching::binary_search::binary_search;

fn main() {
    let list = [25, 24, 32, 72, 100];
    let target = 32;
    // let index = linear_search(&target, &list);
    let index = binary_search(&target, &list);
    println!("{:?}", index);
}
