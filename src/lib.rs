pub struct Stride<A, I> {
    iterator: I,
    stride: uint,
}

impl<A, I: Iterator<A>> Iterator<A> for Stride<A, I> {
    #[inline]
    fn next(&mut self) -> Option<A> {
        let ret = self.iterator.next();
        if self.stride > 1 {
            self.iterator.nth(self.stride - 2);
        }
        ret
    }

    #[inline]
    fn size_hint(&self) -> (uint, Option<uint>) {
        if self.stride > 0 {
            match self.iterator.size_hint() {
                (lower, None) => (lower / self.stride, None),
                (lower, Some(upper)) => (lower / self.stride, Some(upper / self.stride))
            }
        } else {
            self.iterator.size_hint()
        }
    }
}

pub struct MapPairs<'a, A, B, It> {
    iterator: It,
    f: |[A, ..2]|: 'a -> B,
}

impl<'a, A, B, It: Iterator<A>> Iterator<B> for MapPairs<'a, A, B, It> {
    #[inline]
    fn next(&mut self) -> Option<B> {
        let a = self.iterator.next();
        let b = self.iterator.next();
        match (a,b) {
            (Some(x), Some(y)) => Some((self.f)([x,y])),
            _ => None
        }
    }

    #[inline]
    fn size_hint(&self) -> (uint, Option<uint>) {
        self.iterator.size_hint()
    }
}

pub trait IteratorExtra<A> {

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
    /// let strided: Vec<uint> = xs.move_iter().stride(3).collect();
    /// assert_eq!(strided, vec![0u, 3]);
    /// ```
    ///
    fn stride(self, stride: uint) -> Stride<A, Self> {
        Stride { iterator: self, stride: stride }
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
    /// let pairwise_diffs: Vec<int> = xs.move_iter().map_pairs(|[l,r]| r - l).collect();
    /// assert_eq!(pairwise_diffs, vec![1i, 3]);
    /// ```
    fn map_pairs<'r, B>(self, f: |[A, ..2]| : 'r -> B) -> MapPairs<'r, A, B, Self> {
        MapPairs { iterator: self, f: f }
    }
}

impl<A, I: Iterator<A>> IteratorExtra<A> for I { }
