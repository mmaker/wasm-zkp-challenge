use ark_bls12_381::g1::{G1Affine, G1Projective};
use ark_ec::{msm::VariableBaseMSM, AffineCurve, ProjectiveCurve};
use ark_ff::{PrimeField, UniformRand, Zero};

pub fn generate_msm_inputs(
    size: usize,
) -> (
    Vec<<<G1Affine as AffineCurve>::Projective as ProjectiveCurve>::Affine>,
    Vec<<<G1Affine as AffineCurve>::ScalarField as PrimeField>::BigInt>,
) {
    let mut rng = ark_std::test_rng();

    let scalar_vec = (0..size - 1)
        .map(|_| <G1Affine as AffineCurve>::ScalarField::rand(&mut rng).into_bigint())
        .collect::<Vec<_>>();
    let point_vec = (0..size)
        .map(|_| <G1Affine as AffineCurve>::Projective::rand(&mut rng))
        .collect::<Vec<_>>();
    let point_vec =
        <<G1Affine as AffineCurve>::Projective as ProjectiveCurve>::batch_normalization_into_affine(
            &point_vec,
        );

    (point_vec, scalar_vec)
}

/// Currently using Pippenger's algorithm for multi-scalar multiplication (MSM)
pub fn compute_msm(
    point_vec: Vec<<<G1Affine as AffineCurve>::Projective as ProjectiveCurve>::Affine>,
    scalar_vec: Vec<<<G1Affine as AffineCurve>::ScalarField as PrimeField>::BigInt>,
) -> <G1Affine as AffineCurve>::Projective {
    G1Projective::msm_bigint(point_vec.as_slice(), scalar_vec.as_slice())
}

#[test]
fn test() {
    let size = 1 << 14;
    let (point_vec, scalar_vec) = generate_msm_inputs(size);
    let res = compute_msm(point_vec, scalar_vec);
}
