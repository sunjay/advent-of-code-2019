/// Extension trait that adds the `digits()` method to numbers
pub trait GetDigits {
    /// Yields each digit of a number using the given base.
    ///
    /// If the number is 0, this will not yield any digits.
    fn digits(self, base: u64) -> Digits;
}

impl GetDigits for u64 {
    fn digits(self, base: u64) -> Digits {
        Digits::new(self, base)
    }
}

#[derive(Debug, Clone)]
pub struct Digits {
    number: u64,
    base: u64,
    len: usize,
}

impl Digits {
    fn new(number: u64, base: u64) -> Self {
        let len = (number as f64).log10().floor() as usize + 1;

        Self {
            number,
            base,
            len,
        }
    }
}

impl Iterator for Digits {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let &mut Digits {number, base, len: _} = self;
        if number == 0 {
            return None;
        }

        let digit = number % base;
        self.number /= base;

        Some(digit)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl ExactSizeIterator for Digits {}
