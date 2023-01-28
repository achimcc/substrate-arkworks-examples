use ark_algebra_bench_templates::*;
use criterion::Criterion;
use pallet_template::bls12_377::{
	Bls12_377Optimized, G1ProjectiveBls12_377_Host, G2ProjectiveBls12_377_Host,
};
use sp_ark_bls12_377::{
	fq12::Fq12 as Fq12_optimized, fq2::Fq2 as Fq2_optimized, fr::Fr as Fr_optimized,
	Fq as Fq_optimized,
};

bench!(
	Name = "Bls12_377_optimized",
	Pairing = Bls12_377Optimized,
	G1 = G1ProjectiveBls12_377_Host
	G2 = G2ProjectiveBls12_377_Host,
	ScalarField = Fr_optimized,
	G1BaseField = Fq_optimized,
	G2BaseField = Fq2_optimized,
	TargetField = Fq12_optimized,
);
