use std;

pub fn pairwise<T>(container: &[T])
                   -> std::iter::Zip<std::slice::Iter<T>, std::iter::Skip<std::slice::Iter<T>>> {
    let first = container.iter();
    let second = container.iter().skip(1);
    first.zip(second)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pairwise() {
        let a = [1, 2, 3, 4, 5];
        assert_eq!(pairwise(&a).nth(0).unwrap(), (&1, &2));
        assert_eq!(pairwise(&a).last().unwrap(), (&4, &5));
        assert_eq!(pairwise(&a).len(), a.len() - 1);

        let a = [1, 2];
        assert_eq!(pairwise(&a).nth(0).unwrap(), (&1, &2));
        assert_eq!(pairwise(&a).last().unwrap(), (&1, &2));
        assert_eq!(pairwise(&a).len(), a.len() - 1);

        let a = [1];
        assert!(pairwise(&a).nth(0).is_none());
    }
}
