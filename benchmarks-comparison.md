# Bechmark results comparison table


| extrinsic                               |  normal(µs)[^1]  |optimized(µs)[^2]|   speedup[^3]   |  dummy(µs)[^4]  |   wasm(µs)[^5]  |  native(µs)[^6] |
| --------------------------------------- |  --------------- | --------------- | --------------- | --------------- | --------------- | --------------- |
| groth16_verification (bls12_381)        |    26535.30      |    8244.31      |${\color{green}\bf 3.22 \boldsymbol{\times}}$|    5800.99      |     45070       |      5460       | 
| bls12_381_pairing                       |    10401.82      |    1642.82      |${\color{green}\bf 6.33 \boldsymbol{\times}}$|    448.97       |     14140       |      1880       |
| bls12_381_msm_g1, 10 arguments          |    16932.20      |    6869.28      |${\color{green}\bf 2.46 \boldsymbol{\times}}$|    87.63        |     24650       |      1200       |
| bls12_381_msm_g1, 1000 arguments        |    1313899.30    |    653168.11    |${\color{green}\bf 2.01 \boldsymbol{\times}}$|    6486.63      |     191000      |      14200      |
| bls12_381_msm_g2, 10 arguments          |    115465.19     |    23583.63     |${\color{green}\bf 4.90 \boldsymbol{\times}}$|    10738.18     |     185240      |      3300       |
| bls12_381_msm_g2, 1000 arguments        |    10668568.36   |    2458212.20   |${\color{green}\bf 4.34 \boldsymbol{\times}}$|    9896.67      |     14850000    |      31530      |
| bls12_381_mul_projective_g1             |    507.53        |    94.18        |${\color{green}\bf 5.39 \boldsymbol{\times}}$|    12.13        |     19.85       |      64.14      |
| bls12_381_mul_affine_g1                 |    455.39        |    95.40        |${\color{green}\bf 4.77 \boldsymbol{\times}}$|    9.74         |     39.70       |      57.40      |
| bls12_381_mul_projective_g2             |    1584.11       |    238.31       |${\color{green}\bf 6.65 \boldsymbol{\times}}$|    18.22        |     37.74       |      204.95     |
| bls12_381_mul_affine_g2                 |    1285.77       |    207.74       |${\color{green}\bf 6.19 \boldsymbol{\times}}$|    16.41        |     34.40       |      171.65     |
| bls12_377_pairing                       |    9076.44       |    1641.00      |${\color{green}\bf 5.53 \boldsymbol{\times}}$|    16.64        |     15160       |      2220       |
| bls12_377_msm_g1, 10 arguments          |    20745.06      |    9270.83      |${\color{green}\bf 2.24 \boldsymbol{\times}}$|    51.48        |     28620       |      1250       | 
| bls12_377_msm_g1, 1000 arguments        |    1287941.57    |    831275.64    |${\color{green}\bf 1.55 \boldsymbol{\times}}$|    4484.67      |     1920000     |      14320      |
| bls12_377_msm_g2, 10 arguments          |    131852.78     |    34796.36     |${\color{green}\bf 3.79 \boldsymbol{\times}}$|    89.93        |     162870      |      4040       |
| bls12_377_msm_g2, 1000 arguments        |    10196159.70   |    2781007.89   |${\color{green}\bf 3.67 \boldsymbol{\times}}$|    7948.46      |     14570000    |      36600      |
| bls12_377_mul_projective_g1             |    511.28        |    100.63       |${\color{green}\bf 5.08 \boldsymbol{\times}}$|    11.42        |     19.38       |      68.65      |
| bls12_377_mul_affine_g1                 |    459.98        |    89.74        |${\color{green}\bf 5.13 \boldsymbol{\times}}$|    11.11        |     24.49       |      58.27      |
| bls12_377_mul_projective_g2             |    1625.11       |    290.28       |${\color{green}\bf 5.60 \boldsymbol{\times}}$|    16.64        |     28.26       |      268.53     |
| bls12_377_mul_affine_g2                 |    1346.71       |    243.37       |${\color{green}\bf 5.53 \boldsymbol{\times}}$|    17.18        |     38.94       |      226.36     |
| bw6_761_pairing                         |    52427.45      |    6999.06      |${\color{green}\bf 7.49 \boldsymbol{\times}}$|    844.10       |     55440       |      8200       |
| bw6_761_msm_g1, 10 arguments            |    155393.79     |    53231.17     |${\color{green}\bf 2.92 \boldsymbol{\times}}$|    161.28       |     206610      |      4140       |
| bw6_761_msm_g1, 1000 arguments          |    13384952.55   |    5070669.53   |${\color{green}\bf 2.64 \boldsymbol{\times}}$|    13526.84     |     18010000    |      53420      | 
| bw6_761_msm_g2, 10 arguments            |    141484.94     |    39324.56     |${\color{green}\bf 3.60 \boldsymbol{\times}}$|    161.92       |     212280      |      4220       |
| bw6_761_msm_g2, 1000 arguments          |    12528071.10   |    4732393.47   |${\color{green}\bf 2.65 \boldsymbol{\times}}$|    13633.30     |     18020000    |      52750      |
| bw6_761_mul_projective_g1               |    1972.01       |    315.07       |${\color{green}\bf 6.26 \boldsymbol{\times}}$|    21.99        |     34.82       |      251.97     |
| bw6_761_mul_affine_g1                   |    1641.31       |    272.15       |${\color{green}\bf 6.03 \boldsymbol{\times}}$|    21.35        |     35.64       |      209.08     |
| bw6_761_mul_projective_g2               |    1969.34       |    314.97       |${\color{green}\bf 6.25 \boldsymbol{\times}}$|    21.64        |     35.42       |      272.09     |
| bw6_761_mul_affine_g2                   |    1641.21       |    273.36       |${\color{green}\bf 6.00 \boldsymbol{\times}}$|    21.57        |     34.68       |      210.16     |
| ed_on_bls12_381_msm_sw, 10 arguments    |    6663.28       |    3686.07      |${\color{green}\bf 1.81 \boldsymbol{\times}}$|    36.30        |     8610        |      455.85     |
| ed_on_bls12_381_msm_sw, 1000 arguments  |    296140.25     |    215932.66    |${\color{green}\bf 1.37 \boldsymbol{\times}}$|    2465.60      |     430700      |      9150       |
| ed_on_bls12_381_mul_projective_sw       |    297.07        |    55.11        |${\color{green}\bf 5.30 \boldsymbol{\times}}$|    6.69         |     24.89       |      39.05      |
| ed_on_bls12_381_mul_affine_sw           |    241.37        |    54.09        |${\color{green}\bf 4.46 \boldsymbol{\times}}$|    6.17         |     36.63       |      35.16      |
| ed_on_bls12_381_msm_te, 10 arguments    |    7813.27       |    3207.47      |${\color{green}\bf 2.44 \boldsymbol{\times}}$|    35.21        |     12470       |      801.57     |
| ed_on_bls12_381_msm_te, 1000 arguments  |    334199.35     |    242277.02    |${\color{green}\bf 1.38 \boldsymbol{\times}}$|    2391.21      |     533490      |      9280       |
| ed_on_bls12_381_mul_projective_te       |    216.63        |    46.84        |${\color{green}\bf 4,62 \boldsymbol{\times}}$|    7.69         |     22.37       |      28.83      |  
| ed_on_bls12_381_mul_affine_te           |    224.69        |    47.91        |${\color{green}\bf 4.69 \boldsymbol{\times}}$|    7.61         |     17.25       |      35.11      |
| ed_on_bls12_377_msm, 10 arguments       |    7768.41       |    3192.99      |${\color{green}\bf 2.43 \boldsymbol{\times}}$|    43.24        |     10060       |      414.02     | 
| ed_on_bls12_377_msm, 1000 arguments     |    357890.37     |    267844.08    |${\color{green}\bf 1.34 \boldsymbol{\times}}$|    2465.60      |     537810      |      9100       |
| ed_on_bls12_377_mul_projective          |    218.20        |    45.03        |${\color{green}\bf 4.85 \boldsymbol{\times}}$|    7.00         |     22.48       |      28.51      |
| ed_on_bls12_377_mul_affine              |    222.64        |    45.84        |${\color{green}\bf 4.86 \boldsymbol{\times}}$|    8.47         |     22.34       |      28.42      |

[^1]: implemented in a Substrate pallet with [arkworks](https://github.com/arkworks-rs/) library by this repo: https://github.com/achimcc/substrate-arkworks-examples
[^2]: implemented in a Substrate pallet with [ark-substrate](https://github.com/paritytech/ark-substrate) library, executed through host-function call, computed by this repo: https://github.com/achimcc/substrate-arkworks-examples
[^3]: speedup by using ark-substrate and host calls, compared to native speed
[^4]: These extrinsics just receive the arguemnts, deserialize them without using them and then take a generator or zero element of the expected return group, serizlize it and return it. **Calling a host call through a extrinsic which does nothing has been benchmarked with 3.98µs**. Implementation in: https://github.com/achimcc/substrate-arkworks-examples/tree/dummy-calls
[^5]: executed through wasmtime by this repo: [https://github.com/achimcc/native-bench-arkworks](https://github.com/achimcc/wasm-bench-arkworks)
[^6]: native execution, computed by this repo: https://github.com/achimcc/native-bench-arkworks
