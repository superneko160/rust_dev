// use sorting::bubble_sort::bubble_sort;
// use sorting::insertion_sort::insertion_sort;
// use sorting::selection_sort::selection_sort;
// use sorting::bucket_sort::bucket_sort;
// use sorting::counting_sort::counting_sort;
// use sorting::merge_sort::merge_sort;
// use sorting::shell_sort::shell_sort;
// use sorting::heap_sort::heap_sort;
use sorting::quick_sort::quick_sort;

fn main() {
    let mut list = [2, 7, 3, 5, 1, 24, 31, 100, 11];
    // bubble_sort(&mut list, true);
    // insertion_sort(&mut list, true);
    // selection_sort(&mut list, true);
    // counting_sort(&mut list, 100);
    // merge_sort(&mut list);
    // shell_sort(&mut list);
    quick_sort(&mut list);

    // let list = [2, 7, 2, 5, 1, 11, 31, 100, 11];
    // let list: Vec<usize> = bucket_sort(&list);
    // let list = heap_sort(&list, true);

    for val in &list {
        print!("{} ", val);
    }

    println!();
}
