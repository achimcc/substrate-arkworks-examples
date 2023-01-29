use ark_algebra_bench_templates::*;
use pallet_template::bw6_761::{BW6_761Optimized, G1ProjectiveBW6_761, G2ProjectiveBW6_761};
use sp_ark_bw6_761::{
	fq::Fq as FqOptimized, fq3::Fq3 as Fq3Optimized, fq6::Fq6 as Fq6Optimized,
	fr::Fr as FrOptimized,
};

bench!(
	Name = "BW6_761_optimized",
	Pairing = BW6_761Optimized,
	G1 = G1ProjectiveBW6_761,
	G2 = G2ProjectiveBW6_761,
	ScalarField = FrOptimized,
	G1BaseField = FqOptimized,
	G2BaseField = Fq3Optimized,
	TargetField = Fq6Optimized,
);
