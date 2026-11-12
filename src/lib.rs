#![no_std]
#![doc = include_str!("../README.md")]

use core::iter::Fuse;

pub trait OwnedSeqPatternHelper: IntoIterator + Sized
where Self::IntoIter: ExactSizeIterator + DoubleEndedIterator,
      for<'a> &'a Self: IntoIterator,
      for<'a> <&'a Self as IntoIterator>::IntoIter: ExactSizeIterator,
{
    fn heads_tails_with<const H: usize, const T: usize>(self) -> Result<
        ([Self::Item; H], Fuse<Self::IntoIter>, [Self::Item; T]),
        Self,
    > {
        let len = (&self).into_iter().len();

        if H + T > len {
            return Err(self);
        }

        let mut iter = self.into_iter().fuse();
        assert_eq!(iter.len(), len, "mismatch ref/owned iter len implement {len}/{}", iter.len());

        let a = core::array::from_fn(|_| iter.next().unwrap());
        let b = core::array::from_fn(|_| iter.next_back().unwrap());

        Ok((a, iter, b))
    }

    fn heads_tails<const H: usize, const T: usize>(self) -> Result<
        ([Self::Item; H], [Self::Item; T]),
        Self,
    > {
        self.heads_tails_with()
            .map(|(a, _, b)| (a, b))
    }

    fn heads_with<const H: usize>(self) -> Result<
        ([Self::Item; H], Fuse<Self::IntoIter>),
        Self,
    > {
        self.heads_tails_with::<H, 0>()
            .map(|(a, i, [])| (a, i))
    }

    fn tails_with<const T: usize>(self) -> Result<
        (Fuse<Self::IntoIter>, [Self::Item; T]),
        Self,
    > {
        self.heads_tails_with::<0, T>()
            .map(|([], i, b)| (i, b))
    }

    fn heads<const H: usize>(self) -> Result<
        [Self::Item; H],
        Self,
    > {
        self.heads_with().map(|(a, _)| a)
    }

    fn tails<const T: usize>(self) -> Result<
        [Self::Item; T],
        Self,
    > {
        self.tails_with().map(|(_, b)| b)
    }
}

impl<T: IntoIterator> OwnedSeqPatternHelper for T
where Self::IntoIter: ExactSizeIterator + DoubleEndedIterator,
      for<'a> &'a Self: IntoIterator,
      for<'a> <&'a Self as IntoIterator>::IntoIter: ExactSizeIterator,
{
}
