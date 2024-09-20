use sorting::bubble_sort::bubble_sort;

fn main() {
    let mut list = [2, 7, 3, 5, 1, 24, 31, 100, 11];

    bubble_sort(&mut list);

    for val in &list {
        print!("{} ", val);
    }

    println!();
}
