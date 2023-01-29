use ark_algebra_bench_templates::*;
use pallet_template::bw6_761::{BW6_761Optimized, G1ProjectiveBW6_761, G2ProjectiveBW6_761};
use sp_ark_bw6_761::{
	fq::Fq as Fq_optimized, fq3::Fq3 as Fq3_optimized, fq6::Fq6 as Fq6_optimized,
	fr::Fr as Fr_optimized,
};

bench!(
	Name = "BW6_761_optimized",
	Pairing = BW6_761Optimized,
	G1 = G1ProjectiveBW6_761,
	G2 = G2ProjectiveBW6_761,
	ScalarField = Fr_optimized,
	G1BaseField = Fq_optimized,
	G2BaseField = Fq3_optimized,
	TargetField = Fq6_optimized,
);
