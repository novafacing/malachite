use malachite_base::limbs::limbs_test_zero;
use malachite_base::misc::Max;
use malachite_base::num::{
    DivisibleByPowerOfTwo, Parity, PrimitiveInteger, SplitInHalf, WrappingAddAssign,
    WrappingSubAssign,
};
use natural::arithmetic::add::{
    limbs_add_same_length_to_out, limbs_slice_add_greater_in_place_left,
    limbs_slice_add_same_length_in_place_left,
};
use natural::arithmetic::add_limb::{limbs_add_limb_to_out, limbs_slice_add_limb_in_place};
use natural::arithmetic::add_mul_limb::mpn_addmul_1;
use natural::arithmetic::div_exact_limb::{
    limbs_div_exact_3_in_place, limbs_div_exact_limb_in_place,
};
use natural::arithmetic::mul::poly_eval::do_mpn_addlsh_n;
use natural::arithmetic::shl_u::limbs_shl_to_out;
use natural::arithmetic::shr_u::limbs_slice_shr_in_place;
use natural::arithmetic::sub::{
    _limbs_sub_same_length_in_place_with_overlap, limbs_sub_in_place_left,
    limbs_sub_same_length_in_place_left, limbs_sub_same_length_in_place_right,
    limbs_sub_same_length_to_out,
};
use natural::arithmetic::sub_limb::limbs_sub_limb_in_place;
use natural::arithmetic::sub_mul_limb::mpn_submul_1;
use platform::{DoubleLimb, Limb};
use std::mem::swap;

