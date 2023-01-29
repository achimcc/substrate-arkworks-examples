use ark_algebra_bench_templates::*;
use ark_bls12_381::{
	fq12::Fq12, fq2::Fq2, fr::Fr, g1::G1Projective, g2::G2Projective, Bls12_381, Fq,
};

bench!(
	Name = "Bls12_381",
	Pairing = Bls12_381,
	G1 = G1Projective,
	G2 = G2Projective,
	ScalarField = Fr,
	G1BaseField = Fq,
	G2BaseField = Fq2,
	TargetField = Fq12,
);
