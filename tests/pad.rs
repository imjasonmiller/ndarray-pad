use ndarray::array;
use ndarray_pad::{PaddingExt, PaddingMode};

#[test]
fn test_with_zero_value() {
    #[rustfmt::skip]
    let result = array![
        [1.0, 2.0],
        [3.0, 4.0]
    ].pad(vec![[1, 1], [1, 1]], PaddingMode::Constant(0.0));

    assert_eq!(
        array![
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 2.0, 0.0],
            [0.0, 3.0, 4.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ],
        result
    );
}
