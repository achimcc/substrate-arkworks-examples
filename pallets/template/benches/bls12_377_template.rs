use ark_algebra_bench_templates::*;
use ark_bls12_377::{
	fq::Fq, fq2::Fq2, fr::Fr, g1::G1Projective, g2::G2Projective, Bls12_377, Fq12,
};
use criterion::Criterion;

bench!(
	Name = "Bls12_377",
	Pairing = Bls12_377,
	G1 = G1Projective,
	G2 = G2Projective,
	ScalarField = Fr,
	G1BaseField = Fq,
	G2BaseField = Fq2,
	TargetField = Fq12,
);
