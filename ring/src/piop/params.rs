use ark_ec::{AffineRepr, CurveGroup, Group};
use ark_ec::short_weierstrass::{Affine, Projective, SWCurveConfig};
use ark_ff::{BigInteger, PrimeField};
use ark_std::rand::Rng;
use ark_std::UniformRand;
use common::domain::Domain;


#[derive(Clone)]
pub struct PiopParams<F: PrimeField, Curve: SWCurveConfig<BaseField=F>> {
    // Domain over which the piop is represented.
    pub(crate) domain: Domain<F>,

    // Number of bits used to represent a jubjub scalar.
    pub(crate) scalar_bitlen: usize,

    // Length of the part of the column representing the public keys (including the padding).
    pub(crate) keyset_part_size: usize,

    // The blinding base, a point from jubjub.
    pub h: Affine<Curve>,
}

impl<F: PrimeField, Curve: SWCurveConfig<BaseField=F>> PiopParams<F, Curve> {
    pub fn setup<R: Rng>(domain: Domain<F>, rng: &mut R) -> Self {
        let scalar_bitlen = Curve::ScalarField::MODULUS_BIT_SIZE as usize;
        // 1 accounts for the last cells of the points and bits columns that remain unconstrained
        let keyset_part_size = domain.capacity - scalar_bitlen - 1;

        let h = Affine::<Curve>::rand(rng);
        // let powers_of_h = Self::power_of_2_multiples(scalar_bitlen, h.into_projective());
        // let powers_of_h = CurveGroup::batch_normalization_into_affine(&powers_of_h);

        Self {
            domain,
            scalar_bitlen,
            keyset_part_size,
            h,
        }
    }

    pub fn power_of_2_multiples_of_h(&self) -> Vec<Affine::<Curve>> {
        let mut h = self.h.into_group();
        let mut multiples = Vec::with_capacity(self.scalar_bitlen);
        multiples.push(h);
        for _ in 1..self.scalar_bitlen {
            h.double_in_place();
            multiples.push(h);
        }
        CurveGroup::normalize_batch(&multiples)
    }

    pub fn scalar_part(&self, e: Curve::ScalarField) -> Vec<bool> {
        let bits_with_trailing_zeroes = e.into_bigint().to_bits_le();
        let significant_bits = &bits_with_trailing_zeroes[..self.scalar_bitlen];
        significant_bits.to_vec()
    }

    pub fn keyset_part_selector(&self) -> Vec<F> {
        [
            vec![F::one(); self.keyset_part_size],
            vec![F::zero(); self.scalar_bitlen]
        ].concat()
    }
}

#[cfg(test)]
mod tests {
    use ark_ec::AffineRepr;
    use ark_ed_on_bls12_381_bandersnatch::{BandersnatchParameters, Fq, Fr};
    use ark_std::{test_rng, UniformRand};
    use std::ops::Mul;
    use common::domain::Domain;
    use common::test_helpers::cond_sum;
    use crate::piop::params::PiopParams;

    #[test]
    fn test_powers_of_h() {
        let rng = &mut test_rng();
        let domain = Domain::new(1024, false);
        let params = PiopParams::<Fq, BandersnatchParameters>::setup(domain, rng);
        let t = Fr::rand(rng);
        let t_bits = params.scalar_part(t);
        let th = cond_sum(&t_bits, &params.power_of_2_multiples_of_h());
        assert_eq!(th, params.h.mul(t));
    }
}