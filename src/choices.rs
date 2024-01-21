use rand::Rng;
use std::iter::FromIterator;

const CHARSET: [char; 36] = [
    'q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p', 'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l',
    'z', 'x', 'c', 'v', 'b', 'n', 'm', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0',
];

pub fn choices<T: Copy, R: FromIterator<T>>(set: &[T], count: usize, length: usize) -> R {
    let mut rng = rand::thread_rng();

    (0..count)
        .map(|_| {
            let idx = rng.gen_range(0..length);
            set[idx] as T
        })
        .collect()
}
pub fn gen_strid(length: usize) -> String {
    choices(&CHARSET, length, 36)
}
