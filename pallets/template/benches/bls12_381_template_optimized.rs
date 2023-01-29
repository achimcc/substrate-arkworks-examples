use ark_algebra_bench_templates::*;
use pallet_template::bls12_381::{
	Bls12_381Optimized, G1ProjectiveBls12_381, G2ProjectiveBls12_381,
};
use sp_ark_bls12_381::{
	fq::Fq as FqOptimized, fq12::Fq12 as Fq12Optimized, fq2::Fq2 as Fq2Optimized,
	fr::Fr as Fr_optimized,
};

bench!(
	Name = "Bls12_381_optimized",
	Pairing = Bls12_381Optimized,
	G1 = G1ProjectiveBls12_381,
	G2 = G2ProjectiveBls12_381,
	ScalarField = FrOptimized,
	G1BaseField = FqOptimized,
	G2BaseField = Fq2Optimized,
	TargetField = Fq12Otimized,
);
