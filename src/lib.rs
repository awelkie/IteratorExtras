pub struct Stride<I> {
    iter: I,
    stride: usize,
}

impl<I> Iterator for Stride<I> where I: Iterator {
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        let ret = self.iter.next();
        if self.stride > 1 {
            self.iter.nth(self.stride - 2);
        }
        ret
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.stride > 0 {
            match self.iter.size_hint() {
                (lower, None) => (lower / self.stride, None),
                (lower, Some(upper)) => (lower / self.stride, Some(upper / self.stride))
            }
        } else {
            self.iter.size_hint()
        }
    }
}

pub struct MapPairs<B, I, F> where I: Iterator, F: Fn([I::Item; 2]) -> B {
    iter: I,
    f: F,
}

impl<B, I, F> Iterator for MapPairs<B, I, F> where I: Iterator , F: Fn([I::Item; 2]) -> B {
    type Item = B;

    #[inline]
    fn next(&mut self) -> Option<B> {
        let a = self.iter.next();
        let b = self.iter.next();
        match (a,b) {
            (Some(x), Some(y)) => Some((self.f)([x,y])),
            _ => None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

pub struct Scan1<B, I, F> where F: FnMut(&mut I::Item, I::Item) -> Option<B>, I: Iterator {
    iter: I,
    f: F,
    state: Option<I::Item>,
}

impl<B, I, F> Iterator for Scan1<B, I, F> where F: FnMut(&mut I::Item, I::Item) -> Option<B>,
                                                I: Iterator {
    type Item = B;

    #[inline]
    fn next(&mut self) -> Option<B> {

        // If the current state is None, then grab the first element
        if self.state.is_none() {
            self.state = self.iter.next();
        }

        match self.state {
            None => None,
            Some(_) => self.iter.next().and_then(|a| (self.f)(self.state.as_mut().unwrap(), a))
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.iter.size_hint();
        (0, upper) // can't know a lower bound, due to the scan function
    }
}

pub trait IteratorExtra: Iterator + Sized {

    /// This will traverse the iterator `stride` elements at a time.
    ///
    /// The first element in the input iterator will be returned, then `stride - 1`
    /// elements will be skipped. A stride of 0 is the same as a stride of 1 is the
    /// same as an unaltered iterator.
    ///
    /// # Example
    ///
    /// ```rust
    /// use IteratorExtras::IteratorExtra;
    /// let xs = vec![0u, 1, 2, 3, 4, 5];
    /// let strided: Vec<usize> = xs.into_iter().stride(3).collect();
    /// assert_eq!(strided, vec![0u, 3]);
    /// ```
    ///
    fn stride(self, stride: usize) -> Stride<Self> {
        Stride { iter: self, stride: stride }
    }

    /// This will take chunks of 2 elements in the iterator, and map a closure of two elements
    /// to each chunk.
    ///
    /// If there are an odd number of elements, the last element will be ignored.
    ///
    /// # Example
    ///
    /// ```rust
    /// use IteratorExtras::IteratorExtra;
    /// let xs = vec![0i, 1, 5, 8, 10];
    /// let pairwise_diffs: Vec<int> = xs.into_iter().map_pairs(|[l,r]| r - l).collect();
    /// assert_eq!(pairwise_diffs, vec![1i, 3]);
    /// ```
    fn map_pairs<B, F>(self, f: F) -> MapPairs<B, Self, F> where F: Fn([Self::Item; 2]) -> B {
        MapPairs { iter: self, f: f }
    }

    /// This is just like `scan`, but using the first element as the initial state variable
    ///
    /// # Example
    ///
    /// ```rust
    /// use IteratorExtras::IteratorExtra;
    /// let xs = vec![0i, 1, 3, 6, 10];
    /// let diffs: Vec<int> = xs.into_iter().scan1(|st, x| {
    ///     let diff = x - *st;
    ///     *st = x;
    ///     Some(diff)
    ///     }).collect();
    /// assert_eq!(diffs, vec![1i, 2, 3, 4]);
    /// ```
    fn scan1<B, F>(self, f: F) -> Scan1<B, Self, F> where F: FnMut(&mut Self::Item, Self::Item) -> Option<B> {
        Scan1 { iter: self, f: f, state: None }
    }
}

impl<I> IteratorExtra for I where I: Iterator { }
