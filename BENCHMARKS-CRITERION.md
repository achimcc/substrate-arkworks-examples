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
|        | `6.98 ms` (✅ **1.00x**) | `8.20 ms` (❌ *1.18x slower*)    |

### pairing_bls12_381

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `2.23 ms` (✅ **1.00x**) | `2.53 ms` (❌ *1.13x slower*)    |

### msm_g1_bls12_381

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `1.53 ms` (✅ **1.00x**)         | `1.62 ms` (✅ **1.06x slower**)     | `38.19 ms` (❌ *25.01x slower*)    | `38.81 ms` (❌ *25.42x slower*)        |

### msm_g2_bls12_381

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `6.19 ms` (✅ **1.00x**)         | `6.49 ms` (✅ **1.05x slower**)     | `117.81 ms` (❌ *19.03x slower*)   | `119.00 ms` (❌ *19.22x slower*)       |

### mul_affine_g1_bls12_381

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `849.97 ns` (✅ **1.00x**) | `12.51 us` (❌ *14.71x slower*)    |

### mul_projective_g1_bls12_381

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `806.08 ns` (✅ **1.00x**) | `12.50 us` (❌ *15.51x slower*)    |

### mul_affine_g2_bls12_381

|        | `normal`                | `optimized`                      |
|:-------|:------------------------|:-------------------------------- |
|        | `2.25 us` (✅ **1.00x**) | `15.74 us` (❌ *7.00x slower*)    |

### mul_projective_g2_bls12_381

|        | `normal`                | `optimized`                      |
|:-------|:------------------------|:-------------------------------- |
|        | `2.18 us` (✅ **1.00x**) | `15.36 us` (❌ *7.06x slower*)    |

### mul_affine_g1_bls12_377

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `840.33 ns` (✅ **1.00x**) | `12.65 us` (❌ *15.05x slower*)    |

### mul_affine_g2_bls12_377

|        | `normal`                | `optimized`                      |
|:-------|:------------------------|:-------------------------------- |
|        | `2.63 us` (✅ **1.00x**) | `16.69 us` (❌ *6.35x slower*)    |

### mul_projective_g1_bls12_377

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `789.66 ns` (✅ **1.00x**) | `12.44 us` (❌ *15.76x slower*)    |

### mul_projective_g2_bls12_377

|        | `normal`                | `optimized`                      |
|:-------|:------------------------|:-------------------------------- |
|        | `2.46 us` (✅ **1.00x**) | `16.42 us` (❌ *6.68x slower*)    |

### pairing_bls12_377

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `2.66 ms` (✅ **1.00x**) | `2.70 ms` (✅ **1.02x slower**)  |

### pairing_msm_g1_bls12_377

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `1.53 ms` (✅ **1.00x**)         | `1.59 ms` (✅ **1.04x slower**)     | `37.80 ms` (❌ *24.73x slower*)    | `39.72 ms` (❌ *25.99x slower*)        |

### pairing_msm_g2_bls12_377

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `6.86 ms` (✅ **1.00x**)         | `6.95 ms` (✅ **1.01x slower**)     | `136.71 ms` (❌ *19.91x slower*)   | `140.72 ms` (❌ *20.50x slower*)       |

### msm_g1_bw6_761

|        | `normal, 10 arguments`          | `optimized,, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:------------------------------------|:----------------------------------|:------------------------------------- |
|        | `9.49 ms` (✅ **1.00x**)         | `9.53 ms` (✅ **1.00x slower**)      | `198.67 ms` (❌ *20.94x slower*)   | `198.60 ms` (❌ *20.93x slower*)       |

### msm_g2_bw6_761

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `9.25 ms` (✅ **1.00x**)         | `9.14 ms` (✅ **1.01x faster**)     | `201.03 ms` (❌ *21.73x slower*)   | `203.73 ms` (❌ *22.03x slower*)       |

### mul_affine_g1_bw6_761

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `2.72 us` (✅ **1.00x**) | `59.96 us` (❌ *22.04x slower*)    |

### mul_projective_g1_bw6_761

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `2.56 us` (✅ **1.00x**) | `59.61 us` (❌ *23.28x slower*)    |

### pairing_bw6_761

|        | `normal`                 | `optimized`                      |
|:-------|:-------------------------|:-------------------------------- |
|        | `10.11 ms` (✅ **1.00x**) | `10.02 ms` (✅ **1.01x faster**)  |

### mul_affine_g2_bw6_761

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `2.73 us` (✅ **1.00x**) | `58.12 us` (❌ *21.32x slower*)    |

### mul_projective_g2_bw6_761

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `2.66 us` (✅ **1.00x**) | `58.15 us` (❌ *21.85x slower*)    |

### msm_10_ed_on_bls12_377

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `1.17 ms` (✅ **1.00x**)         | `1.19 ms` (✅ **1.02x slower**)     | `24.15 ms` (❌ *20.73x slower*)    | `25.03 ms` (❌ *21.48x slower*)        |

### mul_affine_ed_on_bls12_377

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `721.36 us` (✅ **1.00x**) | `722.71 us` (✅ **1.00x slower**)  |

### mul_projective_ed_on_bls12_377

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `1.26 us` (✅ **1.00x**) | `5.96 us` (❌ *4.71x slower*)    |

### msm_sw_ed_on_bls12_381

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `781.68 us` (✅ **1.00x**)       | `799.11 us` (✅ **1.02x slower**)   | `21.18 ms` (❌ *27.09x slower*)    | `22.06 ms` (❌ *28.22x slower*)        |

### mul_affine_sw_ed_on_bls12_381

|        | `normal`                  | `optimized`                     |
|:-------|:--------------------------|:------------------------------- |
|        | `606.20 ns` (✅ **1.00x**) | `5.34 us` (❌ *8.81x slower*)    |

### mul_projective_sw_ed_on_bls12_381

|        | `normal`                  | `optimized`                     |
|:-------|:--------------------------|:------------------------------- |
|        | `562.87 ns` (✅ **1.00x**) | `5.39 us` (❌ *9.58x slower*)    |

### msm_te_10_ed_on_bls12_381

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `770.55 us` (✅ **1.00x**)       | `788.76 us` (✅ **1.02x slower**)   | `25.24 ms` (❌ *32.75x slower*)    | `25.39 ms` (❌ *32.95x slower*)        |

### mul_affine_te_ed_on_bls12_381

|        | `normal`                  | `optimized`                     |
|:-------|:--------------------------|:------------------------------- |
|        | `599.88 ns` (✅ **1.00x**) | `5.39 us` (❌ *8.99x slower*)    |

### mul_projective_te_ed_on_bls12_381

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `1.29 us` (✅ **1.00x**) | `6.01 us` (❌ *4.65x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

