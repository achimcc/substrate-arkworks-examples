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
|        | `5.39 ms` (✅ **1.00x**) | `6.33 ms` (❌ *1.17x slower*)    |

### pairing_bls12_381

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `1.75 ms` (✅ **1.00x**) | `2.04 ms` (❌ *1.16x slower*)    |

### msm_g1_bls12_381

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `71.58 us` (✅ **1.00x**)        | `85.23 us` (❌ *1.19x slower*)      | `1.84 ms` (❌ *25.75x slower*)     | `2.37 ms` (❌ *33.04x slower*)         |

### msm_g2_bls12_381

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `109.99 us` (✅ **1.00x**)       | `138.18 us` (❌ *1.26x slower*)     | `3.85 ms` (❌ *34.96x slower*)     | `4.65 ms` (❌ *42.24x slower*)         |

### mul_affine_g1_bls12_381

|        | `normal`                  | `optimized`                      |
|:-------|:--------------------------|:-------------------------------- |
|        | `660.74 ns` (✅ **1.00x**) | `9.35 us` (❌ *14.16x slower*)    |

### mul_projective_g1_bls12_381

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `610.68 ns` (✅ **1.00x**) | `10.00 us` (❌ *16.38x slower*)    |

### mul_affine_g2_bls12_381

|        | `normal`                | `optimized`                      |
|:-------|:------------------------|:-------------------------------- |
|        | `1.61 us` (✅ **1.00x**) | `11.65 us` (❌ *7.22x slower*)    |

### mul_projective_g2_bls12_381

|        | `normal`                | `optimized`                      |
|:-------|:------------------------|:-------------------------------- |
|        | `1.64 us` (✅ **1.00x**) | `12.46 us` (❌ *7.61x slower*)    |

### mul_affine_g1_bls12_377

|        | `normal`                  | `optimized`                      |
|:-------|:--------------------------|:-------------------------------- |
|        | `581.92 ns` (✅ **1.00x**) | `9.66 us` (❌ *16.59x slower*)    |

### mul_affine_g2_bls12_377

|        | `normal`                | `optimized`                      |
|:-------|:------------------------|:-------------------------------- |
|        | `2.05 us` (✅ **1.00x**) | `13.02 us` (❌ *6.36x slower*)    |

### mul_projective_g1_bls12_377

|        | `normal`                  | `optimized`                      |
|:-------|:--------------------------|:-------------------------------- |
|        | `604.28 ns` (✅ **1.00x**) | `9.95 us` (❌ *16.47x slower*)    |

### mul_projective_g2_bls12_377

|        | `normal`                | `optimized`                      |
|:-------|:------------------------|:-------------------------------- |
|        | `2.00 us` (✅ **1.00x**) | `12.86 us` (❌ *6.43x slower*)    |

### pairing_bls12_377

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `2.01 ms` (✅ **1.00x**) | `2.07 ms` (✅ **1.03x slower**)  |

### pairing_msm_g1_bls12_377

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `65.99 us` (✅ **1.00x**)        | `91.47 us` (❌ *1.39x slower*)      | `1.73 ms` (❌ *26.25x slower*)     | `2.42 ms` (❌ *36.69x slower*)         |

### pairing_msm_g2_bls12_377

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `109.96 us` (✅ **1.00x**)       | `131.15 us` (❌ *1.19x slower*)     | `4.24 ms` (❌ *38.60x slower*)     | `5.33 ms` (❌ *48.46x slower*)         |

### msm_g1_bw6_761

|        | `normal, 10 arguments`          | `optimized,, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:------------------------------------|:----------------------------------|:------------------------------------- |
|        | `138.25 us` (✅ **1.00x**)       | `188.36 us` (❌ *1.36x slower*)      | `4.64 ms` (❌ *33.54x slower*)     | `6.59 ms` (❌ *47.65x slower*)         |

### msm_g2_bw6_761

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `144.97 us` (✅ **1.00x**)       | `193.63 us` (❌ *1.34x slower*)     | `4.68 ms` (❌ *32.29x slower*)     | `6.41 ms` (❌ *44.19x slower*)         |

### mul_affine_g1_bw6_761

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `2.07 us` (✅ **1.00x**) | `46.99 us` (❌ *22.71x slower*)    |

### mul_projective_g1_bw6_761

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `1.99 us` (✅ **1.00x**) | `46.22 us` (❌ *23.20x slower*)    |

### pairing_bw6_761

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `7.80 ms` (✅ **1.00x**) | `7.50 ms` (✅ **1.04x faster**)  |

### mul_affine_g2_bw6_761

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `2.08 us` (✅ **1.00x**) | `45.98 us` (❌ *22.08x slower*)    |

### mul_projective_g2_bw6_761

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `1.99 us` (✅ **1.00x**) | `44.84 us` (❌ *22.51x slower*)    |

### msm_10_ed_on_bls12_377

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `555.47 us` (✅ **1.00x**)       | `564.19 us` (✅ **1.02x slower**)   | `6.37 ms` (❌ *11.46x slower*)     | `6.72 ms` (❌ *12.10x slower*)         |

### mul_affine_ed_on_bls12_377

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `579.20 us` (✅ **1.00x**) | `569.72 us` (✅ **1.02x faster**)  |

### mul_projective_ed_on_bls12_377

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `1.00 us` (✅ **1.00x**) | `4.92 us` (❌ *4.91x slower*)    |

### msm_sw_ed_on_bls12_381

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `29.84 us` (✅ **1.00x**)        | `36.45 us` (❌ *1.22x slower*)      | `1.11 ms` (❌ *37.27x slower*)     | `1.53 ms` (❌ *51.14x slower*)         |

### mul_affine_sw_ed_on_bls12_381

|        | `normal`                  | `optimized`                     |
|:-------|:--------------------------|:------------------------------- |
|        | `437.12 ns` (✅ **1.00x**) | `4.13 us` (❌ *9.45x slower*)    |

### mul_projective_sw_ed_on_bls12_381

|        | `normal`                  | `optimized`                     |
|:-------|:--------------------------|:------------------------------- |
|        | `435.78 ns` (✅ **1.00x**) | `4.11 us` (❌ *9.44x slower*)    |

### msm_te_10_ed_on_bls12_381

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `27.06 us` (✅ **1.00x**)        | `34.85 us` (❌ *1.29x slower*)      | `6.27 ms` (❌ *231.72x slower*)    | `6.78 ms` (❌ *250.70x slower*)        |

### mul_affine_te_ed_on_bls12_381

|        | `normal`                  | `optimized`                     |
|:-------|:--------------------------|:------------------------------- |
|        | `434.75 ns` (✅ **1.00x**) | `4.05 us` (❌ *9.32x slower*)    |

### mul_projective_te_ed_on_bls12_381

|        | `normal`                  | `optimized`                     |
|:-------|:--------------------------|:------------------------------- |
|        | `987.62 ns` (✅ **1.00x**) | `4.98 us` (❌ *5.04x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

