use ark_algebra_bench_templates::*;
use criterion::Criterion;
use pallet_template::ed_on_bls12_377::HostEdOnBls12_377;
use sp_ark_ed_on_bls12_377::{
	fq::Fq as Fq_optimized, EdwardsProjective as EdwardsProjective_optimized, Fr as Fr_optimized,
};

type G_optimized = EdwardsProjective_optimized<HostEdOnBls12_377>;

bench!(
	Name = "EdOnBls12_381_optimized",
	Group = G_optimized,
	ScalarField = Fr_optimized,
	PrimeBaseField = Fq_optimized,
);
