# Bechmark results comparison table

| extrinsic                                   |  normal(µs)[^1]  |optimized(µs)[^2]|   wasm(µs)[^3]  |  native(µs)[^4] |
| ------------------------------------------- |  --------------- | --------------- | --------------- | --------------- |
| groth16_verification (bls12_381)            |    26535.30      |    8244.31      |     56980       |      4320       | 
| bls12_381_pairing                           |    8257.70       |    1448.53      |     19040       |      1470       |
| bls12_381_msm_g1, 10 arguments              |    16932.20      |    6869.28      |     737.74      |      73.56      |
| bls12_381_msm_g1, 1000 arguments            |    1313899.30    |    653168.11    |     14880       |      1310       |
| bls12_381_msm_g2, 10 arguments              |    115465.19     |    23583.63     |     1090        |      119.14     |
| bls12_381_msm_g2, 1000 arguments            |    10668568.36   |    2458212.20   |     36540       |      2630       |
| bls12_381_mul_projective_g1                 |    8.00          |    21.96        |     29.70       |      0.53       |
| bls12_381_mul_affine_g1                     |    8.56          |    21.74        |     39.70       |      0.45       |
| bls12_381_mul_projective_g2                 |    16.88         |    27.87        |     37.74       |      1.43       |
| bls12_381_mul_affine_g2                     |    15.87         |    27.71        |     37.31       |      1.43       |
| bls12_377_pairing                           |    10963.00      |    1889.50      |     18660       |      1560       |
| bls12_377_msm_g1, 10 arguments              |    20745.06      |    9270.83      |     576.27      |      73.74      | 
| bls12_377_msm_g1, 1000 arguments            |    1287941.57    |    831275.64    |     14520       |      1610       |
| bls12_377_msm_g2, 10 arguments              |    131852.78     |    34796.36     |     1350        |      170.07     |
| bls12_377_msm_g2, 1000 arguments            |    10196159.70   |    2781007.89   |     37880       |      3860       |
| bls12_377_mul_projective_g1                 |    6.87          |    17.36        |     31.59       |      0.52       |
| bls12_377_mul_affine_g1                     |    6.76          |    16.57        |     61.45       |      0.52       |
| bls12_377_mul_projective_g2                 |    13.80         |    22.24        |     38.19       |      1.69       |
| bls12_377_mul_affine_g2                     |    13.60         |    22.49        |     37.10       |      1.73       |
| bw6_761_pairing                             |    44374.64      |    6002.54      |     87300       |      6950       |
| bw6_761_msm_g1, 10 arguments                |    155393.79     |    53231.17     |     1600        |      155.74     |
| bw6_761_msm_g1, 1000 arguments              |    13384952.55   |    5070669.53   |     51300       |      2950       | 
| bw6_761_msm_g2, 10 arguments                |    141484.94     |    39324.56     |     1450        |      151.83     |
| bw6_761_msm_g2, 1000 arguments              |    12528071.10   |    4732393.47   |     46250       |      2940       |
| bw6_761_mul_projective_g1                   |    17.05         |    53.83        |     44.30       |      1.50       |
| bw6_761_mul_affine_g1                       |    18.47         |    55.10        |     44.28       |      1.52       |
| bw6_761_mul_projective_g2                   |    17.45         |    53.65        |     44.84       |      1.79       |
| bw6_761_mul_affine_g2                       |    17.55         |    54.28        |     44.84       |      1.51       |
| ed_on_bls12_381_msm_sw, 10 arguments        |    6663.28       |    3686.07      |     345.06      |      58.88      |
| ed_on_bls12_381_msm_sw, 1000 arguments      |    296140.25     |    215932.66    |     8320        |      1140       |
| ed_on_bls12_381_mul_projective_sw           |    5.57          |    10.08        |     24.89       |      0.30       |
| ed_on_bls12_381_mul_affine_sw               |    5.51          |    10.12        |     36.63       |      0.30       |
| ed_on_bls12_381_msm_te, 10 arguments        |    7813.27       |    3207.47      |     6540        |      406.76     |
| ed_on_bls12_381_msm_te, 1000 arguments      |    334199.35     |    242277.02    |     72860       |      3070       |
| ed_on_bls12_381_mul_projective_te           |    9.13          |    10.60        |     27.47       |      0.74       |  
| ed_on_bls12_381_mul_affine_te               |    5.59          |    10.07        |     30.05       |      0.29       |
| ed_on_bls12_377_msm, 10 arguments           |    7768.41       |    3192.99      |     6070        |      405.37     | 
| ed_on_bls12_377_msm, 1000 arguments         |    357890.37     |    267844.08    |     65890       |      2850       |
| ed_on_bls12_377_mul_projective              |    9.41          |    10.32        |     27.30       |      0.72       |
| ed_on_bls12_377_mul_affine                  |    8.84          |    442.80       |     6040        |      280.58     |

[^1]: implemented in a Substrate pallet with [arkworks](https://github.com/arkworks-rs/) library by this repo: https://github.com/achimcc/substrate-arkworks-examples
[^2]: implemented in a Substrate pallet with [ark-substrate](https://github.com/paritytech/ark-substrate) library, executed through host-function call, computed by this repo: https://github.com/achimcc/substrate-arkworks-examples
[^3]: executed through wasmtime by this repo: https://github.com/achimcc/native-bench-arkworks
[^4]: native execution, computed by this repo: https://github.com/achimcc/native-bench-arkworks