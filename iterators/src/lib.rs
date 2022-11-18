#[allow(dead_code)]
fn flatten<O>(iter: O) -> Flatten<O> {
    Flatten::new(iter)
}

struct Flatten<O> {
    outer: O,
}

impl<O> Flatten<O> {
    fn new(iter: O) -> Self {
        Flatten { outer: iter }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.outer.next().and_then(|inner| inner.into_iter().next())
    }
}

impl<O> DoubleEndedIterator for Flatten<O>
where
    O: DoubleEndedIterator,
    O::Item: IntoIterator,
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.outer
            .next_back()
            .and_then(|inner| inner.into_iter().next_back())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0);
    }

    #[test]
    fn empty_wide() {
        assert_eq!(
            flatten(vec![Vec::<()>::new(), vec![], vec![]].into_iter()).count(),
            0
        );
    }

    #[test]
    fn one() {
        assert_eq!(flatten(std::iter::once(vec![0])).count(), 1);
    }

    #[test]
    fn two_1() {
        assert_eq!(flatten(std::iter::once(vec!["a", "b"])).count(), 1);
    }

    #[test]
    fn two() {
        assert_eq!(flatten(vec![vec!["a"], vec!["b"]].into_iter()).count(), 2);
    }

    #[test]
    fn reverse() {
        assert_eq!(
            flatten(std::iter::once(vec!["a", "b"]))
                .rev()
                .collect::<Vec<_>>(),
            vec!["b"]
        );
    }

    #[test]
    fn reverse_wide() {
        assert_eq!(
            flatten(vec![vec!["a"], vec!["b"]].into_iter())
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        );
    }
}
