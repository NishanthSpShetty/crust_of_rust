#[derive(Debug)]
pub struct StrSplit<'a, 'b> {
    reminder: Option<&'a str>,
    delimiter: &'b str,
}

impl<'a, 'b> StrSplit<'a, 'b> {
    pub fn new(haystack: &'a str, delimiter: &'b str) -> Self {
        Self {
            reminder: Some(haystack),
            delimiter,
        }
    }
}

impl<'a, 'b> Iterator for StrSplit<'a, 'b> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        //find the next position of the delimiter
        if let Some(ref mut reminder) = self.reminder {
            if let Some(next_delim) = reminder.find(self.delimiter) {
                let until_delimiter = &reminder[..next_delim];
                println!("{}", until_delimiter);
                *reminder = &reminder[next_delim + self.delimiter.len()..];
                Some(until_delimiter)
            } else {
                println!("take else {:?}", self.reminder);

                self.reminder.take()
            }
        } else {
            println!("end else");
            None
        }
    }
}

#[test]
fn it_works_with_list() {
    let heystack = "a b c d e";
    let letters = StrSplit::new(heystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", "e"].into_iter()));
}

#[test]
fn it_works_with_trailing_delimiter() {
    let heystack = "a b c d ";
    let letters = StrSplit::new(heystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", ""].into_iter()));
}
