use std;

pub fn pairwise<T>(container: &[T])
                   -> std::iter::Zip<std::slice::Iter<T>, std::iter::Skip<std::slice::Iter<T>>> {
    let first = container.iter();
    let second = container.iter().skip(1);
    first.zip(second)
}
