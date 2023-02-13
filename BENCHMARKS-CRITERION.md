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
|        | `5.94 ms` (✅ **1.00x**) | `6.81 ms` (❌ *1.15x slower*)    |

### pairing_bls12_381

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `1.86 ms` (✅ **1.00x**) | `2.07 ms` (❌ *1.12x slower*)    |

### msm_g1_bls12_381

|        | `normal`               | `optimized`                    |
|:-------|:-----------------------|:------------------------------ |
|        | `1.21 s` (✅ **1.00x**) | `1.25 s` (✅ **1.03x slower**)  |

### msm_g2_bls12_381

|        | `normal`               | `optimized`                    |
|:-------|:-----------------------|:------------------------------ |
|        | `3.65 s` (✅ **1.00x**) | `3.64 s` (✅ **1.00x faster**)  |

### mul_affine_g1_bls12_381

|        | `normal`                  | `optimized`                      |
|:-------|:--------------------------|:-------------------------------- |
|        | `694.39 ns` (✅ **1.00x**) | `8.95 us` (❌ *12.89x slower*)    |

### mul_projective_g1_bls12_381

|        | `normal`                  | `optimized`                      |
|:-------|:--------------------------|:-------------------------------- |
|        | `672.38 ns` (✅ **1.00x**) | `8.96 us` (❌ *13.33x slower*)    |

### mul_affine_g2_bls12_381

|        | `normal`                | `optimized`                      |
|:-------|:------------------------|:-------------------------------- |
|        | `1.77 us` (✅ **1.00x**) | `11.70 us` (❌ *6.59x slower*)    |

### mul_projective_g2_bls12_381

|        | `normal`                | `optimized`                      |
|:-------|:------------------------|:-------------------------------- |
|        | `1.74 us` (✅ **1.00x**) | `11.75 us` (❌ *6.75x slower*)    |

### mul_affine_g1_bls12_377

|        | `normal`                  | `optimized`                      |
|:-------|:--------------------------|:-------------------------------- |
|        | `687.65 ns` (✅ **1.00x**) | `9.16 us` (❌ *13.33x slower*)    |

### mul_affine_g2_bls12_377

|        | `normal`                | `optimized`                      |
|:-------|:------------------------|:-------------------------------- |
|        | `2.04 us` (✅ **1.00x**) | `12.48 us` (❌ *6.12x slower*)    |

### mul_projective_g1_bls12_377

|        | `normal`                  | `optimized`                      |
|:-------|:--------------------------|:-------------------------------- |
|        | `658.91 ns` (✅ **1.00x**) | `9.17 us` (❌ *13.92x slower*)    |

### mul_projective_g2_bls12_377

|        | `normal`                | `optimized`                      |
|:-------|:------------------------|:-------------------------------- |
|        | `2.00 us` (✅ **1.00x**) | `12.63 us` (❌ *6.33x slower*)    |

### pairing_bls12_377

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `2.19 ms` (✅ **1.00x**) | `2.21 ms` (✅ **1.01x slower**)  |

### pairing_msm_g1_bls12_377

|        | `normal`               | `optimized`                    |
|:-------|:-----------------------|:------------------------------ |
|        | `1.21 s` (✅ **1.00x**) | `1.25 s` (✅ **1.03x slower**)  |

### pairing_msm_g2_bls12_377

|        | `normal`               | `optimized`                    |
|:-------|:-----------------------|:------------------------------ |
|        | `4.17 s` (✅ **1.00x**) | `4.27 s` (✅ **1.02x slower**)  |

### msm_g1_bw6_761

|        | `normal`                 | `optimized`                       |
|:-------|:-------------------------|:--------------------------------- |
|        | `94.96 us` (✅ **1.00x**) | `133.93 us` (❌ *1.41x slower*)    |

### msm_g2_bw6_761

|        | `normal`                 | `optimized`                       |
|:-------|:-------------------------|:--------------------------------- |
|        | `95.05 us` (✅ **1.00x**) | `133.44 us` (❌ *1.40x slower*)    |

### mul_affine_g1_bw6_761

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `2.27 us` (✅ **1.00x**) | `43.78 us` (❌ *19.30x slower*)    |

### mul_projective_g1_bw6_761

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `2.27 us` (✅ **1.00x**) | `44.01 us` (❌ *19.39x slower*)    |

### pairing_bw6_761

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `8.69 ms` (✅ **1.00x**) | `8.71 ms` (✅ **1.00x slower**)  |

### mul_affine_g2_bw6_761

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `2.30 us` (✅ **1.00x**) | `43.26 us` (❌ *18.83x slower*)    |

### mul_projective_g2_bw6_761

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `2.28 us` (✅ **1.00x**) | `43.43 us` (❌ *19.03x slower*)    |

### msm_ed_on_bls12_377

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `604.65 us` (✅ **1.00x**) | `607.95 us` (✅ **1.01x slower**)  |

### mul_affine_ed_on_bls12_377

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `604.67 us` (✅ **1.00x**) | `607.86 us` (✅ **1.01x slower**)  |

### mul_projective_ed_on_bls12_377

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `1.05 us` (✅ **1.00x**) | `4.67 us` (❌ *4.43x slower*)    |

### msm_ed_on_bls12_381

|        | `normal`                 | `optimized`                      |
|:-------|:-------------------------|:-------------------------------- |
|        | `14.47 us` (✅ **1.00x**) | `18.77 us` (❌ *1.30x slower*)    |

### mul_affine_ed_on_bls12_381

|        | `normal`                  | `optimized`                     |
|:-------|:--------------------------|:------------------------------- |
|        | `463.30 ns` (✅ **1.00x**) | `4.02 us` (❌ *8.67x slower*)    |

### mul_projective_ed_on_bls12_381

|        | `normal`                  | `optimized`                     |
|:-------|:--------------------------|:------------------------------- |
|        | `488.12 ns` (✅ **1.00x**) | `4.03 us` (❌ *8.26x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

