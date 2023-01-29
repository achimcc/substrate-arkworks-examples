use ark_algebra_bench_templates::*;
use pallet_template::bls12_377::{
	Bls12_377Optimized, G1ProjectiveBls12_377, G2ProjectiveBls12_377,
};
use sp_ark_bls12_377::{
	fq12::Fq12 as Fq12Optimized, fq2::Fq2 as Fq2Optimized, fr::Fr as FrOptimized, Fq as FqOptimized,
};

bench!(
	Name = "Bls12_377_optimized",
	Pairing = Bls12_377Optimized,
	G1 = G1ProjectiveBls12_377,
	G2 = G2ProjectiveBls12_377,
	ScalarField = FrOptimized,
	G1BaseField = FqOptimized,
	G2BaseField = Fq2Optimized,
	TargetField = Fq12Optimized,
);
