use ark_algebra_bench_templates::*;
use ark_bw6_761::{fq::Fq, fq3::Fq3, fq6::Fq6, Fr, G1Projective, G2Projective, BW6_761};
use criterion::Criterion;

bench!(
	Name = "BW6_761",
	Pairing = BW6_761,
	G1 = G1Projective,
	G2 = G2Projective,
	ScalarField = Fr,
	G1BaseField = Fq,
	G2BaseField = Fq3,
	TargetField = Fq6,
);
