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
|        | `5.38 ms` (✅ **1.00x**) | `6.20 ms` (❌ *1.15x slower*)    |

### pairing_bls12_381

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `1.72 ms` (✅ **1.00x**) | `1.94 ms` (❌ *1.12x slower*)    |

### msm_g1_bls12_381

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `1.15 ms` (✅ **1.00x**)         | `1.18 ms` (✅ **1.02x slower**)     | `27.93 ms` (❌ *24.24x slower*)    | `28.40 ms` (❌ *24.65x slower*)        |

### msm_g2_bls12_381

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `4.71 ms` (✅ **1.00x**)         | `4.74 ms` (✅ **1.01x slower**)     | `87.29 ms` (❌ *18.53x slower*)    | `88.11 ms` (❌ *18.71x slower*)        |

### mul_affine_g1_bls12_381

|        | `normal`                  | `optimized`                      |
|:-------|:--------------------------|:-------------------------------- |
|        | `624.25 ns` (✅ **1.00x**) | `9.20 us` (❌ *14.74x slower*)    |

### mul_projective_g1_bls12_381

|        | `normal`                  | `optimized`                      |
|:-------|:--------------------------|:-------------------------------- |
|        | `585.45 ns` (✅ **1.00x**) | `9.19 us` (❌ *15.70x slower*)    |

### mul_affine_g2_bls12_381

|        | `normal`                | `optimized`                      |
|:-------|:------------------------|:-------------------------------- |
|        | `1.62 us` (✅ **1.00x**) | `11.62 us` (❌ *7.19x slower*)    |

### mul_projective_g2_bls12_381

|        | `normal`                | `optimized`                      |
|:-------|:------------------------|:-------------------------------- |
|        | `1.58 us` (✅ **1.00x**) | `11.69 us` (❌ *7.41x slower*)    |

### mul_affine_g1_bls12_377

|        | `normal`                  | `optimized`                      |
|:-------|:--------------------------|:-------------------------------- |
|        | `618.26 ns` (✅ **1.00x**) | `9.22 us` (❌ *14.91x slower*)    |

### mul_affine_g2_bls12_377

|        | `normal`                | `optimized`                      |
|:-------|:------------------------|:-------------------------------- |
|        | `1.91 us` (✅ **1.00x**) | `12.28 us` (❌ *6.44x slower*)    |

### mul_projective_g1_bls12_377

|        | `normal`                  | `optimized`                      |
|:-------|:--------------------------|:-------------------------------- |
|        | `588.48 ns` (✅ **1.00x**) | `9.26 us` (❌ *15.73x slower*)    |

### mul_projective_g2_bls12_377

|        | `normal`                | `optimized`                      |
|:-------|:------------------------|:-------------------------------- |
|        | `1.88 us` (✅ **1.00x**) | `12.34 us` (❌ *6.58x slower*)    |

### pairing_bls12_377

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `1.80 ms` (✅ **1.00x**) | `1.82 ms` (✅ **1.01x slower**)  |

### pairing_msm_g1_bls12_377

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `1.15 ms` (✅ **1.00x**)         | `1.03 ms` (✅ **1.11x faster**)     | `28.40 ms` (❌ *24.75x slower*)    | `25.58 ms` (❌ *22.30x slower*)        |

### pairing_msm_g2_bls12_377

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `5.28 ms` (✅ **1.00x**)         | `4.81 ms` (✅ **1.10x faster**)     | `104.51 ms` (❌ *19.81x slower*)   | `105.77 ms` (❌ *20.04x slower*)       |

### msm_g1_bw6_761

|        | `normal, 10 arguments`          | `optimized,, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:------------------------------------|:----------------------------------|:------------------------------------- |
|        | `7.06 ms` (✅ **1.00x**)         | `7.13 ms` (✅ **1.01x slower**)      | `149.20 ms` (❌ *21.12x slower*)   | `150.65 ms` (❌ *21.33x slower*)       |

### msm_g2_bw6_761

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `6.79 ms` (✅ **1.00x**)         | `6.84 ms` (✅ **1.01x slower**)     | `149.50 ms` (❌ *22.03x slower*)   | `150.99 ms` (❌ *22.25x slower*)       |

### mul_affine_g1_bw6_761

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `1.98 us` (✅ **1.00x**) | `42.85 us` (❌ *21.65x slower*)    |

### mul_projective_g1_bw6_761

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `1.95 us` (✅ **1.00x**) | `42.84 us` (❌ *21.98x slower*)    |

### pairing_bw6_761

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `7.56 ms` (✅ **1.00x**) | `7.58 ms` (✅ **1.00x slower**)  |

### mul_affine_g2_bw6_761

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `1.98 us` (✅ **1.00x**) | `42.05 us` (❌ *21.25x slower*)    |

### mul_projective_g2_bw6_761

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `1.95 us` (✅ **1.00x**) | `42.32 us` (❌ *21.70x slower*)    |

### msm_10_ed_on_bls12_377

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `916.74 us` (✅ **1.00x**)       | `929.30 us` (✅ **1.01x slower**)   | `18.75 ms` (❌ *20.45x slower*)    | `19.05 ms` (❌ *20.78x slower*)        |

### mul_affine_ed_on_bls12_377

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `550.02 us` (✅ **1.00x**) | `554.46 us` (✅ **1.01x slower**)  |

### mul_projective_ed_on_bls12_377

|        | `normal`                  | `optimized`                     |
|:-------|:--------------------------|:------------------------------- |
|        | `957.03 ns` (✅ **1.00x**) | `4.58 us` (❌ *4.78x slower*)    |

### msm_sw_ed_on_bls12_381

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `591.07 us` (✅ **1.00x**)       | `604.29 us` (✅ **1.02x slower**)   | `16.00 ms` (❌ *27.07x slower*)    | `16.35 ms` (❌ *27.67x slower*)        |

### mul_affine_sw_ed_on_bls12_381

|        | `normal`                  | `optimized`                     |
|:-------|:--------------------------|:------------------------------- |
|        | `451.88 ns` (✅ **1.00x**) | `4.18 us` (❌ *9.26x slower*)    |

### mul_projective_sw_ed_on_bls12_381

|        | `normal`                  | `optimized`                     |
|:-------|:--------------------------|:------------------------------- |
|        | `423.56 ns` (✅ **1.00x**) | `4.13 us` (❌ *9.76x slower*)    |

### msm_te_10_ed_on_bls12_381

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `590.89 us` (✅ **1.00x**)       | `603.65 us` (✅ **1.02x slower**)   | `19.07 ms` (❌ *32.27x slower*)    | `19.55 ms` (❌ *33.09x slower*)        |

### mul_affine_te_ed_on_bls12_381

|        | `normal`                  | `optimized`                     |
|:-------|:--------------------------|:------------------------------- |
|        | `451.86 ns` (✅ **1.00x**) | `4.18 us` (❌ *9.26x slower*)    |

### mul_projective_te_ed_on_bls12_381

|        | `normal`                  | `optimized`                     |
|:-------|:--------------------------|:------------------------------- |
|        | `976.79 ns` (✅ **1.00x**) | `4.65 us` (❌ *4.77x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

