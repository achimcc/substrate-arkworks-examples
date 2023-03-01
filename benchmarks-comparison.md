# Bechmark results comparison table

| extrinsic                                   |  normal(µs)[^1]  |optimized(µs)[^2]|   wasm(µs)[^3]  |  native(µs)[^4] |
| ------------------------------------------- |  --------------- | --------------- | --------------- | --------------- |
| groth16_verification (bls12_381)            |    28790.226     |    8583.25      |     56980       |      4320       | 
| bls12_381_pairing                           |    8727.14       |    1786.77      |     19040       |      1470       |
| bls12_381_msm_g1, 10 arguments              |    386.85        |    109.91       |     737.74      |      73.56      |
| bls12_381_msm_g1, 1000 arguments            |    7658.52       |    3412.56      |     14880       |      1310       |
| bls12_381_msm_g2, 10 arguments              |    691.47        |    137.23       |     1090        |      119.14     |
| bls12_381_msm_g2, 1000 arguments            |    20943.74      |    5935.93      |     36540       |      2630       |
| bls12_381_mul_projective_g1                 |    10.79         |    27.47        |     29.70       |      0.53       |
| bls12_381_mul_affine_g1                     |    8.53          |    21.13        |     39.70       |      0.45       |
| bls12_381_mul_projective_g2                 |    15.68         |    29.69        |     37.74       |      1.43       |
| bls12_381_mul_affine_g2                     |    16.61         |    28.93        |     37.31       |      1.43       |
| bls12_377_pairing                           |    8897.47       |    1848.60      |     18660       |      1560       |
| bls12_377_msm_g1, 10 arguments              |    386.04        |    112.90       |     576.27      |      73.74      | 
| bls12_377_msm_g1, 1000 arguments            |    7606.42       |    3813.22      |     14520       |      1610       |
| bls12_377_msm_g2, 10 arguments              |    739.09        |    159.48       |     1350        |      170.07     |
| bls12_377_msm_g2, 1000 arguments            |    21648.62      |    6461.81      |     37880       |      3860       |
| bls12_377_mul_projective_g1                 |    7.23          |    17.54        |     31.59       |      0.52       |
| bls12_377_mul_affine_g1                     |    6.70          |    16.19        |     61.45       |      0.52       |
| bls12_377_mul_projective_g2                 |    12.99         |    22.23        |     38.19       |      1.69       |
| bls12_377_mul_affine_g2                     |    14.16         |    23.78        |     37.10       |      1.73       |
| bw6_761_pairing                             |    42476.93      |    6077.93      |     87300       |      6950       |
| bw6_761_msm_g1, 10 arguments                |    1219.33       |    260.25       |     1600        |      155.74     |
| bw6_761_msm_g1, 1000 arguments              |    27734.41      |    9746.98      |     51300       |      2950       | 
| bw6_761_msm_g2, 10 arguments                |    1220.31       |    276.68       |     1450        |      151.83     |
| bw6_761_msm_g2, 1000 arguments              |    28499.99      |    9866.41      |     46250       |      2940       |
| bw6_761_mul_projective_g1                   |    17.47         |    54.27        |     44.30       |      1.50       |
| bw6_761_mul_affine_g1                       |    18.66         |    52.85        |     44.28       |      1.52       |
| bw6_761_mul_projective_g2                   |    18.44         |    73.41        |     44.84       |      1.79       |
| bw6_761_mul_affine_g2                       |    22.07         |    77.63        |     44.84       |      1.51       |
| ed_on_bls12_381_msm_sw, 10 arguments        |    279.81        |    58.79        |     345.06      |      58.88      |
| ed_on_bls12_381_msm_sw, 1000 arguments      |    4684.732      |    2470.43      |     8320        |      1140       |
| ed_on_bls12_381_mul_projective_sw           |    5.59          |    10.75        |     24.89       |      0.30       |
| ed_on_bls12_381_mul_affine_sw               |    6.00          |    13.00        |     36.63       |      0.30       |
| ed_on_bls12_381_msm_te, 10 arguments        |    3516.00       |    465.38       |     6540        |      406.76     |
| ed_on_bls12_381_msm_te, 1000 arguments      |    37165.91      |    5952.35      |     72860       |      3070       |
| ed_on_bls12_381_mul_projective_te           |    8.84          |    9.72         |     27.47       |      0.74       |  
| ed_on_bls12_381_mul_affine_te               |    5.25          |    9.64         |     30.05       |      0.29       |
| ed_on_bls12_377_msm, 10 arguments           |    3504.85       |    446.56       |     6070        |      405.37     | 
| ed_on_bls12_377_msm, 1000 arguments         |    37079.82      |    6150.74      |     65890       |      2850       |
| ed_on_bls12_377_mul_projective              |    8.88          |    11.29        |     27.30       |      0.72       |
| ed_on_bls12_377_mul_affine                  |    3585.92       |    437.80       |     6040        |      280.58    d |

[^1]: implemented in a Substrate pallet with [arkworks](https://github.com/arkworks-rs/) library by this repo: https://github.com/achimcc/substrate-arkworks-examples
[^2]: implemented in a Substrate pallet with [ark-substrate](https://github.com/paritytech/ark-substrate) library, executed through host-function call, computed by this repo: https://github.com/achimcc/substrate-arkworks-examples
[^3]: executed through wasmtime by this repo: https://github.com/achimcc/native-bench-arkworks
[^4]: native execution, computed by this repo: https://github.com/achimcc/native-bench-arkworks