/// This is mpn_toom_interpolate_5pts in mpn/generic/toom_interpolate_5pts.c.
#[allow(clippy::cyclomatic_complexity)]
pub(crate) fn _limbs_mul_toom_interpolate_5_points(
    c: &mut [Limb],
    v_2: &mut [Limb],
    v_neg_1: &mut [Limb],
    k: usize,
    two_r: usize,
    v_neg_1_neg: bool,
    mut v_inf_0: Limb,
) {
    let two_k = k + k;
    let two_k_plus_1 = two_k + 1;
    let four_k_plus_1 = two_k_plus_1 + two_k;
    assert_eq!(v_neg_1.len(), two_k_plus_1);
    assert!(two_r <= two_k);
    let v_2 = &mut v_2[..two_k_plus_1];
    {
        let v_1 = &c[two_k..four_k_plus_1]; // v_1 length: 2 * k + 1

        // (1) v_2 <- v_2 - v_neg_1 < v_2 + |v_neg_1|,            (16 8 4 2 1) - (1 -1 1 -1  1) =
        // thus 0 <= v_2 < 50 * B ^ (2 * k) < 2 ^ 6 * B ^ (2 * k) (15 9 3  3  0)
        //
        if v_neg_1_neg {
            assert!(!limbs_slice_add_same_length_in_place_left(v_2, v_neg_1));
        } else {
            assert!(!limbs_sub_same_length_in_place_left(v_2, v_neg_1));
        }

        // {c,2k} {c + 2k,2k + 1} {c + 4k + 1,2r - 1} {t,2k + 1} {t + 2k + 1,2k + 1} {t + 4k + 2,2r}
        //   v0        v_1             hi(v_inf)      |v_neg_1|     v_2-v_neg_1          EMPTY
        limbs_div_exact_3_in_place(v_2); // v_2 <- v_2 / 3
                                         // (5 3 1 1 0)

        // {c,2k} {c + 2k,2k + 1} {c + 4k + 1,2r - 1} {t,2k + 1} {t + 2k + 1,2k + 1} {t + 4k + 2,2r}
        //   v0       v_1             hi(v_inf)        |v_neg_1|    (v_2-v_neg_1)/3       EMPTY
        //
        // (2) v_neg_1 <- tm1 := (v_1 - v_neg_1) / 2  [(1 1 1 1 1) - (1 -1 1 -1 1)] / 2 =
        // tm1 >= 0                                    (0  1 0  1 0)
        // No carry comes out from {v_1, two_k_plus_1} +/- {v_neg_1, two_k_plus_1},
        // and the division by two is exact.
        // If v_neg_1_neg the sign of v_neg_1 is negative
        if v_neg_1_neg {
            assert!(!limbs_slice_add_same_length_in_place_left(v_neg_1, v_1));
        } else {
            assert!(!limbs_sub_same_length_in_place_right(v_1, v_neg_1));
        }
        assert_eq!(limbs_slice_shr_in_place(v_neg_1, 1), 0);

        // {c,2k} {c + 2k,2k + 1} {c + 4k + 1,2r - 1} {t,2k + 1} {t + 2k + 1,2k + 1} {t + 4k + 2,2r}
        //   v0       v_1             hi(v_inf)          tm1       (v_2-v_neg_1)/3        EMPTY
        //
        // (3) v_1 <- t1 := v_1 - v0  (1 1 1 1 1) - (0 0 0 0 1) = (1 1 1 1 0)
        // t1 >= 0
    }
    {
        let (c_lo, v_1) = c.split_at_mut(two_k);
        if limbs_sub_same_length_in_place_left(&mut v_1[..two_k], c_lo) {
            v_1[two_k].wrapping_sub_assign(1);
        }
        let v1 = &mut v_1[..two_k_plus_1];
        // {c,2k} {c + 2k,2k + 1} {c + 4k + 1,2r - 1} {t,2k + 1} {t + 2k + 1,2k + 1} {t + 4k + 2,2r}
        //   v0       v_1-v0           hi(v_inf)          tm1      (v_2-v_neg_1)/3        EMPTY
        //
        // (4) v_2 <- t2 := ((v_2 - v_neg_1) / 3 - t1) / 2 = (v_2 - v_neg_1 - 3 * t1) / 6
        // t2 >= 0                  [(5 3 1 1 0) - (1 1 1 1 0)]/2 = (2 1 0 0 0)
        //
        assert!(!limbs_sub_same_length_in_place_left(v_2, v1));
        assert_eq!(limbs_slice_shr_in_place(v_2, 1), 0);
        // {c,2k} {c + 2k,2k + 1} {c + 4k + 1,2r - 1} {t,2k + 1} {t + 2k + 1,2k + 1} {t + 4k + 2,2r}
        //   v0      v_1 - v0        hi(v_inf)          tm1    (v_2 - v_neg_1 - 3t1) / 6    EMPTY
        //
        // (5) v_1 <- t1 - tm1           (1 1 1 1 0) - (0 1 0 1 0) = (1 0 1 0 0)
        // result is v_1 >= 0
        //
        assert!(!limbs_sub_same_length_in_place_left(v1, v_neg_1));
    }

    let saved;
    // We do not need to read the value in v_neg_1, so we add it in {c + k, ..}
    {
        let (c_lo, c_hi) = c.split_at_mut(3 * k + 1);
        if limbs_slice_add_same_length_in_place_left(&mut c_lo[k..], v_neg_1) {
            // 2 * n - (3 * k + 1) = 2 * r + k - 1
            // Memory allocated for v_neg_1 is now free, it can be recycled
            assert!(!limbs_slice_add_limb_in_place(
                &mut c_hi[..two_r + k - 1],
                1
            ));
        }
        let v_inf = &mut c_hi[k - 1..two_r + k - 1];
        // (6) v_2 <- v_2 - 2 * v_inf, (2 1 0 0 0) - 2 * (1 0 0 0 0) = (0 1 0 0 0)
        // result is v_2 >= 0
        saved = v_inf[0]; // Remember v1's highest byte (will be overwritten).
        v_inf[0] = v_inf_0; // Set the right value for v_inf_0

        // Overwrite unused v_neg_1
        let mut carry = limbs_shl_to_out(v_neg_1, &v_inf[..two_r], 1);
        if limbs_sub_same_length_in_place_left(&mut v_2[..two_r], &v_neg_1[..two_r]) {
            carry += 1;
        }
        assert!(!limbs_sub_limb_in_place(&mut v_2[two_r..], carry));
    }
    //  Current matrix is
    //  [1 0 0 0 0; v_inf
    //   0 1 0 0 0; v_2
    //   1 0 1 0 0; v1
    //   0 1 0 1 0; v_neg_1
    //   0 0 0 0 1] v0
    //  Some values already are in-place (we added v_neg_1 in the correct position)
    //  | v_inf|  v1 |  v0 |
    //       | v_neg_1 |
    //  One still is in a separated area
    // | +v_2 |
    //  We have to compute v1-=v_inf; v_neg_1 -= v_2,
    //    |-v_inf|
    //       | -v_2 |
    //  Carefully reordering operations we can avoid to compute twice the sum
    //  of the high half of v_2 plus the low half of v_inf.
    //
    // Add the high half of t2 in {v_inf}
    if two_r > k + 1 {
        // This is the expected flow
        let (c_lo, c_hi) = c[4 * k..].split_at_mut(k + 1);
        if limbs_slice_add_same_length_in_place_left(c_lo, &v_2[k..]) {
            // 2n-(5k+1) = 2r-k-1
            assert!(!limbs_slice_add_limb_in_place(
                &mut c_hi[..two_r - k - 1],
                1
            ));
        }
    } else {
        // triggered only by very unbalanced cases like (k+k+(k-2))x(k+k+1), should be handled by
        // toom32
        // two_r < k + 1 so k + two_r < two_k, the size of v_2
        assert!(!limbs_slice_add_same_length_in_place_left(
            &mut c[4 * k..4 * k + two_r],
            &v_2[k..k + two_r]
        ));
    }
    let carry;
    {
        let (v_1, v_inf) = c[2 * k..].split_at_mut(2 * k);
        // (7) v_1 <- v_1 - v_inf,       (1 0 1 0 0) - (1 0 0 0 0) = (0 0 1 0 0)
        // result is >= 0
        // Side effect: we also subtracted (high half) v_neg_1 -= v_2
        // v_inf is at most two_r long.
        carry = limbs_sub_same_length_in_place_left(&mut v_1[..two_r], &v_inf[..two_r]);
        v_inf_0 = v_inf[0]; // Save again the right value for v_inf_0
        v_inf[0] = saved;
    }
    {
        let (c1, v1) = c[k..].split_at_mut(k);
        let v1 = &mut v1[..two_k_plus_1];
        if carry {
            assert!(!limbs_sub_limb_in_place(&mut v1[two_r..], 1)); // Treat the last bytes.
        }

        // (8) v_neg_1 <- v_neg_1 - v_2 (0 1 0 1 0) - (0 1 0 0 0) = (0 0 0 1 0)
        // Operate only on the low half.
        //
        if limbs_sub_same_length_in_place_left(c1, &v_2[..k]) {
            assert!(!limbs_sub_limb_in_place(v1, 1));
        }
    }
    let (c3, v_inf) = c[3 * k..].split_at_mut(k);
    // Beginning the final phase
    // Most of the recomposition was done
    // add t2 in {c + 3 * k, ...}, but only the low half
    if limbs_slice_add_same_length_in_place_left(c3, &v_2[..k]) {
        v_inf[0].wrapping_add_assign(1);
        assert!(v_inf[0] >= 1); // No carry
    }

    // Add v_inf_0, propagate carry.
    assert!(!limbs_slice_add_limb_in_place(&mut v_inf[..two_r], v_inf_0));
}

