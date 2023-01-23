# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [Fibonacci](#fibonacci)
    - [bench_pairing_arkworks_bls12_381](#bench_pairing_arkworks_bls12_381)
    - [bench_msm_g1_bls12_381](#bench_msm_g1_bls12_381)
    - [bench_msm_g2_bls12_381](#bench_msm_g2_bls12_381)
    - [bench_mul_affine_g1_bls12_381](#bench_mul_affine_g1_bls12_381)
    - [bench_mul_projective_g1_bls12_381](#bench_mul_projective_g1_bls12_381)
    - [bench_mul_affine_g2_bls12_381](#bench_mul_affine_g2_bls12_381)
    - [bench_mul_projective_g1_bls12_377](#bench_mul_projective_g1_bls12_377)
    - [bench_mul_affine_g1_bls12_377](#bench_mul_affine_g1_bls12_377)
    - [bench_mul_affine_g2_bls12_377](#bench_mul_affine_g2_bls12_377)
    - [bench_msm_g1_bw6_761](#bench_msm_g1_bw6_761)
    - [bench_msm_g2_bw6_761](#bench_msm_g2_bw6_761)
    - [bench_mul_affine_g1_bw6_761](#bench_mul_affine_g1_bw6_761)
    - [bench_msm_ed_on_bls12_377](#bench_msm_ed_on_bls12_377)
    - [bench_mul_affine_ed_on_bls12_377](#bench_mul_affine_ed_on_bls12_377)
    - [bench_mul_projective_ed_on_bls12_377](#bench_mul_projective_ed_on_bls12_377)
    - [bench_msm_ed_on_bls12_381](#bench_msm_ed_on_bls12_381)
    - [bench_mul_affine_ed_on_bls12_381](#bench_mul_affine_ed_on_bls12_381)
    - [bench_mul_projective_ed_on_bls12_381](#bench_mul_projective_ed_on_bls12_381)

## Benchmark Results

### Fibonacci

|        | `verify_groth16_optimized`          | `verify_groth16`               | `mul_projective_g2_bls12_381_optimized`          | `mul_projective_g2_bls12_381`          | `mul_projective_g2_bls12_377_optimized`          | `mul_projective_g2_bls12_377`          | `pairing_arkworks_bls12_377_optimized`          | `pairing_arkworks_bls12_377`          | `mul_projective_g1_bw6_761_optimized`          | `mul_projective_g1_bw6_761`          | `pairing_arkworks_bw6_761_optimized`          | `pairing_arkworks_bw6_761`          | `mul_affine_g2_bw6_761_optimized`          | `mul_affine_g2_bw6_761`           | `mul_projective_g2_bw6_761_optimized`          | `mul_projective_g2_bw6_761`           |
|:-------|:------------------------------------|:-------------------------------|:-------------------------------------------------|:---------------------------------------|:-------------------------------------------------|:---------------------------------------|:------------------------------------------------|:--------------------------------------|:-----------------------------------------------|:-------------------------------------|:----------------------------------------------|:------------------------------------|:-------------------------------------------|:----------------------------------|:-----------------------------------------------|:------------------------------------- |
|        | `15.71 ms` (✅ **1.00x**)            | `9.95 ms` (✅ **1.58x faster**) | `557.73 us` (🚀 **28.17x faster**)                | `236.84 us` (🚀 **66.33x faster**)      | `648.14 us` (🚀 **24.24x faster**)                | `222.93 us` (🚀 **70.47x faster**)      | `2.66 ms` (🚀 **5.91x faster**)                  | `3.19 ms` (🚀 **4.93x faster**)        | `844.74 us` (🚀 **18.60x faster**)              | `234.29 us` (🚀 **67.05x faster**)    | `8.88 ms` (✅ **1.77x faster**)                | `12.40 ms` (✅ **1.27x faster**)     | `968.21 us` (🚀 **16.23x faster**)          | `363.73 us` (🚀 **43.19x faster**) | `831.86 us` (🚀 **18.89x faster**)              | `260.74 us` (🚀 **60.25x faster**)     |

### bench_pairing_arkworks_bls12_381

|        | `pairing_arkworks_bls12_381_optimized`          | `pairing_arkworks_bls12_381`           |
|:-------|:------------------------------------------------|:-------------------------------------- |
|        | `2.65 ms` (✅ **1.00x**)                         | `2.40 ms` (✅ **1.10x faster**)         |

### bench_msm_g1_bls12_381

|        | `msm_g1_bls12_381_optimized`          | `msm_g1_bls12_381`                |
|:-------|:--------------------------------------|:--------------------------------- |
|        | `845.16 us` (✅ **1.00x**)             | `615.38 us` (✅ **1.37x faster**)  |

### bench_msm_g2_bls12_381

|        | `msm_g2_bls12_381_optimized`          | `msm_g2_bls12_381`              |
|:-------|:--------------------------------------|:------------------------------- |
|        | `1.53 ms` (✅ **1.00x**)               | `1.51 ms` (✅ **1.01x faster**)  |

### bench_mul_affine_g1_bls12_381

|        | `mul_affine_g1_bls12_381_optimized`          | `mul_affine_g1_bls12_381`           |
|:-------|:---------------------------------------------|:----------------------------------- |
|        | `351.00 us` (✅ **1.00x**)                    | `539.50 us` (❌ *1.54x slower*)      |

### bench_mul_projective_g1_bls12_381

|        | `mul_projective_g1_bls12_381_optimized`          | `mul_projective_g1_bls12_381`           |
|:-------|:-------------------------------------------------|:--------------------------------------- |
|        | `457.11 us` (✅ **1.00x**)                        | `318.55 us` (✅ **1.43x faster**)        |

### bench_mul_affine_g2_bls12_381

|        | `mul_affine_g2_bls12_381_optimized`          | `mul_affine_g2_bls12_381`           |
|:-------|:---------------------------------------------|:----------------------------------- |
|        | `488.28 us` (✅ **1.00x**)                    | `250.31 us` (🚀 **1.95x faster**)    |

### bench_mul_projective_g1_bls12_377

|        | `mul_projective_g1_bls12_377_optimized`          | `mul_projective_g1_bls12_377`           |
|:-------|:-------------------------------------------------|:--------------------------------------- |
|        | `375.27 us` (✅ **1.00x**)                        | `212.35 us` (✅ **1.77x faster**)        |

### bench_mul_affine_g1_bls12_377

|        | `mul_affine_g1_bls12_377_optimized`          | `mul_affine_g1_bls12_377`           |
|:-------|:---------------------------------------------|:----------------------------------- |
|        | `574.11 us` (✅ **1.00x**)                    | `262.96 us` (🚀 **2.18x faster**)    |

### bench_mul_affine_g2_bls12_377

|        | `mul_affine_g2_bls12_377_optimized`          | `mul_affine_g2_bls12_377`           |
|:-------|:---------------------------------------------|:----------------------------------- |
|        | `649.86 us` (✅ **1.00x**)                    | `225.31 us` (🚀 **2.88x faster**)    |

### bench_msm_g1_bw6_761

|        | `msm_g1_bw6_761_optimized`          | `msm_g1_bw6_761`                  |
|:-------|:------------------------------------|:--------------------------------- |
|        | `1.18 ms` (✅ **1.00x**)             | `389.72 us` (🚀 **3.04x faster**)  |

### bench_msm_g2_bw6_761

|        | `msm_g2_bw6_761_optimized`          | `msm_g2_bw6_761`                  |
|:-------|:------------------------------------|:--------------------------------- |
|        | `790.23 us` (✅ **1.00x**)           | `275.04 us` (🚀 **2.87x faster**)  |

### bench_mul_affine_g1_bw6_761

|        | `mul_affine_g1_bw6_761_optimized`          | `mul_affine_g1_bw6_761`           |
|:-------|:-------------------------------------------|:--------------------------------- |
|        | `873.10 us` (✅ **1.00x**)                  | `204.83 us` (🚀 **4.26x faster**)  |

### bench_msm_ed_on_bls12_377

|        | `msm_ed_on_bls12_377_optimized`          | `msm_ed_on_bls12_377`             |
|:-------|:-----------------------------------------|:--------------------------------- |
|        | `956.21 us` (✅ **1.00x**)                | `916.60 us` (✅ **1.04x faster**)  |

### bench_mul_affine_ed_on_bls12_377

|        | `msm_ed_on_bls12_377_optimized`          | `msm_ed_on_bls12_377`             |
|:-------|:-----------------------------------------|:--------------------------------- |
|        | `369.29 us` (✅ **1.00x**)                | `330.87 us` (✅ **1.12x faster**)  |

### bench_mul_projective_ed_on_bls12_377

|        | `mul_projective_ed_on_bls12_377_optimized`          | `mul_projective_ed_on_bls12_377`           |
|:-------|:----------------------------------------------------|:------------------------------------------ |
|        | `288.05 us` (✅ **1.00x**)                           | `403.50 us` (❌ *1.40x slower*)             |

### bench_msm_ed_on_bls12_381

|        | `msm_ed_on_bls12_381_optimized`          | `msm_ed_on_bls12_381`             |
|:-------|:-----------------------------------------|:--------------------------------- |
|        | `689.08 us` (✅ **1.00x**)                | `521.80 us` (✅ **1.32x faster**)  |

### bench_mul_affine_ed_on_bls12_381

|        | `mul_affine_ed_on_bls12_381_optimized`          | `mul_affine_ed_on_bls12_381`           |
|:-------|:------------------------------------------------|:-------------------------------------- |
|        | `556.55 us` (✅ **1.00x**)                       | `548.53 us` (✅ **1.01x faster**)       |

### bench_mul_projective_ed_on_bls12_381

|        | `mul_projective_ed_on_bls12_381_optimized`          | `mul_projective_ed_on_bls12_381`           |
|:-------|:----------------------------------------------------|:------------------------------------------ |
|        | `665.26 us` (✅ **1.00x**)                           | `538.30 us` (✅ **1.24x faster**)           |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

