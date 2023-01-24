#![feature(custom_test_frameworks)]
#![test_runner(criterion::runner)]
#![feature(test)]
use criterion::{criterion_group, criterion_main, Criterion};
mod bls12_381;
use bls12_381::{
	bench_msm_g1_bls12_381, bench_msm_g2_bls12_381, bench_mul_affine_g1_bls12_381,
	bench_mul_affine_g2_bls12_381, bench_mul_projective_g1_bls12_381,
	bench_mul_projective_g2_bls12_381, bench_pairing_arkworks_bls12_381,
};

mod bls12_377;
use bls12_377::{
	bench_mul_affine_g1_bls12_377, bench_mul_affine_g2_bls12_377,
	bench_mul_projective_g1_bls12_377, bench_mul_projective_g2_bls12_377,
	bench_pairing_arkworks_bls12_377,
};

// mod bw6_761;
// use bw6_761::{
// 	bench_msm_g1_bw6_761, bench_msm_g2_bw6_761, bench_mul_affine_g1_bw6_761,
// 	bench_mul_affine_g2_bw6_761, bench_mul_projective_g1_bw6_761, bench_mul_projective_g2_bw6_761,
// 	bench_pairing_arkworks_bw6_761,
// };

// mod ed_on_bls12_381;
// use ed_on_bls12_381::{
// 	bench_msm_ed_on_bls12_381, bench_mul_affine_ed_on_bls12_381,
// 	bench_mul_projective_ed_on_bls12_381,
// };

// mod ed_on_bls12_377;
// use ed_on_bls12_377::{
// 	bench_msm_ed_on_bls12_377, bench_mul_affine_ed_on_bls12_377,
// 	bench_mul_projective_ed_on_bls12_377,
// };

// mod groth16;
// use groth16::bench_groth16;

// criterion_group! {
// 	name = groth16;
// 	config = Criterion::default();
// 	targets =
// 	bench_groth16
// }

criterion_group! {
	name = bls12_381;
	config = Criterion::default();
	targets =
	bench_pairing_arkworks_bls12_381, bench_msm_g1_bls12_381, bench_msm_g2_bls12_381,
bench_mul_affine_g1_bls12_381, 	bench_mul_projective_g1_bls12_381, bench_mul_affine_g2_bls12_381,
bench_mul_projective_g2_bls12_381, }

criterion_group! {
	name = bls12_377;
	config = Criterion::default();
	targets =
	bench_mul_projective_g1_bls12_377, bench_mul_affine_g1_bls12_377, bench_mul_affine_g2_bls12_377,
	bench_mul_projective_g2_bls12_377, bench_pairing_arkworks_bls12_377,
}

// criterion_group! {
// 	name = blw6_761;
// 	config = Criterion::default();
// 	targets =
// 	bench_msm_g1_bw6_761, bench_msm_g2_bw6_761, bench_mul_affine_g1_bw6_761,
// bench_mul_projective_g1_bw6_761, 	bench_pairing_arkworks_bw6_761, bench_mul_affine_g2_bw6_761,
// bench_mul_projective_g2_bw6_761, }

// criterion_group! {
// 	name = ed_on_bls12_377;
// 	config = Criterion::default();
// 	targets =
// 	bench_msm_ed_on_bls12_377, bench_mul_affine_ed_on_bls12_377,
// bench_mul_projective_ed_on_bls12_377, }

// criterion_group! {
// 	name = ed_on_bls12_381;
// 	config = Criterion::default();
// 	targets =
// 	bench_msm_ed_on_bls12_381, bench_mul_affine_ed_on_bls12_381, bench_mul_projective_ed_on_bls12_381
// }
criterion_main!(bls12_377);
// criterion_main!(groth16, bls12_381, bls12_377, blw6_761, ed_on_bls12_377, ed_on_bls12_381);