/// Interpolation for Toom-3.5, using the evaluation points infinity, 1, -1, 2, -2. More precisely,
/// we want to compute f(2 ^ (GMP_NUMB_BITS * n)) for a polynomial f of degree 5, given the six
/// values
///
/// w5 = f(0),
/// w4 = f(-1),
/// w3 = f(1)
/// w2 = f(-2),
/// w1 = f(2),
/// w0 = limit at infinity of f(x) / x^5,
///
/// The result is stored in {out, 5 * n + n_high}. At entry, w5 is stored at
/// {out, 2 * n}, w3 is stored at {out + 2 * n, 2 * n + 1}, and w0 is stored at
/// {out + 5 * n, n_high}. The other values are 2 * n + 1 limbs each (with most significant
/// limbs small). f(-1) and f(-2) may be negative; signs are passed in. All intermediate results are
/// positive. Inputs are destroyed.
///
/// Interpolation sequence was taken from the paper: "Integer and Polynomial Multiplication: Towards
/// Optimal Toom-Cook Matrices". Some slight variations were introduced: adaptation to "gmp
/// instruction set", and a final saving of an operation by interlacing interpolation and
/// recomposition phases.
///
/// This is mpn_toom_interpolate_6pts from mpn/generic/mpn_toom_interpolate_6pts.c, but the argument
/// w0n == `n_high` is moved to immediately after `n`.
#[allow(clippy::cyclomatic_complexity, clippy::too_many_arguments)]
pub(crate) fn _limbs_mul_toom_interpolate_6_points(
    out: &mut [Limb],
    n: usize,
    n_high: usize,
    w4_neg: bool,
    w4: &mut [Limb],
    w2_neg: bool,
    w2: &mut [Limb],
    w1: &mut [Limb],
) {
    assert_ne!(n, 0);
    assert!(2 * n >= n_high && n_high != 0);
    let limit = 2 * n + 1;
    {
        let (w5, w3) = out.split_at_mut(2 * n); // w5 length: 2 * n

        // Interpolate with sequence:
        // w2 = (w1 - w2) >> 2
        // w1 = (w1 - w5) >> 1
        // w1 = (w1 - w2) >> 1
        // w4 = (w3 - w4) >> 1
        // w2 = (w2 - w4) / 3
        // w3 =  w3 - w4 - w5
        // w1 = (w1 - w3) / 3
        //
        // Last steps are mixed with recomposition:
        // w2 = w2 - w0 << 2
        // w4 = w4 - w2
        // w3 = w3 - w1
        // w2 = w2 - w0
        //
        // w2 = (w1 - w2) >> 2
        if w2_neg {
            limbs_slice_add_same_length_in_place_left(&mut w2[..limit], &w1[..limit]);
        } else {
            limbs_sub_same_length_in_place_right(&w1[..limit], &mut w2[..limit]);
        }
        limbs_slice_shr_in_place(&mut w2[..limit], 2);

        // w1 = (w1 - w5) >> 1
        if limbs_sub_same_length_in_place_left(&mut w1[..2 * n], w5) {
            w1[2 * n].wrapping_sub_assign(1);
        }
        limbs_slice_shr_in_place(&mut w1[..limit], 1);

        // w1 = (w1 - w2) >> 1
        limbs_sub_same_length_in_place_left(&mut w1[..limit], &w2[..limit]);
        limbs_slice_shr_in_place(&mut w1[..limit], 1);

        // w4 = (w3 - w4) >> 1
        if w4_neg {
            limbs_slice_add_same_length_in_place_left(&mut w4[..limit], &w3[..limit]);
            limbs_slice_shr_in_place(&mut w4[..limit], 1);
        } else {
            limbs_sub_same_length_in_place_right(&w3[..limit], &mut w4[..limit]);
            limbs_slice_shr_in_place(&mut w4[..limit], 1);
        }

        // w2 = (w2 - w4) / 3
        limbs_sub_same_length_in_place_left(&mut w2[..limit], &w4[..limit]);
        limbs_div_exact_3_in_place(&mut w2[..limit]);

        // w3 = w3 - w4 - w5
        limbs_sub_same_length_in_place_left(&mut w3[..limit], &w4[..limit]);
        if limbs_sub_same_length_in_place_left(&mut w3[..2 * n], w5) {
            w3[2 * n].wrapping_sub_assign(1);
        }

        // w1 = (w1 - w3) / 3
        limbs_sub_same_length_in_place_left(&mut w1[..limit], &w3[..limit]);
        limbs_div_exact_3_in_place(&mut w1[..limit]);
    }
    // [1 0 0 0 0 0;
    //  0 1 0 0 0 0;
    //  1 0 1 0 0 0;
    //  0 1 0 1 0 0;
    //  1 0 1 0 1 0;
    //  0 0 0 0 0 1]
    //
    // out[] prior to operations:
    //  |_H w0__|_L w0__|______||_H w3__|_L w3__|_H w5__|_L w5__|
    //
    // summation scheme for remaining operations:
    //  |______________5|n_____4|n_____3|n_____2|n______|n______| out
    //  |_H w0__|_L w0__|______||_H w3__|_L w3__|_H w5__|_L w5__|
    //                 || H w4  | L w4  |
    //         || H w2  | L w2  |
    //     || H w1  | L w1  |
    //             ||-H w1  |-L w1  |
    //          |-H w0  |-L w0 ||-H w2  |-L w2  |
    //
    if limbs_slice_add_same_length_in_place_left(&mut out[n..=3 * n], &w4[..limit]) {
        assert!(!limbs_slice_add_limb_in_place(
            &mut out[3 * n + 1..=4 * n],
            1
        ));
    }

    // w2 -= w0 << 2
    // {w4, 2 * n + 1} is now free and can be overwritten.
    let mut carry = limbs_shl_to_out(w4, &out[5 * n..5 * n + n_high], 2);
    if limbs_sub_same_length_in_place_left(&mut w2[..n_high], &w4[..n_high]) {
        carry += 1;
    }
    assert!(!limbs_sub_limb_in_place(&mut w2[n_high..limit], carry));

    // w4L = w4L - w2L
    if limbs_sub_same_length_in_place_left(&mut out[n..2 * n], &w2[..n]) {
        assert!(!limbs_sub_limb_in_place(&mut out[2 * n..2 * n + limit], 1));
    }

    let carry = if limbs_slice_add_same_length_in_place_left(&mut out[3 * n..4 * n], &w2[..n]) {
        1
    } else {
        0
    };
    // w3H = w3H + w2L
    let special_carry_1 = out[4 * n] + carry;
    // w1L + w2H
    let mut carry = w2[2 * n];
    if limbs_add_same_length_to_out(&mut out[4 * n..], &w1[..n], &w2[n..2 * n]) {
        carry += 1;
    }
    assert!(!limbs_slice_add_limb_in_place(&mut w1[n..limit], carry));
    // w0 = w0 + w1H
    let mut special_carry_2 = 0;
    if n_high > n {
        special_carry_2 = w1[2 * n];
        if limbs_slice_add_same_length_in_place_left(&mut out[5 * n..6 * n], &w1[n..2 * n]) {
            special_carry_2.wrapping_add_assign(1);
        }
    } else if limbs_slice_add_same_length_in_place_left(
        &mut out[5 * n..5 * n + n_high],
        &w1[n..n + n_high],
    ) {
        special_carry_2 = 1;
    }

    // summation scheme for the next operation:
    //  |...____5|n_____4|n_____3|n_____2|n______|n______| out
    //  |...w0___|_w1_w2_|_H w3__|_L w3__|_H w5__|_L w5__|
    //          ...-w0___|-w1_w2 |
    //
    // if (LIKELY(n_high > n)) the two operands below DO overlap!
    let carry =
        _limbs_sub_same_length_in_place_with_overlap(&mut out[2 * n..5 * n + n_high], 2 * n);

    // embankment is a "dirty trick" to avoid carry/borrow propagation beyond allocated memory
    let embankment;
    {
        let out_high = &mut out[5 * n + n_high - 1];
        embankment = out_high.wrapping_sub(1);
        *out_high = 1;
    }
    if n_high > n {
        if special_carry_1 > special_carry_2 {
            assert!(!limbs_slice_add_limb_in_place(
                &mut out[4 * n..5 * n + n_high],
                special_carry_1 - special_carry_2
            ));
        } else {
            assert!(!limbs_sub_limb_in_place(
                &mut out[4 * n..5 * n + n_high],
                special_carry_2 - special_carry_1
            ));
        }
        if carry {
            assert!(!limbs_sub_limb_in_place(
                &mut out[3 * n + n_high..5 * n + n_high],
                1
            ));
        }
        assert!(!limbs_slice_add_limb_in_place(
            &mut out[6 * n..5 * n + n_high],
            special_carry_2
        ));
    } else {
        assert!(!limbs_slice_add_limb_in_place(
            &mut out[4 * n..5 * n + n_high],
            special_carry_1
        ));
        if carry {
            special_carry_2.wrapping_add_assign(1);
        }
        assert!(!limbs_sub_limb_in_place(
            &mut out[3 * n + n_high..5 * n + n_high],
            special_carry_2
        ));
    }
    out[5 * n + n_high - 1].wrapping_add_assign(embankment);
}

