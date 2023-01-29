use ark_algebra_bench_templates::*;
use pallet_template::bls12_381::{
	Bls12_381Optimized, G1ProjectiveBls12_381_Host, G2ProjectiveBls12_381_Host,
};
use sp_ark_bls12_381::{
	fq::Fq as Fq_optimized, fq12::Fq12 as Fq12_optimized, fq2::Fq2 as Fq2_optimized,
	fr::Fr as Fr_optimized,
};

bench!(
	Name = "Bls12_381_optimized",
	Pairing = Bls12_381_optimied,
	G1 = G1ProjectiveBls12_381_Host
	G2 = G2ProjectiveBls12_381_Host,
	ScalarField = Fr_optimized,
	G1BaseField = Fq_optimized,
	G2BaseField = Fq2_optimized,
	TargetField = Fq12_optimized,
);
