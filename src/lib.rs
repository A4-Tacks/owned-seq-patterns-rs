#![no_std]
#![doc = include_str!("../README.md")]
#![warn(clippy::pedantic)]

use core::iter::FusedIterator;

/// Helper extension functions.
///
/// See crate documents for detail.
///
/// # Panics
///
/// Panics if `self.iter().len() != self.into_iter().len()`.
/// Panics if incorrect [`ExactSizeIterator`] implement.
#[allow(clippy::type_complexity)]
pub trait OwnedSeqPatternHelper: IntoIterator + Sized
where Self::IntoIter: FusedIterator + ExactSizeIterator + DoubleEndedIterator,
      for<'a> &'a Self: IntoIterator,
      for<'a> <&'a Self as IntoIterator>::IntoIter: ExactSizeIterator,
{
    /// Returns `let Ok(([a, b], s, [d]))` similar to `let [a, b, s @ .., d]`.
    ///
    /// # Errors
    ///
    /// This function will return an error if `self.len() < H + T`.
    fn heads_tails_with<const H: usize, const T: usize>(self) -> Result<
        ([Self::Item; H], Self::IntoIter, [Self::Item; T]),
        Self,
    > {
        let len = (&self).into_iter().len();

        if H + T > len {
            return Err(self);
        }

        let mut iter = self.into_iter();
        assert_eq!(iter.len(), len, "mismatch ref/owned iter len implement {len}/{}", iter.len());

        let a = core::array::from_fn(|_| iter.next().unwrap());
        let b = core::array::from_fn(|_| iter.next_back().unwrap());

        Ok((a, iter, b))
    }

    /// Returns `let Ok(([a, b], [d]))` similar to `let [a, b, .., d]`.
    ///
    /// # Errors
    ///
    /// This function will return an error if `self.len() < H + T`.
    fn heads_tails<const H: usize, const T: usize>(self) -> Result<
        ([Self::Item; H], [Self::Item; T]),
        Self,
    > {
        self.heads_tails_with()
            .map(|(a, _, b)| (a, b))
    }

    /// Returns `let Ok(([a, b], s))` similar to `let [a, b, s@..]`.
    ///
    /// # Errors
    ///
    /// This function will return an error if `self.len() < H`.
    fn heads_with<const H: usize>(self) -> Result<
        ([Self::Item; H], Self::IntoIter),
        Self,
    > {
        self.heads_tails_with::<H, 0>()
            .map(|(a, i, [])| (a, i))
    }

    /// Returns `let Ok((s, [d, e]))` similar to `let [s@.., d, e]`.
    ///
    /// # Errors
    ///
    /// This function will return an error if `self.len() < T`.
    fn tails_with<const T: usize>(self) -> Result<
        (Self::IntoIter, [Self::Item; T]),
        Self,
    > {
        self.heads_tails_with::<0, T>()
            .map(|([], i, b)| (i, b))
    }

    /// Returns `let Ok([a, b])` similar to `let [a, b, ..]`.
    ///
    /// # Errors
    ///
    /// This function will return an error if `self.len() < H`.
    fn heads<const H: usize>(self) -> Result<
        [Self::Item; H],
        Self,
    > {
        self.heads_with().map(|(a, _)| a)
    }

    /// Returns `let Ok([d, e])` similar to `let [.., d, e]`.
    ///
    /// # Errors
    ///
    /// This function will return an error if `self.len() < T`.
    fn tails<const T: usize>(self) -> Result<
        [Self::Item; T],
        Self,
    > {
        self.tails_with().map(|(_, b)| b)
    }
}

impl<T: IntoIterator> OwnedSeqPatternHelper for T
where Self::IntoIter: FusedIterator + ExactSizeIterator + DoubleEndedIterator,
      for<'a> &'a Self: IntoIterator,
      for<'a> <&'a Self as IntoIterator>::IntoIter: ExactSizeIterator,
{
}

#[test]
#[allow(unused_variables)]
fn works() {
    let arr = [1, 2, 3];
    assert!(matches!(arr.heads(), Ok([a, b, c])));
    assert!(matches!(arr.heads(), Ok([a, b])));
    assert!(matches!(arr.heads(), Ok([a])));
    assert!(matches!(arr.heads(), Ok([])));

    assert!(! matches!(arr.heads(), Ok([a, b, c, d])));

    assert!(matches!(arr.heads::<4>(), Err([1, 2, 3])));
    assert!(matches!(arr.heads::<5>(), Err([1, 2, 3])));
}