const WANT_ASSERT: bool = true;

/// Interpolation for toom4, using the evaluation points 0, infinity, 1, -1, 2, -2, 1 / 2. More
/// precisely, we want to compute f(2 ^ (GMP_NUMB_BITS * n)) for a polynomial f of degree 6, given
/// the seven values
/// w0 = f(0),
/// w1 = f(-2),
/// w2 = f(1),
/// w3 = f(-1),
/// w4 = f(2)
/// w5 = 64 * f(1/2)
/// w6 = limit at infinity of f(x) / x ^ 6,
///
/// The result is 6 * n + n_high limbs. At entry, w0 is stored at {out, 2 * n}, w2 is stored
/// at {out + 2 * n, 2 * n + 1}, and w6 is stored at {out + 6 * n, n_high}. The other
/// values are 2 * n + 1 limbs each (with most significant limbs small). f(-1) and f(-1/2) may be
/// negative, signs determined by the flag bits. Inputs are destroyed.
///
/// Needs 2 * n + 1 limbs of temporary storage.
///
/// This is mpn_toom_interpolate_7pts from mpn/generic/mpn_toom_interpolate_7pts.c, but the argument
/// w6n == `n_high` is moved to immediately after `n`.
#[allow(clippy::cyclomatic_complexity, clippy::too_many_arguments)]
pub(crate) fn _limbs_mul_toom_interpolate_7_points(
    out: &mut [Limb],
    n: usize,
    n_high: usize,
    w1_neg: bool,
    w1: &mut [Limb],
    w3_neg: bool,
    w3: &mut [Limb],
    w4: &mut [Limb],
    w5: &mut [Limb],
    scratch: &mut [Limb],
) {
    let m = 2 * n + 1;
    assert_ne!(n_high, 0);
    assert!(n_high < m);
    assert_eq!(w1.len(), m);
    assert_eq!(w3.len(), m);
    assert_eq!(w4.len(), m);
    assert_eq!(w5.len(), m);
    {
        let (w0, remainder) = out.split_at_mut(2 * n); // w0 length: 2 * n
        let (w2, w6) = remainder.split_at_mut(4 * n);
        let w2 = &mut w2[..m];
        let w6 = &mut w6[..n_high];

        // Using formulas similar to Marco Bodrato's
        //
        // w5 =  w5 + w4
        // w1 = (w4 - w1) / 2
        // w4 =  w4 - w0
        // w4 = (w4 - w1) / 4 - w6 * 16
        // w3 = (w2 - w3) / 2
        // w2 =  w2 - w3
        //
        // w5 =  w5 - w2 * 65 May be negative.
        // w2 =  w2 - w6 - w0
        // w5 = (w5 + w2 * 45) / 2 Now >= 0 again.
        // w4 = (w4 - w2) / 3
        // w2 =  w2 - w4
        //
        // w1 =  w5 - w1 May be negative.
        // w5 = (w5 - w3 * 8) / 9
        // w3 =  w3 - w5
        // w1 = (w1 / 15 + w5) / 2 Now >= 0 again.
        // w5 =  w5 - w1
        //
        // where w0 = f(0), w1 = f(-2), w2 = f(1), w3 = f(-1),
        //   w4 = f(2), w5 = f(1/2), w6 = f(infinity),
        //
        // Note that most intermediate results are positive; the ones that may be negative are
        // represented in two's complement. We must never shift right a value that may be negative,
        // since that would invalidate the sign bit. On the other hand, divexact by odd numbers
        // works fine with two's complement.
        limbs_slice_add_same_length_in_place_left(w5, w4);
        if w1_neg {
            limbs_slice_add_same_length_in_place_left(w1, w4);
        } else {
            limbs_sub_same_length_in_place_right(w4, w1);
        }
        assert!(w1[0].even());
        limbs_slice_shr_in_place(w1, 1);
        limbs_sub_in_place_left(w4, w0);
        limbs_sub_same_length_in_place_left(w4, w1);
        assert!(w4[0].divisible_by_power_of_two(2));
        limbs_slice_shr_in_place(w4, 2); // w4 >= 0

        scratch[n_high] = limbs_shl_to_out(scratch, w6, 4);
        limbs_sub_in_place_left(w4, &scratch[..=n_high]);

        if w3_neg {
            limbs_slice_add_same_length_in_place_left(w3, w2);
        } else {
            limbs_sub_same_length_in_place_right(w2, w3);
        }
        assert!(w3[0].even());
        limbs_slice_shr_in_place(w3, 1);

        limbs_sub_same_length_in_place_left(w2, w3);

        mpn_submul_1(w5, w2, 65);
        limbs_sub_in_place_left(w2, w6);
        limbs_sub_in_place_left(w2, w0);

        mpn_addmul_1(w5, w2, 45);
        assert!(w5[0].even());
        limbs_slice_shr_in_place(w5, 1);
        limbs_sub_same_length_in_place_left(w4, w2);

        limbs_div_exact_3_in_place(w4);
        limbs_sub_same_length_in_place_left(w2, w4);

        limbs_sub_same_length_in_place_right(w5, w1);
        limbs_shl_to_out(scratch, w3, 3);
        limbs_sub_same_length_in_place_left(w5, &scratch[..m]);
        limbs_div_exact_limb_in_place(w5, 9);
        limbs_sub_same_length_in_place_left(w3, w5);

        limbs_div_exact_limb_in_place(w1, 15);
        limbs_slice_add_same_length_in_place_left(w1, w5);
        assert!(w1[0].even());
        limbs_slice_shr_in_place(w1, 1); // w1 >= 0 now
        limbs_sub_same_length_in_place_left(w5, w1);

        // These bounds are valid for the 4x4 polynomial product of toom44,
        // and they are conservative for toom53 and toom62.
        assert!(w1[2 * n] < 2);
        assert!(w2[2 * n] < 3);
        assert!(w3[2 * n] < 4);
        assert!(w4[2 * n] < 3);
        assert!(w5[2 * n] < 2);
    }

    // Addition chain. Note carries and the 2n'th limbs that need to be added in.
    //
    // Special care is needed for w2[2 * n] and the corresponding carry, since the "simple" way of
    // adding it all together would overwrite the limb at wp[2 * n] and out[4 * n] (same
    // location) with the sum of the high half of w3 and the low half of w4.
    //
    //         7    6    5    4    3    2    1    0
    //    |    |    |    |    |    |    |    |    |
    //                  ||w3 (2n+1)|
    //             ||w4 (2n+1)|
    //        ||w5 (2n+1)|        ||w1 (2n+1)|
    //  +     |w6(n_high)|        ||w2 (2n+1)| w0 (2n) |  (share storage with r)
    //  -----------------------------------------------
    //  r |    |    |    |    |    |    |    |    |
    //        c7   c6   c5   c4   c3                 Carries to propagate
    {
        let (out_lo, out_hi) = out[n..].split_at_mut(m);
        if limbs_slice_add_same_length_in_place_left(out_lo, w1) {
            assert!(!limbs_slice_add_limb_in_place(&mut out_hi[..n], 1));
        }
    }
    split_into_chunks_mut!(&mut out[3 * n..], n, [out_3, out_4, out_5], remainder);
    let mut addend = out_4[0];
    let (w3_lo, w3_hi) = w3.split_at_mut(n);
    if limbs_slice_add_same_length_in_place_left(out_3, w3_lo) {
        addend.wrapping_add_assign(1);
    }
    assert!(!limbs_slice_add_limb_in_place(w3_hi, addend));
    let (w3_hi_last, w3_hi_init) = w3_hi.split_last_mut().unwrap();
    let mut addend = *w3_hi_last;
    let (w4_lo, w4_hi) = w4.split_at_mut(n);
    if limbs_add_same_length_to_out(out_4, w3_hi_init, w4_lo) {
        addend += 1;
    }
    assert!(!limbs_slice_add_limb_in_place(w4_hi, addend));
    let (w4_last, w4_init) = w4_hi.split_last_mut().unwrap();
    let mut addend = *w4_last;
    let (w5_lo, w5_hi) = w5.split_at_mut(n);
    if limbs_add_same_length_to_out(out_5, w4_init, w5_lo) {
        addend += 1;
    }
    assert!(!limbs_slice_add_limb_in_place(w5_hi, addend));
    if n_high > n + 1 {
        assert!(!limbs_slice_add_greater_in_place_left(remainder, w5_hi));
    } else {
        let (w5_hi_lo, w5_hi_hi) = w5_hi.split_at_mut(n_high);
        assert!(!limbs_slice_add_same_length_in_place_left(
            &mut remainder[..n_high],
            w5_hi_lo
        ));
        if WANT_ASSERT && n + n_high < m {
            limbs_test_zero(w5_hi_hi);
        }
    }
}

