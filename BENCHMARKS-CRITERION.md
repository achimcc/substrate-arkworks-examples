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
    - [msm_ed_on_bls12_377](#msm_ed_on_bls12_377)
    - [mul_affine_ed_on_bls12_377](#mul_affine_ed_on_bls12_377)
    - [mul_projective_ed_on_bls12_377](#mul_projective_ed_on_bls12_377)
    - [msm_ed_on_bls12_381](#msm_ed_on_bls12_381)
    - [mul_affine_ed_on_bls12_381](#mul_affine_ed_on_bls12_381)
    - [mul_projective_ed_on_bls12_381](#mul_projective_ed_on_bls12_381)

## Benchmark Results

### groth16

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `5.39 ms` (✅ **1.00x**) | `6.23 ms` (❌ *1.16x slower*)    |

### pairing_bls12_381

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `1.72 ms` (✅ **1.00x**) | `1.95 ms` (❌ *1.13x slower*)    |

### msm_g1_bls12_381

|        | `normal`               | `optimized`                    |
|:-------|:-----------------------|:------------------------------ |
|        | `1.15 s` (✅ **1.00x**) | `1.17 s` (✅ **1.02x slower**)  |

### msm_g2_bls12_381

|        | `normal`               | `optimized`                    |
|:-------|:-----------------------|:------------------------------ |
|        | `3.37 s` (✅ **1.00x**) | `3.43 s` (✅ **1.02x slower**)  |

### mul_affine_g1_bls12_381

|        | `normal`                  | `optimized`                      |
|:-------|:--------------------------|:-------------------------------- |
|        | `624.82 ns` (✅ **1.00x**) | `9.06 us` (❌ *14.50x slower*)    |

### mul_projective_g1_bls12_381

|        | `normal`                  | `optimized`                      |
|:-------|:--------------------------|:-------------------------------- |
|        | `592.52 ns` (✅ **1.00x**) | `9.17 us` (❌ *15.47x slower*)    |

### mul_affine_g2_bls12_381

|        | `normal`                | `optimized`                      |
|:-------|:------------------------|:-------------------------------- |
|        | `1.62 us` (✅ **1.00x**) | `11.53 us` (❌ *7.13x slower*)    |

### mul_projective_g2_bls12_381

|        | `normal`                | `optimized`                      |
|:-------|:------------------------|:-------------------------------- |
|        | `1.58 us` (✅ **1.00x**) | `11.58 us` (❌ *7.31x slower*)    |

### mul_affine_g1_bls12_377

|        | `normal`                  | `optimized`                      |
|:-------|:--------------------------|:-------------------------------- |
|        | `587.72 ns` (✅ **1.00x**) | `9.28 us` (❌ *15.79x slower*)    |

### mul_affine_g2_bls12_377

|        | `normal`                | `optimized`                      |
|:-------|:------------------------|:-------------------------------- |
|        | `1.90 us` (✅ **1.00x**) | `12.37 us` (❌ *6.50x slower*)    |

### mul_projective_g1_bls12_377

|        | `normal`                  | `optimized`                      |
|:-------|:--------------------------|:-------------------------------- |
|        | `580.57 ns` (✅ **1.00x**) | `9.28 us` (❌ *15.98x slower*)    |

### mul_projective_g2_bls12_377

|        | `normal`                | `optimized`                      |
|:-------|:------------------------|:-------------------------------- |
|        | `1.87 us` (✅ **1.00x**) | `12.48 us` (❌ *6.66x slower*)    |

### pairing_bls12_377

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `2.04 ms` (✅ **1.00x**) | `2.05 ms` (✅ **1.01x slower**)  |

### pairing_msm_g1_bls12_377

|        | `normal`               | `optimized`                    |
|:-------|:-----------------------|:------------------------------ |
|        | `1.14 s` (✅ **1.00x**) | `1.18 s` (✅ **1.03x slower**)  |

### pairing_msm_g2_bls12_377

|        | `normal`               | `optimized`                    |
|:-------|:-----------------------|:------------------------------ |
|        | `3.98 s` (✅ **1.00x**) | `4.08 s` (✅ **1.03x slower**)  |

### msm_g1_bw6_761

|        | `normal`                 | `optimized`                       |
|:-------|:-------------------------|:--------------------------------- |
|        | `89.77 us` (✅ **1.00x**) | `134.37 us` (❌ *1.50x slower*)    |

### msm_g2_bw6_761

|        | `normal`                 | `optimized`                       |
|:-------|:-------------------------|:--------------------------------- |
|        | `89.81 us` (✅ **1.00x**) | `133.92 us` (❌ *1.49x slower*)    |

### mul_affine_g1_bw6_761

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `1.96 us` (✅ **1.00x**) | `42.83 us` (❌ *21.89x slower*)    |

### mul_projective_g1_bw6_761

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `1.95 us` (✅ **1.00x**) | `42.96 us` (❌ *22.01x slower*)    |

### pairing_bw6_761

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `7.55 ms` (✅ **1.00x**) | `7.56 ms` (✅ **1.00x slower**)  |

### mul_affine_g2_bw6_761

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `1.95 us` (✅ **1.00x**) | `42.29 us` (❌ *21.64x slower*)    |

### mul_projective_g2_bw6_761

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `1.95 us` (✅ **1.00x**) | `42.36 us` (❌ *21.69x slower*)    |

### msm_ed_on_bls12_377

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `549.76 us` (✅ **1.00x**) | `553.32 us` (✅ **1.01x slower**)  |

### mul_affine_ed_on_bls12_377

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `549.69 us` (✅ **1.00x**) | `553.22 us` (✅ **1.01x slower**)  |

### mul_projective_ed_on_bls12_377

|        | `normal`                  | `optimized`                     |
|:-------|:--------------------------|:------------------------------- |
|        | `957.00 ns` (✅ **1.00x**) | `4.57 us` (❌ *4.77x slower*)    |

### msm_ed_on_bls12_381

|        | `normal`                 | `optimized`                      |
|:-------|:-------------------------|:-------------------------------- |
|        | `16.92 us` (✅ **1.00x**) | `21.74 us` (❌ *1.29x slower*)    |

### mul_affine_ed_on_bls12_381

|        | `normal`                  | `optimized`                     |
|:-------|:--------------------------|:------------------------------- |
|        | `414.81 ns` (✅ **1.00x**) | `4.00 us` (❌ *9.64x slower*)    |

### mul_projective_ed_on_bls12_381

|        | `normal`                  | `optimized`                     |
|:-------|:--------------------------|:------------------------------- |
|        | `413.73 ns` (✅ **1.00x**) | `4.01 us` (❌ *9.68x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

