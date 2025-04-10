//! vector.rs
//! 
//! vector userops

use crate::backend;
use super::*;

pub(super) fn _vfp_expand_immediate(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _simd_expand_immediate(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_absolute_difference_and_accumulate(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_absolute_difference(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _float_vector_absolute_difference(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_absolute(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _float_vector_absolute(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _float_compare_ge(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _float_compare_gt(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_add(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_sub(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _float_vector_add(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_pairwise_add(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_pairwise_min(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_pairwise_max(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _float_vector_pairwise_add(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _float_vector_pairwise_min(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _float_vector_pairwise_max(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_pairwise_add_long(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_pairwise_add_accumulate_long(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_add_return_high(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_bitwise_insert_if_false(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_bitwise_insert_if_true(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_bitwise_select(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_compare_equal(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _float_vector_compare_equal(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_compare_greater_than_or_equal(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _float_vector_compare_greater_than_or_equal(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_compare_greater_than(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _float_vector_compare_greater_than(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_count_leading_sign_bits(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_count_leading_zeros(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_count_one_bits(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_float_to_signed(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_float_to_unsigned(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_signed_to_float(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_unsigned_to_float(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_float_to_signed_fixed(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_float_to_unsigned_fixed(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_signed_fixed_to_float(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_unsigned_fixed_to_float(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_float_double_to_single(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_float_single_to_double(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_float_single_to_half(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_float_half_to_single(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_halving_add(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_halving_subtract(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_round_halving_add(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_round_add_and_narrow(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_min(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_max(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _float_vector_min(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _float_vector_max(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_multiply_accumulate(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_multiply_subtract(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_multiply_subtract_long(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_double_multiply_high_half(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_round_double_multiply_high_half(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_double_multiply_long(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_double_multiply_accumulate_long(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_double_multiply_subtract_long(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _float_vector_multiply_accumulate(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _float_vector_multiply_subtract(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_get_element(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_set_element(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_copy_long(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_copy_narrow(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _float_vector_mult(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_multiply(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _polynomial_multiply(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _float_vector_neg(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _sat_q(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _signed_sat_q(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_reciprocal_estimate(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_reciprocal_step(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vrev(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_shift_left(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_round_shift_left(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_shift_right(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_shift_left_insert(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_shift_right_insert(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_shift_right_narrow(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_shift_right_accumulate(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_round_shift_right(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_round_shift_right_narrow(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_round_shift_right_accumulate(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_shift_long_left(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_shift_narrow_right(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_reciprocal_square_root_estimate(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_reciprocal_square_root_step(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _float_vector_sub(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_sub_and_narrow(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_table_lookup(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_test(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_transpose(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_unzip(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}
pub(super) fn _vector_zip(this: &mut Backend,
    index: usize,
    inputs: &[VarnodeData],
    output: Option<&VarnodeData>,
) -> Result<Option<Location>, backend::Error> {
    unimplemented!("unsupported userop: {}", _lookup_userop(index).name)
}