// This is DO_mpn_sublsh_n from mpn/generic/mpn_toom_interpolate_8pts.c.
fn shl_and_sub_same_length(xs: &mut [Limb], ys: &[Limb], shift: u32, scratch: &mut [Limb]) -> Limb {
    let n = ys.len();
    assert!(scratch.len() >= n);
    assert!(xs.len() >= n);
    let mut carry = limbs_shl_to_out(scratch, ys, shift);
    if limbs_sub_same_length_in_place_left(&mut xs[..n], &scratch[..n]) {
        carry.wrapping_add_assign(1);
    }
    carry
}

// This is DO_mpn_subrsh from mpn/generic/mpn_toom_interpolate_8pts.c.
fn shl_and_sub(xs: &mut [Limb], ys: &[Limb], shift: u32, scratch: &mut [Limb]) {
    assert!(!limbs_sub_limb_in_place(xs, ys[0] >> shift));
    let carry = shl_and_sub_same_length(xs, &ys[1..], Limb::WIDTH - shift, scratch);
    assert!(!limbs_sub_limb_in_place(&mut xs[ys.len() - 1..], carry));
}

/// Interpolation for Toom-4.5 (or Toom-4), using the evaluation points: infinity(4.5 only), 4, -4,
/// 2, -2, 1, -1, 0. More precisely, we want to compute f(2 ^ (`Limb::WIDTH` * n)) for a polynomial
/// f of degree 7 (or 6), given the 8 (rsp. 7) values:
///
/// r1 = limit at infinity of f(x) / x ^ 7,
/// r2 = f(4),
/// r3 = f(-4),
/// r4 = f(2),
/// r5 = f(-2),
/// r6 = f(1),
/// r7 = f(-1),
/// r8 = f(0).
///
/// All couples of the form f(n),f(-n) must be already mixed with
/// `_limbs_toom_couple_handling`(f(n),..., f(-n), ...)
///
/// The result is stored in {`out`, `s_plus_t` + 7 * n (or 6 * n)}. At entry, `r8` is stored at
/// {`out`, 2 * `n`}, and r5 is stored at {`out` + 3 * `n`, 3 * `n` + 1}.
///
/// The other values are 2 * `n` + ... limbs each (with most significant limbs small).
///
/// All intermediate results are positive. Inputs are destroyed.
///
/// This is mpn_toom_interpolate_8pts from mpn/generic/mpn_toom_interpolate_8pts.c, but the argument
/// spt == `s_plus_t` is moved to immediately after `n`.
pub(crate) fn _limbs_mul_toom_interpolate_8_points(
    out: &mut [Limb],
    n: usize,
    s_plus_t: usize,
    r3: &mut [Limb],
    r7: &mut [Limb],
    scratch: &mut [Limb],
) {
    assert!(s_plus_t >= n);
    let limit = 3 * n + 1;
    assert_eq!(r3.len(), limit);
    assert_eq!(r7.len(), limit);
    let (out_1, remainder) = out.split_at_mut(2 * n);
    let (out_2, remainder) = remainder.split_at_mut(n);
    let (r5, r1) = remainder.split_at_mut(4 * n);
    let r1 = &mut r1[..s_plus_t];
    {
        let r5 = &mut r5[..limit];

        // Interpolation
        shl_and_sub(&mut r3[n..], out_1, 4, scratch);
        let cy = shl_and_sub_same_length(r3, r1, 12, scratch);
        assert!(!limbs_sub_limb_in_place(&mut r3[s_plus_t..], cy));

        shl_and_sub(&mut r5[n..], out_1, 2, scratch);
        let cy = shl_and_sub_same_length(r5, r1, 6, scratch);
        assert!(!limbs_sub_limb_in_place(&mut r5[s_plus_t..], cy));

        if limbs_sub_same_length_in_place_left(&mut r7[n..3 * n], out_1) {
            r7[3 * n].wrapping_sub_assign(1);
        }
        if limbs_sub_same_length_in_place_left(&mut r7[..s_plus_t], r1) {
            assert!(!limbs_sub_limb_in_place(&mut r7[s_plus_t..], 1));
        }
        assert!(!limbs_sub_same_length_in_place_left(r3, r5));
        assert_eq!(limbs_slice_shr_in_place(r3, 2), 0);
        assert!(!limbs_sub_same_length_in_place_left(r5, r7));
        assert!(!limbs_sub_same_length_in_place_left(r3, r5));
        limbs_div_exact_limb_in_place(r3, 45);
        limbs_div_exact_3_in_place(r5);
        assert_eq!(shl_and_sub_same_length(r5, r3, 2, scratch), 0);

        // Last interpolation steps are mixed with recomposition.
        //
        // out[] prior to operations:
        // |_H r1|_L r1|____||_H r5|_M_r5|_L r5|_____|_H r8|_L r8|out
        //
        // summation scheme for remaining operations:
        // |____8|n___7|n___6|n___5|n___4|n___3|n___2|n____|n____|out
        // |_H r1|_L r1|____||_H*r5|_M r5|_L r5|_____|_H_r8|_L r8|out
        //  ||_H r3|_M r3|_L*r3|
        //              ||_H_r7|_M_r7|_L_r7|
        //          ||-H r3|-M r3|-L*r3|
        //              ||-H*r5|-M_r5|-L_r5|
        //
        // Hr8+Lr7-Lr5
        let carry_1 = limbs_slice_add_same_length_in_place_left(&mut out_1[n..], &r7[..n]);
        let carry_2 = limbs_sub_same_length_in_place_left(&mut out_1[n..], &r5[..n]);
        if carry_1 && !carry_2 {
            assert!(!limbs_slice_add_limb_in_place(&mut r7[n..], 1));
        } else if !carry_1 && carry_2 {
            assert!(!limbs_sub_limb_in_place(&mut r7[n..], 1));
        }

        // Mr7-Mr5
        if limbs_sub_same_length_to_out(out_2, &r7[n..2 * n], &r5[n..2 * n]) {
            assert!(!limbs_sub_limb_in_place(&mut r7[2 * n..], 1));
        }

        // Hr5+Lr3
        if limbs_slice_add_same_length_in_place_left(&mut r5[2 * n..3 * n], &r3[..n]) {
            r5[3 * n].wrapping_add_assign(1);
        }
        // Hr7+Lr5
        let carry_1 = limbs_slice_add_same_length_in_place_left(&mut r5[..n + 1], &r7[2 * n..]);
        let carry_2;
        // Hr7-Hr5+Lr5-Lr3
        {
            let (r5_lo, r5_hi) = r5.split_at_mut(2 * n);
            carry_2 = limbs_sub_same_length_in_place_left(&mut r5_lo[..n + 1], &r5_hi[..n + 1]);
        }
        if carry_1 && !carry_2 {
            assert!(!limbs_slice_add_limb_in_place(&mut r5[n + 1..], 1));
        } else if !carry_1 && carry_2 {
            assert!(!limbs_sub_limb_in_place(&mut r5[n + 1..], 1));
        }

        // Mr5-Mr3,Hr5-Hr3
        assert!(!limbs_sub_same_length_in_place_left(&mut r5[n..], &r3[n..]));
    }
    // here `r5` is back to its full length
    let r5_3n = r5[3 * n];
    if limbs_add_limb_to_out(&mut r5[3 * n..], &r3[n..2 * n], r5_3n) {
        assert!(!limbs_slice_add_limb_in_place(&mut r3[2 * n..], 1));
    }
    let mut r3_3n = r3[3 * n];
    if limbs_slice_add_same_length_in_place_left(&mut r1[..n], &r3[2 * n..3 * n]) {
        r3_3n.wrapping_add_assign(1);
    }
    if s_plus_t == n {
        assert_eq!(r3_3n, 0);
    } else {
        assert!(!limbs_slice_add_limb_in_place(&mut r1[n..], r3_3n));
    }
}

