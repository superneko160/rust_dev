// use sorting::bubble_sort::bubble_sort;
// use sorting::insertion_sort::insertion_sort;
// use sorting::selection_sort::selection_sort;
// use sorting::bucket_sort::bucket_sort;
// use sorting::counting_sort::counting_sort;
use sorting::merge_sort::merge_sort;

fn main() {
    let mut list = [2, 7, 3, 5, 1, 24, 31, 100, 11];
    // bubble_sort(&mut list, true);
    // insertion_sort(&mut list, true);
    // selection_sort(&mut list, true);
    // counting_sort(&mut list, 100);
    merge_sort(&mut list);

    // let list = [2, 7, 2, 5, 1, 11, 31, 100, 11];
    // let list: Vec<usize> = bucket_sort(&list);

    for val in &list {
        print!("{} ", val);
    }

    println!();
}
