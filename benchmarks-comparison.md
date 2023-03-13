# Bechmark results comparison table

| extrinsic                                   |  normal(µs)[^1]  |optimized(µs)[^2]|  dummy(µs)[^3]  |   wasm(µs)[^4]  |  native(µs)[^5] |
| ------------------------------------------- |  --------------- | --------------- | --------------- | --------------- | --------------- |
| groth16_verification (bls12_381)            |    26535.30      |    8244.31$${\color{green}Green}$$      |    5800.99      |     45070       |      4040       | 
| bls12_381_pairing                           |    8257.70       |    1448.53      |    448.97       |     14140       |      1350       |
| bls12_381_msm_g1, 10 arguments              |    16932.20      |    6869.28      |    87.63        |     24650       |      600.44     |
| bls12_381_msm_g1, 1000 arguments            |    1313899.30    |    653168.11    |    6486.63      |     191000      |      11160      |
| bls12_381_msm_g2, 10 arguments              |    115465.19     |    23583.63     |    10738.18     |     185240      |      1660       |
| bls12_381_msm_g2, 1000 arguments            |    10668568.36   |    2458212.20   |    9896.67      |     14850000    |      33420      |
| bls12_381_mul_projective_g1[^*]             |    8.00          |    21.96        |    12.13        |     19.85       |      0.45       |
| bls12_381_mul_affine_g1[^*]                 |    8.56          |    21.74        |    9.74         |     39.70       |      0.45       |
| bls12_381_mul_projective_g2[^*]             |    16.88         |    27.87        |    18.22        |     37.74       |      1.18       |
| bls12_381_mul_affine_g2[^*]                 |    15.87         |    27.71        |    16.41        |     34.40       |      1.19       |
| bls12_377_pairing                           |    10963.00      |    1889.50      |    16.64        |     15160       |      1520       |
| bls12_377_msm_g1, 10 arguments              |    20745.06      |    9270.83      |    51.48        |     28620       |      559.16     | 
| bls12_377_msm_g1, 1000 arguments            |    1287941.57    |    831275.64    |    4484.67      |     1920000     |      11160      |
| bls12_377_msm_g2, 10 arguments              |    131852.78     |    34796.36     |    89.93        |     162870      |      2020       |
| bls12_377_msm_g2, 1000 arguments            |    10196159.70   |    2781007.89   |    7948.46      |     14570000    |      40410      |
| bls12_377_mul_projective_g1[^*]             |    6.87          |    17.36        |    11.42        |     19.38       |      0.44       |
| bls12_377_mul_affine_g1[^*]                 |    6.76          |    16.57        |    11.11        |     24.49       |      0.45       |
| bls12_377_mul_projective_g2[^*]             |    13.80         |    22.24        |    16.64        |     28.26       |      1.42       |
| bls12_377_mul_affine_g2[^*]                 |    13.60         |    22.49        |    17.18        |     38.94       |      1.46       |
| bw6_761_pairing                             |    44374.64      |    6002.54      |    844.10       |     55440       |      6940       |
| bw6_761_msm_g1, 10 arguments                |    155393.79     |    53231.17     |    161.28       |     206610      |      3490       |
| bw6_761_msm_g1, 1000 arguments              |    13384952.55   |    5070669.53   |    13526.84     |     18010000    |      75270      | 
| bw6_761_msm_g2, 10 arguments                |    141484.94     |    39324.56     |    161.92       |     212280      |      3430       |
| bw6_761_msm_g2, 1000 arguments              |    12528071.10   |    4732393.47   |    13633.30     |     18020000    |      75330      |
| bw6_761_mul_projective_g1[^*]               |    17.05         |    53.83        |    21.99        |     34.82       |      1.79       |
| bw6_761_mul_affine_g1[^*]                   |    18.47         |    55.10        |    21.35        |     35.64       |      1.77       |
| bw6_761_mul_projective_g2[^*]               |    17.45         |    53.65        |    21.64        |     35.42       |      1.78       |
| bw6_761_mul_affine_g2[^*]                   |    17.55         |    54.28        |    21.57        |     34.68       |      1.78       |
| ed_on_bls12_381_msm_sw, 10 arguments        |    6663.28       |    3686.07      |    36.30        |     8610        |      376.61     |
| ed_on_bls12_381_msm_sw, 1000 arguments      |    296140.25     |    215932.66    |    2465.60      |     430700      |      6010       |
| ed_on_bls12_381_mul_projective_sw[^*]       |    5.57          |    10.08        |    6.69         |     24.89       |      0.36       |
| ed_on_bls12_381_mul_affine_sw[^*]           |    5.51          |    10.12        |    6.17         |     36.63       |      0.36       |
| ed_on_bls12_381_msm_te, 10 arguments        |    7813.27       |    3207.47      |    35.21        |     12470       |      560.82     |
| ed_on_bls12_381_msm_te, 1000 arguments      |    334199.35     |    242277.02    |    2391.21      |     533490      |      7890       |
| ed_on_bls12_381_mul_projective_te[^*]       |    9.13          |    10.60        |    7.69         |     22.37       |      0.83       |  
| ed_on_bls12_381_mul_affine_te[^*]           |    5.59          |    10.07        |    7.61         |     17.25       |      0.37       |
| ed_on_bls12_377_msm, 10 arguments           |    7768.41       |    3192.99      |    43.24        |     10060       |      553.69     | 
| ed_on_bls12_377_msm, 1000 arguments         |    357890.37     |    267844.08    |    2465.60      |     537810      |      7680       |
| ed_on_bls12_377_mul_projective[^*]          |    9.41          |    10.32        |    7.00         |     22.48       |      0.89       |
| ed_on_bls12_377_mul_affine[^*]              |    8.84          |    442.80       |    8.47         |     22.34       |      0.86       |

[^1]: implemented in a Substrate pallet with [arkworks](https://github.com/arkworks-rs/) library by this repo: https://github.com/achimcc/substrate-arkworks-examples
[^2]: implemented in a Substrate pallet with [ark-substrate](https://github.com/paritytech/ark-substrate) library, executed through host-function call, computed by this repo: https://github.com/achimcc/substrate-arkworks-examples
[^3]: These extrinsics just receive the arguemnts, deserialize them without using them and then take a generator or zero element of the expected return group, serizlize it and return it. **Calling a host call through a extrinsic which does nothing has been benchmarked with 3.98µs**. Implementation in: https://github.com/achimcc/substrate-arkworks-examples/tree/dummy-calls
[^4]: executed through wasmtime by this repo: [https://github.com/achimcc/native-bench-arkworks](https://github.com/achimcc/wasm-bench-arkworks)
[^5]: native execution, computed by this repo: https://github.com/achimcc/native-bench-arkworks
[^*]: we removed these host calls in the final ark-substrate implementation, since they didn't yield a performance improvement. Implementations can be found in the branches: https://github.com/paritytech/ark-substrate/tree/arkworks-host-function-mul and https://github.com/paritytech/substrate/tree/arkworks-host-function-mul
