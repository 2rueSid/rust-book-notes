// mod sort;
pub mod search;
pub mod sort;
fn main() {
    let mut arr: [i32; 5] = [1, 3, 10, 30, 0];
    sort::bubble_sort(&mut arr);
    assert_eq!(arr, [0, 1, 3, 10, 30]);
    assert_eq!(search::linear(arr, 3), true);
    assert_eq!(search::linear(arr, 50), false);

    assert_eq!(
        search::binary_search_loop(&[0, 1, 3, 4, 10, 12, 13, 20, 30, 100], 3),
        true
    );
    assert_eq!(
        search::binary_search_loop(&[0, 1, 3, 4, 10, 12, 13, 20, 30, 100], 3200),
        false
    );
    assert_ne!(
        search::binary_search_loop(&[0, 1, 3, 4, 10, 12, 13, 20, 30, 100], 3200),
        true
    );

    assert_eq!(
        search::two_balls(&[false, false, false, true, true, true]),
        3
    )
}
