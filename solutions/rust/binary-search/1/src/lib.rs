use std::cmp::Ordering::*;

pub fn find<A: AsRef<[T]>, T: Ord>(array: A, key: T) -> Option<usize> {
    let mut array = array.as_ref();
    if array.is_empty() {
        return None;
    }

    let mut i = array.len() / 2;
    let mut i_offset = 0usize;

    loop {
        match key.cmp(&array[i]) {
            Equal => return Some(i + i_offset),
            Less if i == 0 => return None,
            Greater if i == array.len() - 1 => return None,
            Less => (array, _) = array.split_at(i),
            Greater => {
                (_, array) = array.split_at(i);
                i_offset += i;
            }
        }
        i = array.len() / 2;
    }
}

#[test]
#[ignore]
fn identifies_that_a_value_is_included_in_the_array() {
    assert_eq!(find([1, 3, 4, 6, 7, 8, 9, 11], 8), Some(5));
}
