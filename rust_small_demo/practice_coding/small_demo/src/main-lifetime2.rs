#[derive(Debug)]
struct StrSplit<'a, D> {
    remainder: Option<&'a str>,
    delimiter: D,
}

impl<'a, D> StrSplit<'a, D> {
    fn new(stack: &'a str, delimiter: D) -> Self {
        Self {
            remainder: Some(stack),
            delimiter,
        }
    }
}

trait Delimiter {
    fn find_next(&self, stack: &str) -> Option<(usize, usize)>;
}

impl<'a, D> Iterator for StrSplit<'a, D>
where
    D: Delimiter,
{
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(remainder) = self.remainder {
            if let Some((delimi_start, delimi_end)) = self.delimiter.find_next(remainder) {
                let until_delimiter = &remainder[..delimi_start];
                self.remainder = remainder.get(delimi_end..);
                Some(until_delimiter)
            } else {
                self.remainder.take()
            }
        } else {
            None
        }
    }
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|idx| (idx, idx + self.len()))
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, start + self.len_utf8()))
    }
}

fn until_char(s: &str, c: char) -> &str {
    StrSplit::new(s, c)
        .next()
        .expect("StrSplit always gives at least one result")
}

fn main() {
    let stack = "a b c d e";
    let letters = StrSplit::new(stack, " ");
    // let letters = StrSplit::new(stack, " ").collect::<Vec<&str>>();
    // assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
    assert!(letters.eq(vec!["a", "b", "c", "d", "e"].into_iter()));

    let stack = "a b c d ";
    let letters: Vec<_> = StrSplit::new(stack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);

    assert_eq!(until_char("hello world", 'o'), "hell");
}
