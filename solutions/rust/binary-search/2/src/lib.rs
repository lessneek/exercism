use std::cmp::Ordering::*;

pub fn find<A: AsRef<[T]>, T: Ord>(array: A, key: T) -> Option<usize> {
    let mut array = array.as_ref();
    let mut i = array.len() / 2;
    let mut i_offset = 0usize;

    loop {
        match key.cmp(array.get(i)?) {
            Equal => return Some(i + i_offset),
            Less if i == 0 => return None,
            Greater if i == array.len() - 1 => return None,
            Less => array = &array[..i],
            Greater => {
                array = &array[i..];
                i_offset += i;
            }
        }
        i = array.len() / 2;
    }
}