fn umul_ppmm(ph: &mut Limb, pl: &mut Limb, m1: Limb, m2: Limb) {
    let (hi, lo) = (DoubleLimb::from(m1) * DoubleLimb::from(m2)).split_in_half();
    *ph = hi;
    *pl = lo;
}

// This is mpn_bdiv_dbm1c from gmp-impl.h.
pub fn mpn_bdiv_dbm1c(qp: &mut [Limb], ap: &[Limb], bd: Limb, mut h: Limb) -> Limb {
    let n = ap.len();
    for i in 0..n {
        let a = ap[i];
        let mut p1 = 0;
        let mut p0 = 0;
        umul_ppmm(&mut p1, &mut p0, a, bd);
        let cy = if h < p0 { 1 } else { 0 };
        h = h.wrapping_sub(p0);
        qp[i] = h;
        h = h.wrapping_sub(p1).wrapping_sub(cy);
    }
    h
}

// This is mpn_bdiv_dbm1c from gmp-impl.h.
fn mpn_bdiv_dbm1c_in_place(qp: &mut [Limb], bd: Limb, mut h: Limb) -> Limb {
    let n = qp.len();
    for i in 0..n {
        let a = qp[i];
        let mut p1 = 0;
        let mut p0 = 0;
        umul_ppmm(&mut p1, &mut p0, a, bd);
        let cy = if h < p0 { 1 } else { 0 };
        h = h.wrapping_sub(p0);
        qp[i] = h;
        h = h.wrapping_sub(p1).wrapping_sub(cy);
    }
    h
}

