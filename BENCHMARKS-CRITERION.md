# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [groth16](#groth16)
    - [pairing_bls12_381](#pairing_bls12_381)
    - [msm_g1_bls12_381](#msm_g1_bls12_381)
    - [msm_g2_bls12_381](#msm_g2_bls12_381)
    - [mul_affine_g1_bls12_381](#mul_affine_g1_bls12_381)
    - [mul_projective_g1_bls12_381](#mul_projective_g1_bls12_381)
    - [mul_affine_g2_bls12_381](#mul_affine_g2_bls12_381)
    - [mul_projective_g2_bls12_381](#mul_projective_g2_bls12_381)
    - [mul_affine_g1_bls12_377](#mul_affine_g1_bls12_377)
    - [mul_affine_g2_bls12_377](#mul_affine_g2_bls12_377)
    - [mul_projective_g1_bls12_377](#mul_projective_g1_bls12_377)
    - [mul_projective_g2_bls12_377](#mul_projective_g2_bls12_377)
    - [pairing_bls12_377](#pairing_bls12_377)
    - [pairing_msm_g1_bls12_377](#pairing_msm_g1_bls12_377)
    - [pairing_msm_g2_bls12_377](#pairing_msm_g2_bls12_377)
    - [msm_g1_bw6_761](#msm_g1_bw6_761)
    - [msm_g2_bw6_761](#msm_g2_bw6_761)
    - [mul_affine_g1_bw6_761](#mul_affine_g1_bw6_761)
    - [mul_projective_g1_bw6_761](#mul_projective_g1_bw6_761)
    - [pairing_bw6_761](#pairing_bw6_761)
    - [mul_affine_g2_bw6_761](#mul_affine_g2_bw6_761)
    - [mul_projective_g2_bw6_761](#mul_projective_g2_bw6_761)
    - [msm_10_ed_on_bls12_377](#msm_10_ed_on_bls12_377)
    - [mul_affine_ed_on_bls12_377](#mul_affine_ed_on_bls12_377)
    - [mul_projective_ed_on_bls12_377](#mul_projective_ed_on_bls12_377)
    - [msm_sw_ed_on_bls12_381](#msm_sw_ed_on_bls12_381)
    - [mul_affine_sw_ed_on_bls12_381](#mul_affine_sw_ed_on_bls12_381)
    - [mul_projective_sw_ed_on_bls12_381](#mul_projective_sw_ed_on_bls12_381)
    - [msm_te_10_ed_on_bls12_381](#msm_te_10_ed_on_bls12_381)
    - [mul_affine_te_ed_on_bls12_381](#mul_affine_te_ed_on_bls12_381)
    - [mul_projective_te_ed_on_bls12_381](#mul_projective_te_ed_on_bls12_381)

## Benchmark Results

### groth16

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `5.39 ms` (✅ **1.00x**) | `6.22 ms` (❌ *1.15x slower*)    |

### pairing_bls12_381

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `1.73 ms` (✅ **1.00x**) | `1.95 ms` (❌ *1.12x slower*)    |

### msm_g1_bls12_381

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `66.44 us` (✅ **1.00x**)        | `80.06 us` (❌ *1.21x slower*)      | `1.70 ms` (❌ *25.55x slower*)     | `2.17 ms` (❌ *32.63x slower*)         |

### msm_g2_bls12_381

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `105.37 us` (✅ **1.00x**)       | `118.61 us` (❌ *1.13x slower*)     | `3.71 ms` (❌ *35.24x slower*)     | `4.56 ms` (❌ *43.28x slower*)         |

### mul_affine_g1_bls12_381

|        | `normal`                  | `optimized`                      |
|:-------|:--------------------------|:-------------------------------- |
|        | `622.52 ns` (✅ **1.00x**) | `9.13 us` (❌ *14.67x slower*)    |

### mul_projective_g1_bls12_381

|        | `normal`                  | `optimized`                      |
|:-------|:--------------------------|:-------------------------------- |
|        | `587.45 ns` (✅ **1.00x**) | `9.18 us` (❌ *15.63x slower*)    |

### mul_affine_g2_bls12_381

|        | `normal`                | `optimized`                      |
|:-------|:------------------------|:-------------------------------- |
|        | `1.58 us` (✅ **1.00x**) | `11.51 us` (❌ *7.27x slower*)    |

### mul_projective_g2_bls12_381

|        | `normal`                | `optimized`                      |
|:-------|:------------------------|:-------------------------------- |
|        | `1.58 us` (✅ **1.00x**) | `11.56 us` (❌ *7.33x slower*)    |

### mul_affine_g1_bls12_377

|        | `normal`                  | `optimized`                      |
|:-------|:--------------------------|:-------------------------------- |
|        | `582.62 ns` (✅ **1.00x**) | `9.20 us` (❌ *15.80x slower*)    |

### mul_affine_g2_bls12_377

|        | `normal`                | `optimized`                      |
|:-------|:------------------------|:-------------------------------- |
|        | `1.87 us` (✅ **1.00x**) | `12.26 us` (❌ *6.54x slower*)    |

### mul_projective_g1_bls12_377

|        | `normal`                  | `optimized`                      |
|:-------|:--------------------------|:-------------------------------- |
|        | `590.58 ns` (✅ **1.00x**) | `9.23 us` (❌ *15.64x slower*)    |

### mul_projective_g2_bls12_377

|        | `normal`                | `optimized`                      |
|:-------|:------------------------|:-------------------------------- |
|        | `1.87 us` (✅ **1.00x**) | `12.37 us` (❌ *6.62x slower*)    |

### pairing_bls12_377

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `2.04 ms` (✅ **1.00x**) | `2.05 ms` (✅ **1.00x slower**)  |

### pairing_msm_g1_bls12_377

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `65.44 us` (✅ **1.00x**)        | `81.05 us` (❌ *1.24x slower*)      | `1.70 ms` (❌ *25.99x slower*)     | `2.29 ms` (❌ *34.93x slower*)         |

### pairing_msm_g2_bls12_377

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `108.55 us` (✅ **1.00x**)       | `128.74 us` (❌ *1.19x slower*)     | `4.29 ms` (❌ *39.48x slower*)     | `5.32 ms` (❌ *48.97x slower*)         |

### msm_g1_bw6_761

|        | `normal, 10 arguments`          | `optimized,, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:------------------------------------|:----------------------------------|:------------------------------------- |
|        | `147.16 us` (✅ **1.00x**)       | `180.46 us` (❌ *1.23x slower*)      | `4.57 ms` (❌ *31.06x slower*)     | `6.09 ms` (❌ *41.39x slower*)         |

### msm_g2_bw6_761

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `144.06 us` (✅ **1.00x**)       | `181.34 us` (❌ *1.26x slower*)     | `4.60 ms` (❌ *31.96x slower*)     | `6.10 ms` (❌ *42.33x slower*)         |

### mul_affine_g1_bw6_761

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `1.95 us` (✅ **1.00x**) | `42.77 us` (❌ *21.92x slower*)    |

### mul_projective_g1_bw6_761

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `1.95 us` (✅ **1.00x**) | `42.95 us` (❌ *21.99x slower*)    |

### pairing_bw6_761

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `7.56 ms` (✅ **1.00x**) | `7.57 ms` (✅ **1.00x slower**)  |

### mul_affine_g2_bw6_761

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `1.95 us` (✅ **1.00x**) | `42.34 us` (❌ *21.72x slower*)    |

### mul_projective_g2_bw6_761

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `1.95 us` (✅ **1.00x**) | `42.38 us` (❌ *21.77x slower*)    |

### msm_10_ed_on_bls12_377

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `562.15 us` (✅ **1.00x**)       | `567.62 us` (✅ **1.01x slower**)   | `6.39 ms` (❌ *11.37x slower*)     | `6.73 ms` (❌ *11.97x slower*)         |

### mul_affine_ed_on_bls12_377

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `549.96 us` (✅ **1.00x**) | `553.41 us` (✅ **1.01x slower**)  |

### mul_projective_ed_on_bls12_377

|        | `normal`                  | `optimized`                     |
|:-------|:--------------------------|:------------------------------- |
|        | `959.91 ns` (✅ **1.00x**) | `4.60 us` (❌ *4.79x slower*)    |

### msm_sw_ed_on_bls12_381

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `28.17 us` (✅ **1.00x**)        | `36.02 us` (❌ *1.28x slower*)      | `1.05 ms` (❌ *37.42x slower*)     | `1.42 ms` (❌ *50.57x slower*)         |

### mul_affine_sw_ed_on_bls12_381

|        | `normal`                  | `optimized`                     |
|:-------|:--------------------------|:------------------------------- |
|        | `429.06 ns` (✅ **1.00x**) | `4.14 us` (❌ *9.64x slower*)    |

### mul_projective_sw_ed_on_bls12_381

|        | `normal`                  | `optimized`                     |
|:-------|:--------------------------|:------------------------------- |
|        | `415.19 ns` (✅ **1.00x**) | `4.14 us` (❌ *9.97x slower*)    |

### msm_te_10_ed_on_bls12_381

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `28.06 us` (✅ **1.00x**)        | `35.95 us` (❌ *1.28x slower*)      | `6.42 ms` (❌ *228.61x slower*)    | `6.81 ms` (❌ *242.78x slower*)        |

### mul_affine_te_ed_on_bls12_381

|        | `normal`                  | `optimized`                     |
|:-------|:--------------------------|:------------------------------- |
|        | `429.09 ns` (✅ **1.00x**) | `4.14 us` (❌ *9.64x slower*)    |

### mul_projective_te_ed_on_bls12_381

|        | `normal`                  | `optimized`                     |
|:-------|:--------------------------|:------------------------------- |
|        | `974.75 ns` (✅ **1.00x**) | `4.65 us` (❌ *4.77x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

