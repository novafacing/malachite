use std::cmp::Ordering;

use malachite_base::num::arithmetic::traits::DivisibleBy;
use malachite_base::slices::slice_trailing_zeros;
use malachite_nz::natural::arithmetic::divisible_by::{
    limbs_divisible_by_limb, limbs_divisible_by_val_ref,
};
use malachite_nz::natural::arithmetic::mod_op::{limbs_mod, limbs_mod_limb};
use malachite_nz::natural::arithmetic::sub::{limbs_sub, limbs_sub_limb};
use malachite_nz::natural::comparison::ord::limbs_cmp;
use malachite_nz::platform::Limb;

pub fn limbs_eq_limb_mod_naive_1(xs: &[Limb], y: Limb, ms: &[Limb]) -> bool {
    assert!(xs.len() > 1);
    assert!(ms.len() > 1);
    let mut xs_mod = if xs.len() >= ms.len() {
        limbs_mod(xs, ms)
    } else {
        xs.to_vec()
    };
    xs_mod.truncate(xs_mod.len() - slice_trailing_zeros(&xs_mod));
    xs_mod == [y]
}

pub fn limbs_eq_limb_mod_naive_2(xs: &[Limb], y: Limb, ms: &[Limb]) -> bool {
    let mut diff = limbs_sub_limb(xs, y).0;
    diff.truncate(diff.len() - slice_trailing_zeros(&diff));
    diff.len() >= ms.len() && limbs_divisible_by_val_ref(&mut diff, ms)
}

pub fn limbs_eq_mod_limb_naive_1(xs: &[Limb], ys: &[Limb], ms: Limb) -> bool {
    assert!(xs.len() > 1);
    assert!(ys.len() > 1);
    limbs_mod_limb(xs, ms) == limbs_mod_limb(ys, ms)
}

pub fn limbs_eq_mod_limb_naive_2(xs: &[Limb], ys: &[Limb], ms: Limb) -> bool {
    if xs == ys {
        return true;
    }
    let mut diff = if limbs_cmp(xs, ys) >= Ordering::Equal {
        limbs_sub(xs, ys)
    } else {
        limbs_sub(ys, xs)
    }
    .0;
    diff.truncate(diff.len() - slice_trailing_zeros(&diff));
    if diff.len() == 1 {
        diff[0].divisible_by(ms)
    } else {
        limbs_divisible_by_limb(&diff, ms)
    }
}

pub fn limbs_eq_mod_naive_1(xs: &[Limb], ys: &[Limb], ms: &[Limb]) -> bool {
    let mut xs_mod = if xs.len() >= ms.len() {
        limbs_mod(xs, ms)
    } else {
        xs.to_vec()
    };
    let mut ys_mod = if ys.len() >= ms.len() {
        limbs_mod(ys, ms)
    } else {
        ys.to_vec()
    };
    xs_mod.truncate(xs_mod.len() - slice_trailing_zeros(&xs_mod));
    ys_mod.truncate(ys_mod.len() - slice_trailing_zeros(&ys_mod));
    limbs_cmp(&xs_mod, &ys_mod) == Ordering::Equal
}

pub fn limbs_eq_mod_naive_2(xs: &[Limb], ys: &[Limb], ms: &[Limb]) -> bool {
    if xs == ys {
        return true;
    }
    let mut diff = if limbs_cmp(xs, ys) >= Ordering::Equal {
        limbs_sub(xs, ys)
    } else {
        limbs_sub(ys, xs)
    }
    .0;
    diff.truncate(diff.len() - slice_trailing_zeros(&diff));
    diff.len() >= ms.len() && limbs_divisible_by_val_ref(&mut diff, ms)
}