fn mpn_divexact_by255_in_place(dst: &mut [Limb]) -> Limb {
    255 & mpn_bdiv_dbm1c_in_place(dst, Limb::MAX / 255, 0)
}

//TODO tune
const AORSMUL_FASTER_AORS_AORSLSH: bool = false;
const AORSMUL_FASTER_2AORSLSH: bool = false;
const AORSMUL_FASTER_3AORSLSH: bool = false;
const AORSMUL_FASTER_AORS_2AORSLSH: bool = false;

/// Interpolation for Toom-6.5 (or Toom-6), using the evaluation
/// points: infinity(6.5 only), +-4, +-2, +-1, +-1/4, +-1/2, 0. More precisely,
/// we want to compute f(2^(GMP_NUMB_BITS * n)) for a polynomial f of
/// degree 11 (or 10), given the 12 (rsp. 11) values:
///
///   r0 = limit at infinity of f(x) / x^7,
///   r1 = f(4),f(-4),
///   r2 = f(2),f(-2),
///   r3 = f(1),f(-1),
///   r4 = f(1/4),f(-1/4),
///   r5 = f(1/2),f(-1/2),
///   r6 = f(0).
///
/// All couples of the form f(n),f(-n) must be already mixed with
/// toom_couple_handling(f(n),...,f(-n),...)
///
/// The result is stored in {pp, spt + 7*n (or 6*n)}.
/// At entry, r6 is stored at {pp, 2n},
/// r4 is stored at {pp + 3n, 3n + 1}.
/// r2 is stored at {pp + 7n, 3n + 1}.
/// r0 is stored at {pp +11n, spt}.
///
/// The other values are 3n+1 limbs each (with most significant limbs small).
///
/// Negative intermediate results are stored two-complemented.
/// Inputs are destroyed.
///
/// This is mpn_toom_interpolate_12pts from mpn/generic/mpn_toom_interpolate_12pts.c.
pub fn _limbs_mul_toom_interpolate_12_points<'a>(
    pp: &mut [Limb],
    mut r1: &'a mut [Limb],
    r3: &mut [Limb],
    mut r5: &'a mut [Limb],
    n: usize,
    spt: usize,
    half: bool,
    mut wsi: &'a mut [Limb],
) {
    let n3 = 3 * n;
    let n3p1 = n3 + 1;
    {
        let (pp_lo, remainder) = pp.split_at_mut(3 * n);
        let pp_lo = &mut pp_lo[..2 * n];
        let (r4, r2) = remainder.split_at_mut(4 * n);

        // Interpolation
        if half {
            let (r2, r0) = r2.split_at_mut(4 * n);
            let cy = if limbs_sub_same_length_in_place_left(&mut r3[..spt], &r0[..spt]) {
                1
            } else {
                0
            };
            assert!(!limbs_sub_limb_in_place(&mut r3[spt..n3p1], cy));

            let cy = shl_and_sub_same_length(r2, &r0[..spt], 10, wsi);
            assert!(!limbs_sub_limb_in_place(&mut r2[spt..n3p1], cy));
            shl_and_sub(&mut r5[..n3p1], &r0[..spt], 2, wsi);

            let cy = shl_and_sub_same_length(r1, &r0[..spt], 20, wsi);
            assert!(!limbs_sub_limb_in_place(&mut r1[spt..n3p1], cy));
            shl_and_sub(&mut r4[..n3p1], &r0[..spt], 4, wsi);
        };

        let carry = shl_and_sub_same_length(&mut r4[n..], pp_lo, 20, wsi);
        r4[n3].wrapping_sub_assign(carry);
        shl_and_sub(&mut r1[n..3 * n + 1], pp_lo, 4, wsi);

        assert!(!limbs_add_same_length_to_out(wsi, &r1[..n3p1], &r4[..n3p1]));
        limbs_sub_same_length_in_place_left(&mut r4[..n3p1], &r1[..n3p1]); // can be negative
        swap(&mut r1, &mut wsi);

        let carry = shl_and_sub_same_length(&mut r5[n..], pp_lo, 10, wsi);
        r5[n3].wrapping_sub_assign(carry);
        shl_and_sub(&mut r2[n..3 * n + 1], pp_lo, 2, wsi);

        limbs_sub_same_length_to_out(wsi, &r5[..n3p1], &r2[..n3p1]); // can be negative
        assert!(!limbs_slice_add_same_length_in_place_left(
            &mut r2[..n3p1],
            &r5[..n3p1]
        ));
        swap(&mut r5, &mut wsi);

        let carry = if limbs_sub_same_length_in_place_left(&mut r3[n..3 * n], pp_lo) {
            1
        } else {
            0
        };
        r3[n3].wrapping_sub_assign(carry);
        if AORSMUL_FASTER_AORS_AORSLSH {
            mpn_submul_1(r4, &r5[..n3p1], 257); // can be negative
        } else {
            limbs_sub_same_length_in_place_left(&mut r4[..n3p1], &r5[..n3p1]); // can be negative
            shl_and_sub_same_length(r4, &r5[..n3p1], 8, wsi); // can be negative
        }

        // A division by 2835x4 follows. Warning: the operand can be negative!
        limbs_div_exact_limb_in_place(&mut r4[..n3p1], 2_835 << 2);
        if (r4[n3] & (Limb::MAX << (Limb::WIDTH - 3))) != 0 {
            r4[n3] |= Limb::MAX << (Limb::WIDTH - 2);
        }

        if AORSMUL_FASTER_2AORSLSH {
            mpn_addmul_1(r5, &r4[..n3p1], 60); // can be negative
        } else {
            shl_and_sub_same_length(r5, &r4[..n3p1], 2, wsi); // can be negative
            do_mpn_addlsh_n(r5, &r4[..n3p1], 6, wsi); // can give a carry
        }
        mpn_divexact_by255_in_place(&mut r5[..n3p1]);

        assert_eq!(shl_and_sub_same_length(r2, &r3[..n3p1], 5, wsi), 0);
        if AORSMUL_FASTER_3AORSLSH {
            assert_eq!(mpn_submul_1(r1, &r2[..n3p1], 100), 0);
        } else {
            assert_eq!(shl_and_sub_same_length(r1, &r2[..n3p1], 6, wsi), 0);
            assert_eq!(shl_and_sub_same_length(r1, &r2[..n3p1], 5, wsi), 0);
            assert_eq!(shl_and_sub_same_length(r1, &r2[..n3p1], 2, wsi), 0);
        }
        assert_eq!(shl_and_sub_same_length(r1, &r3[..n3p1], 9, wsi), 0);
        limbs_div_exact_limb_in_place(&mut r1[..n3p1], 42_525);

        if AORSMUL_FASTER_AORS_2AORSLSH {
            assert_eq!(mpn_submul_1(r2, &r1[..n3p1], 225), 0);
        } else {
            assert!(!limbs_sub_same_length_in_place_left(
                &mut r2[..n3p1],
                &r1[..n3p1]
            ));
            assert_eq!(do_mpn_addlsh_n(r2, &r1[..n3p1], 5, wsi), 0);
            assert_eq!(shl_and_sub_same_length(r2, &r1[..n3p1], 8, wsi), 0);
        }
        limbs_div_exact_limb_in_place(&mut r2[..n3p1], 9 << 2);

        assert!(!limbs_sub_same_length_in_place_left(
            &mut r3[..n3p1],
            &r2[..n3p1]
        ));

        limbs_sub_same_length_in_place_right(&r2[..n3p1], &mut r4[..n3p1]);
        assert_eq!(limbs_slice_shr_in_place(&mut r4[..n3p1], 1), 0);
        assert!(!limbs_sub_same_length_in_place_left(
            &mut r2[..n3p1],
            &r4[..n3p1]
        ));
    }

    limbs_slice_add_same_length_in_place_left(&mut r5[..n3p1], &r1[..n3p1]);
    assert_eq!(limbs_slice_shr_in_place(&mut r5[..n3p1], 1), 0);

    // Last interpolation steps...
    assert!(!limbs_sub_same_length_in_place_left(
        &mut r3[..n3p1],
        &r1[..n3p1]
    ));
    assert!(!limbs_sub_same_length_in_place_left(
        &mut r1[..n3p1],
        &r5[..n3p1]
    ));
    // ...could be mixed with recomposition
    // ||H-r5|M-r5|L-r5|   ||H-r1|M-r1|L-r1|
    //
    // Recomposition
    //
    // pp[] prior to operations:
    // |M r0|L r0|___||H r2|M r2|L r2|___||H r4|M r4|L r4|____|H_r6|L r6|pp
    //
    // summation scheme for remaining operations:
    // |__12|n_11|n_10|n__9|n__8|n__7|n__6|n__5|n__4|n__3|n__2|n___|n___|pp
    // |M r0|L r0|___||H r2|M r2|L r2|___||H r4|M r4|L r4|____|H_r6|L r6|pp
    // ||H r1|M r1|L r1|   ||H r3|M r3|L r3|   ||H_r5|M_r5|L_r5|
    let mut cy = if limbs_slice_add_same_length_in_place_left(&mut pp[n..2 * n], &r5[..n]) {
        1
    } else {
        0
    };
    cy = if limbs_add_limb_to_out(&mut pp[n << 1..], &r5[n..n << 1], cy) {
        1
    } else {
        0
    };
    assert!(!limbs_slice_add_limb_in_place(
        &mut r5[2 * n..3 * n + 1],
        cy
    ));
    cy = r5[n3].wrapping_add(
        if limbs_slice_add_same_length_in_place_left(&mut pp[n3..n3 + n], &r5[2 * n..3 * n]) {
            1
        } else {
            0
        },
    );
    assert!(!limbs_slice_add_limb_in_place(
        &mut pp[n3 + n..n3 + 3 * n + 1],
        cy
    ));

    let carry = if limbs_slice_add_same_length_in_place_left(&mut pp[5 * n..6 * n], &r3[..n]) {
        1
    } else {
        0
    };
    pp[2 * n3].wrapping_add_assign(carry);
    let x = pp[2 * n3];
    cy = if limbs_add_limb_to_out(&mut pp[2 * n3..], &r3[n..2 * n], x) {
        1
    } else {
        0
    };
    assert!(!limbs_slice_add_limb_in_place(
        &mut r3[2 * n..3 * n + 1],
        cy
    ));
    cy = r3[n3].wrapping_add(
        if limbs_slice_add_same_length_in_place_left(&mut pp[7 * n..8 * n], &r3[2 * n..3 * n]) {
            1
        } else {
            0
        },
    );
    assert!(!limbs_slice_add_limb_in_place(
        &mut pp[8 * n..10 * n + 1],
        cy
    ));

    let carry = if limbs_slice_add_same_length_in_place_left(&mut pp[9 * n..10 * n], &r1[..n]) {
        1
    } else {
        0
    };
    pp[10 * n].wrapping_add_assign(carry);
    if half {
        let x = pp[10 * n];
        cy = if limbs_add_limb_to_out(&mut pp[10 * n..], &r1[n..2 * n], x) {
            1
        } else {
            0
        };
        assert!(!limbs_slice_add_limb_in_place(
            &mut r1[2 * n..3 * n + 1],
            cy
        ));
        if spt > n {
            cy = r1[n3].wrapping_add(
                if limbs_slice_add_same_length_in_place_left(
                    &mut pp[11 * n..12 * n],
                    &r1[2 * n..3 * n],
                ) {
                    1
                } else {
                    0
                },
            );
            assert!(!limbs_slice_add_limb_in_place(
                &mut pp[4 * n3..4 * n3 + spt - n],
                cy
            ));
        } else {
            assert!(!limbs_slice_add_same_length_in_place_left(
                &mut pp[11 * n..11 * n + spt],
                &r1[2 * n..2 * n + spt]
            ));
        }
    } else {
        let x = pp[10 * n];
        assert!(!limbs_add_limb_to_out(
            &mut pp[10 * n..],
            &r1[n..n + spt],
            x
        ));
    }
}
