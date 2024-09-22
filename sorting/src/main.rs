// use sorting::bubble_sort::bubble_sort;
// use sorting::insertion_sort::insertion_sort;
use sorting::selection_sort::selection_sort;

fn main() {
    let mut list = [2, 7, 3, 5, 1, 24, 31, 100, 11];

    // bubble_sort(&mut list, true);
    // insertion_sort(&mut list, true);
    selection_sort(&mut list, true);

    for val in &list {
        print!("{} ", val);
    }

    println!();
}
