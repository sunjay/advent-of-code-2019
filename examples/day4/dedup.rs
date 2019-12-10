use std::iter::Peekable;

pub trait DedupCount: Iterator + Sized {
    fn dedup_count(self) -> DuplicateCounts<Self>;
}

impl<I> DedupCount for I
    where I: Iterator
{
    fn dedup_count(self) -> DuplicateCounts<Self> {
        DuplicateCounts::new(self)
    }
}

pub struct DuplicateCounts<I: Iterator> {
    iter: Peekable<I>,
}

impl<I> Clone for DuplicateCounts<I>
    where I: Iterator + Clone,
          <I as Iterator>::Item: Clone,
{
    fn clone(&self) -> Self {
        Self {
            iter: self.iter.clone(),
        }
    }
}

impl<I> DuplicateCounts<I>
    where I: Iterator
{
    fn new(iter: I) -> Self {
        Self {
            iter: iter.peekable(),
        }
    }
}

impl<T, I> Iterator for DuplicateCounts<I>
    where T: Eq,
          I: Iterator<Item=T>
{
    type Item = (T, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let digit = match self.iter.next() {
            Some(digit) => digit,
            None => return None,
        };

        let mut count = 1;
        while let Some(next_digit) = self.iter.peek() {
            if *next_digit != digit {
                break;
            }

            count += 1;
            self.iter.next();
        }

        Some((digit, count))
    }
}
