use ark_algebra_bench_templates::*;
use pallet_template::ed_on_bls12_377::HostEdOnBls12_377;
use sp_ark_ed_on_bls12_377::{
	fq::Fq as FqOptimized, EdwardsProjective as EdwardsProjectiveOptimized, Fr as Fr_optimized,
};

type GOptimized = EdwardsProjectiveOptimized<HostEdOnBls12_377>;

bench!(
	Name = "EdOnBls12_381_optimized",
	Group = G_optimized,
	ScalarField = Fr_optimized,
	PrimeBaseField = Fq_optimized,
);
