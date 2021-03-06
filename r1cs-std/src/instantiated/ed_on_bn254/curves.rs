use crate::groups::curves::twisted_edwards::AffineGadget;
use algebra::ed_on_bn254::*;

use crate::ed_on_bn254::FqGadget;

pub type EdwardsGadget = AffineGadget<EdwardsParameters, Fq, FqGadget>;

#[test]
fn test() {
    crate::groups::curves::twisted_edwards::test::<Fq, _, EdwardsGadget>();
}
