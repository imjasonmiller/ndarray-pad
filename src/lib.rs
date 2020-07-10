//! The [`ndarray-pad`] crate provides padding for `ArrayBase` in [`ndarray`].
//!
//! This crate is based on [`ndarray-stats`]. It is inspired by and aims to achieve feature parity
//! with [`numpy`] (Python).
//!
//! Please feel free to contribute!
//!
//! [`ndarray`]: https://github.com/rust-ndarray/ndarray
//! [`ndarray-pad`]: https://github.com/imjasonmiller/ndarray-pad
//! [`ndarray-stats`]: https://github.com/rust-ndarray/ndarray-stats
//! [`numpy`]: https://numpy.org/doc/stable/reference/generated/numpy.pad.html
use ndarray::{Array, ArrayBase, Axis, Data, Dimension, Slice};
use num_traits::NumCast;

pub enum PaddingMode<T> {
    Constant(T),
    Edge,
}

/// An extension trait for `ArrayBase` providing padding
pub trait PaddingExt<A, D>
where
    D: Dimension,
{
    /// Return a padded array based on `self`
    fn pad(&self, pad_width: Vec<[usize; 2]>, pad_mode: PaddingMode<A>) -> Array<A, D>
    where
        A: Clone + NumCast,
        D: Dimension;
}

impl<A, S, D> PaddingExt<A, D> for ArrayBase<S, D>
where
    A: Clone + NumCast,
    S: Data<Elem = A>,
    D: Dimension,
{
    fn pad(&self, pad_width: Vec<[usize; 2]>, pad_mode: PaddingMode<A>) -> Array<A, D> {
        // Compute shape of final padded array
        let mut padded_shape = self.raw_dim();
        for (axis, (&axis_len, &[pad_lo, pad_hi])) in
            self.shape().iter().zip(&pad_width).enumerate()
        {
            padded_shape[axis] = axis_len + pad_lo + pad_hi;
        }

        match pad_mode {
            PaddingMode::Constant(value) => {
                let mut padded = Array::from_elem(padded_shape, value);

                // Select area of padded array that the original array will be copied into
                let mut original_area = padded.view_mut();

                for (axis, &[pad_lo, pad_hi]) in pad_width.iter().enumerate() {
                    original_area.slice_axis_inplace(
                        Axis(axis),
                        Slice::from(pad_lo as isize..-(pad_hi as isize)),
                    );
                }

                // Copy original array into padded array
                original_area.assign(self);

                return padded;
            }
            PaddingMode::Edge => {
                unimplemented!();
            }
        }
    }
}

