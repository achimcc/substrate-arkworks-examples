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
|        | `5.36 ms` (✅ **1.00x**) | `6.20 ms` (❌ *1.16x slower*)    |

### pairing_bls12_381

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `1.72 ms` (✅ **1.00x**) | `1.93 ms` (❌ *1.13x slower*)    |

### msm_g1_bls12_381

|        | `normal`               | `optimized`                    |
|:-------|:-----------------------|:------------------------------ |
|        | `2.22 s` (✅ **1.00x**) | `2.31 s` (✅ **1.04x slower**)  |

### msm_g2_bls12_381

|        | `normal`               | `optimized`                    |
|:-------|:-----------------------|:------------------------------ |
|        | `6.61 s` (✅ **1.00x**) | `6.74 s` (✅ **1.02x slower**)  |

### mul_affine_g1_bls12_381

|        | `normal`                  | `optimized`                      |
|:-------|:--------------------------|:-------------------------------- |
|        | `624.88 ns` (✅ **1.00x**) | `9.22 us` (❌ *14.76x slower*)    |

### mul_projective_g1_bls12_381

|        | `normal`                  | `optimized`                      |
|:-------|:--------------------------|:-------------------------------- |
|        | `586.59 ns` (✅ **1.00x**) | `9.21 us` (❌ *15.69x slower*)    |

### mul_affine_g2_bls12_381

|        | `normal`                | `optimized`                      |
|:-------|:------------------------|:-------------------------------- |
|        | `1.61 us` (✅ **1.00x**) | `11.61 us` (❌ *7.20x slower*)    |

### mul_projective_g2_bls12_381

|        | `normal`                | `optimized`                      |
|:-------|:------------------------|:-------------------------------- |
|        | `1.57 us` (✅ **1.00x**) | `11.58 us` (❌ *7.36x slower*)    |

### mul_affine_g1_bls12_377

|        | `normal`                  | `optimized`                      |
|:-------|:--------------------------|:-------------------------------- |
|        | `587.12 ns` (✅ **1.00x**) | `9.39 us` (❌ *15.99x slower*)    |

### mul_affine_g2_bls12_377

|        | `normal`                | `optimized`                      |
|:-------|:------------------------|:-------------------------------- |
|        | `1.91 us` (✅ **1.00x**) | `12.42 us` (❌ *6.52x slower*)    |

### mul_projective_g1_bls12_377

|        | `normal`                  | `optimized`                      |
|:-------|:--------------------------|:-------------------------------- |
|        | `590.91 ns` (✅ **1.00x**) | `9.39 us` (❌ *15.90x slower*)    |

### mul_projective_g2_bls12_377

|        | `normal`                | `optimized`                      |
|:-------|:------------------------|:-------------------------------- |
|        | `1.87 us` (✅ **1.00x**) | `12.55 us` (❌ *6.71x slower*)    |

### pairing_bls12_377

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `2.05 ms` (✅ **1.00x**) | `2.06 ms` (✅ **1.01x slower**)  |

### pairing_msm_g1_bls12_377

|        | `normal`               | `optimized`                    |
|:-------|:-----------------------|:------------------------------ |
|        | `2.30 s` (✅ **1.00x**) | `2.37 s` (✅ **1.03x slower**)  |

### pairing_msm_g2_bls12_377

|        | `normal`               | `optimized`                    |
|:-------|:-----------------------|:------------------------------ |
|        | `7.91 s` (✅ **1.00x**) | `8.06 s` (✅ **1.02x slower**)  |

### msm_g1_bw6_761

|        | `normal`                 | `optimized`                       |
|:-------|:-------------------------|:--------------------------------- |
|        | `98.14 us` (✅ **1.00x**) | `137.45 us` (❌ *1.40x slower*)    |

### msm_g2_bw6_761

|        | `normal`                 | `optimized`                       |
|:-------|:-------------------------|:--------------------------------- |
|        | `98.18 us` (✅ **1.00x**) | `136.41 us` (❌ *1.39x slower*)    |

### mul_affine_g1_bw6_761

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `1.95 us` (✅ **1.00x**) | `42.88 us` (❌ *21.97x slower*)    |

### mul_projective_g1_bw6_761

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `1.95 us` (✅ **1.00x**) | `43.15 us` (❌ *22.16x slower*)    |

### pairing_bw6_761

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `7.52 ms` (✅ **1.00x**) | `7.55 ms` (✅ **1.00x slower**)  |

### mul_affine_g2_bw6_761

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `1.95 us` (✅ **1.00x**) | `42.37 us` (❌ *21.75x slower*)    |

### mul_projective_g2_bw6_761

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `1.95 us` (✅ **1.00x**) | `42.62 us` (❌ *21.90x slower*)    |

### msm_ed_on_bls12_377

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `553.40 us` (✅ **1.00x**) | `555.07 us` (✅ **1.00x slower**)  |

### mul_affine_ed_on_bls12_377

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `551.91 us` (✅ **1.00x**) | `555.03 us` (✅ **1.01x slower**)  |

### mul_projective_ed_on_bls12_377

|        | `normal`                  | `optimized`                     |
|:-------|:--------------------------|:------------------------------- |
|        | `949.17 ns` (✅ **1.00x**) | `4.60 us` (❌ *4.84x slower*)    |

### msm_ed_on_bls12_381

|        | `normal`                 | `optimized`                      |
|:-------|:-------------------------|:-------------------------------- |
|        | `16.74 us` (✅ **1.00x**) | `20.88 us` (❌ *1.25x slower*)    |

### mul_affine_ed_on_bls12_381

|        | `normal`                  | `optimized`                     |
|:-------|:--------------------------|:------------------------------- |
|        | `417.04 ns` (✅ **1.00x**) | `4.00 us` (❌ *9.58x slower*)    |

### mul_projective_ed_on_bls12_381

|        | `normal`                  | `optimized`                     |
|:-------|:--------------------------|:------------------------------- |
|        | `426.55 ns` (✅ **1.00x**) | `4.00 us` (❌ *9.38x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

