// バブルソート
pub fn bubble_sort<T: Ord>(list: &mut [T]) {

    if list.is_empty() {
        return;
    }

    let mut sorted = false;
    let mut n = list.len();

    while !sorted {
        sorted = true;
        for i in 0..n - 1 {
            if list[i] > list[i + 1] {
                list.swap(i, i + 1);
                sorted = false;
            }
        }
        n -= 1;
    }
}
