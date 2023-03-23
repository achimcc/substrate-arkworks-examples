# Bechmark results comparison table


| extrinsic                               |  normal(µs)[^1]  |optimized(µs)[^2]|   speedup[^3]   |  dummy(µs)[^4]  |   wasm(µs)[^5]  |  native(µs)[^6] |
| --------------------------------------- |  --------------- | --------------- | --------------- | --------------- | --------------- | --------------- |
| groth16_verification (bls12_381)        |    28199.79      |    9117.80      |${\color{green}\bf 3.09 \boldsymbol{\times}}$|    5800.99      |     45070       |      5460       | 
| bls12_381_pairing                       |    10402.36      |    1590.62      |${\color{green}\bf 6.54 \boldsymbol{\times}}$|    448.97       |     14140       |      1880       |
| bls12_381_msm_g1, 10 arguments          |    7970.50       |    1122.22      |${\color{green}\bf 7.10 \boldsymbol{\times}}$|    87.63        |     24650       |      1200       |
| bls12_381_msm_g1, 1000 arguments        |    229069.53     |    35833.72     |${\color{green}\bf 6.39 \boldsymbol{\times}}$|    6486.63      |     191000      |      14200      |
| bls12_381_msm_g2, 10 arguments          |    24854.55      |    3284.34      |${\color{green}\bf 7.57 \boldsymbol{\times}}$|    10738.18     |     185240      |      3300       |
| bls12_381_msm_g2, 1000 arguments        |    716298.98     |    101603.89    |${\color{green}\bf 7.05 \boldsymbol{\times}}$|    9896.67      |     14850000    |      31530      |
| bls12_381_mul_projective_g1             |    505.58        |    104.31       |${\color{green}\bf 4.85 \boldsymbol{\times}}$|    12.13        |     19.85       |      64.14      |
| bls12_381_mul_affine_g1                 |    439.51        |    89.42        |${\color{green}\bf 4.92 \boldsymbol{\times}}$|    9.74         |     39.70       |      57.40      |
| bls12_381_mul_projective_g2             |    1498.49       |    231.95       |${\color{green}\bf 6.46 \boldsymbol{\times}}$|    18.22        |     37.74       |      204.95     |
| bls12_381_mul_affine_g2                 |    1255.50       |    201.16       |${\color{green}\bf 6.24 \boldsymbol{\times}}$|    16.41        |     34.40       |      171.65     |
| bls12_377_pairing                       |    8998.99       |    1594.38      |${\color{green}\bf 5.64 \boldsymbol{\times}}$|    16.64        |     15160       |      2220       |
| bls12_377_msm_g1, 10 arguments          |    6710.72       |    950.38       |${\color{green}\bf 7.06 \boldsymbol{\times}}$|    51.48        |     28620       |      1250       | 
| bls12_377_msm_g1, 1000 arguments        |    196176.16     |    30106.65     |${\color{green}\bf 6.52 \boldsymbol{\times}}$|    4484.67      |     1920000     |      14320      |
| bls12_377_msm_g2, 10 arguments          |    22969.00      |    3503.74      |${\color{green}\bf 6.56 \boldsymbol{\times}}$|    89.93        |     162870      |      4040       |
| bls12_377_msm_g2, 1000 arguments        |    698696.46     |    118429.47    |${\color{green}\bf 5.90 \boldsymbol{\times}}$|    7948.46      |     14570000    |      36600      |
| bls12_377_mul_projective_g1             |    504.24        |    89.33        |${\color{green}\bf 5.64 \boldsymbol{\times}}$|    11.42        |     19.38       |      68.65      |
| bls12_377_mul_affine_g1                 |    419.75        |    80.46        |${\color{green}\bf 5.22 \boldsymbol{\times}}$|    11.11        |     24.49       |      58.27      |
| bls12_377_mul_projective_g2             |    1539.78       |    270.16       |${\color{green}\bf 5.70 \boldsymbol{\times}}$|    16.64        |     28.26       |      268.53     |
| bls12_377_mul_affine_g2                 |    1290.96       |    234.93       |${\color{green}\bf 5.50 \boldsymbol{\times}}$|    17.18        |     38.94       |      226.36     |
| bw6_761_pairing                         |    52506.13      |    6905.97      |${\color{green}\bf 7.60 \boldsymbol{\times}}$|    844.10       |     55440       |      8200       |
| bw6_761_msm_g1, 10 arguments            |    47190.40      |    5653.72      |${\color{green}\bf 8.35 \boldsymbol{\times}}$|    161.28       |     206610      |      4140       |
| bw6_761_msm_g1, 1000 arguments          |    1342834.87    |    168826.52    |${\color{green}\bf 7.95 \boldsymbol{\times}}$|    13526.84     |     18010000    |      53420      | 
| bw6_761_msm_g2, 10 arguments            |    47136.15      |    5686.05      |${\color{green}\bf 8.29 \boldsymbol{\times}}$|    161.92       |     212280      |      4220       |
| bw6_761_msm_g2, 1000 arguments          |    1344407.42    |    168580.08    |${\color{green}\bf 7.97 \boldsymbol{\times}}$|    13633.30     |     18020000    |      52750      |
| bw6_761_mul_projective_g1               |    1927.85       |    305.39       |${\color{green}\bf 6.31 \boldsymbol{\times}}$|    21.99        |     34.82       |      251.97     |
| bw6_761_mul_affine_g1                   |    1598.12       |    265.21       |${\color{green}\bf 6.03 \boldsymbol{\times}}$|    21.35        |     35.64       |      209.08     |
| bw6_761_mul_projective_g2               |    1919.98       |    308.60       |${\color{green}\bf 6.22 \boldsymbol{\times}}$|    21.64        |     35.42       |      272.09     |
| bw6_761_mul_affine_g2                   |    1599.12       |    270.36       |${\color{green}\bf 5.91 \boldsymbol{\times}}$|    21.57        |     34.68       |      210.16     |
| ed_on_bls12_381_msm_sw, 10 arguments    |    4139.53       |    678.09       |${\color{green}\bf 6.10 \boldsymbol{\times}}$|    36.30        |     8610        |      455.85     |
| ed_on_bls12_381_msm_sw, 1000 arguments  |    108774.04     |    20241.58     |${\color{green}\bf 5.37 \boldsymbol{\times}}$|    2465.60      |     430700      |      9150       |
| ed_on_bls12_381_mul_projective_sw       |    269.16        |    53.42        |${\color{green}\bf 5.04 \boldsymbol{\times}}$|    6.69         |     24.89       |      39.05      |
| ed_on_bls12_381_mul_affine_sw           |    234.34        |    49.17        |${\color{green}\bf 4.77 \boldsymbol{\times}}$|    6.17         |     36.63       |      35.16      |
| ed_on_bls12_381_msm_te, 10 arguments    |    6124.97       |    891.09       |${\color{green}\bf 6.87 \boldsymbol{\times}}$|    35.21        |     12470       |      801.57     |
| ed_on_bls12_381_msm_te, 1000 arguments  |    122059.27     |    20473.18     |${\color{green}\bf 5.96 \boldsymbol{\times}}$|    2391.21      |     533490      |      9280       |
| ed_on_bls12_381_mul_projective_te       |    217.60        |    45.47        |${\color{green}\bf 4.79 \boldsymbol{\times}}$|    7.69         |     22.37       |      28.83      |  
| ed_on_bls12_381_mul_affine_te           |    224.69        |    47.91        |${\color{green}\bf 4.69 \boldsymbol{\times}}$|    7.61         |     17.25       |      35.11      |
| ed_on_bls12_377_msm, 10 arguments       |    6101.68       |    857.74       |${\color{green}\bf 7.11 \boldsymbol{\times}}$|    43.24        |     10060       |      414.02     | 
| ed_on_bls12_377_msm, 1000 arguments     |    124114.05     |    20309.37     |${\color{green}\bf 6.11 \boldsymbol{\times}}$|    2465.60      |     537810      |      9100       |
| ed_on_bls12_377_mul_projective          |    216.51        |    45.31        |${\color{green}\bf 4.78 \boldsymbol{\times}}$|    7.00         |     22.48       |      28.51      |
| ed_on_bls12_377_mul_affine              |    213.23        |    43.56        |${\color{green}\bf 4.90 \boldsymbol{\times}}$|    8.47         |     22.34       |      28.42      |

[^1]: implemented in a Substrate pallet with [arkworks](https://github.com/arkworks-rs/) library by this repo: https://github.com/achimcc/substrate-arkworks-examples
[^2]: implemented in a Substrate pallet with [ark-substrate](https://github.com/paritytech/ark-substrate) library, executed through host-function call, computed by this repo: https://github.com/achimcc/substrate-arkworks-examples
[^3]: speedup by using ark-substrate and host calls, compared to native speed
[^4]: These extrinsics just receive the arguemnts, deserialize them without using them and then take a generator or zero element of the expected return group, serizlize it and return it. **Calling a host call through a extrinsic which does nothing has been benchmarked with 3.98µs**. Implementation in: https://github.com/achimcc/substrate-arkworks-examples/tree/dummy-calls
[^5]: executed through wasmtime by this repo: [https://github.com/achimcc/native-bench-arkworks](https://github.com/achimcc/wasm-bench-arkworks)
[^6]: native execution, computed by this repo: https://github.com/achimcc/native-bench-arkworks
