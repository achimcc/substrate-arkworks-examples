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
|        | `6.85 ms` (✅ **1.00x**) | `7.85 ms` (❌ *1.14x slower*)    |

### pairing_bls12_381

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `2.22 ms` (✅ **1.00x**) | `2.52 ms` (❌ *1.13x slower*)    |

### msm_g1_bls12_381

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `86.88 us` (✅ **1.00x**)        | `105.00 us` (❌ *1.21x slower*)     | `2.26 ms` (❌ *26.01x slower*)     | `2.85 ms` (❌ *32.86x slower*)         |

### msm_g2_bls12_381

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `137.07 us` (✅ **1.00x**)       | `158.42 us` (❌ *1.16x slower*)     | `4.82 ms` (❌ *35.14x slower*)     | `5.87 ms` (❌ *42.81x slower*)         |

### mul_affine_g1_bls12_381

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `817.55 ns` (✅ **1.00x**) | `12.14 us` (❌ *14.84x slower*)    |

### mul_projective_g1_bls12_381

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `784.09 ns` (✅ **1.00x**) | `12.42 us` (❌ *15.84x slower*)    |

### mul_affine_g2_bls12_381

|        | `normal`                | `optimized`                      |
|:-------|:------------------------|:-------------------------------- |
|        | `2.09 us` (✅ **1.00x**) | `15.28 us` (❌ *7.31x slower*)    |

### mul_projective_g2_bls12_381

|        | `normal`                | `optimized`                      |
|:-------|:------------------------|:-------------------------------- |
|        | `2.07 us` (✅ **1.00x**) | `15.25 us` (❌ *7.36x slower*)    |

### mul_affine_g1_bls12_377

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `756.67 ns` (✅ **1.00x**) | `12.14 us` (❌ *16.05x slower*)    |

### mul_affine_g2_bls12_377

|        | `normal`                | `optimized`                      |
|:-------|:------------------------|:-------------------------------- |
|        | `2.40 us` (✅ **1.00x**) | `16.06 us` (❌ *6.69x slower*)    |

### mul_projective_g1_bls12_377

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `763.33 ns` (✅ **1.00x**) | `12.38 us` (❌ *16.22x slower*)    |

### mul_projective_g2_bls12_377

|        | `normal`                | `optimized`                      |
|:-------|:------------------------|:-------------------------------- |
|        | `2.45 us` (✅ **1.00x**) | `16.06 us` (❌ *6.56x slower*)    |

### pairing_bls12_377

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `2.54 ms` (✅ **1.00x**) | `2.55 ms` (✅ **1.00x slower**)  |

### pairing_msm_g1_bls12_377

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `84.87 us` (✅ **1.00x**)        | `104.35 us` (❌ *1.23x slower*)     | `2.22 ms` (❌ *26.18x slower*)     | `3.01 ms` (❌ *35.49x slower*)         |

### pairing_msm_g2_bls12_377

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `138.68 us` (✅ **1.00x**)       | `169.74 us` (❌ *1.22x slower*)     | `5.41 ms` (❌ *39.01x slower*)     | `6.81 ms` (❌ *49.12x slower*)         |

### msm_g1_bw6_761

|        | `normal, 10 arguments`          | `optimized,, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:------------------------------------|:----------------------------------|:------------------------------------- |
|        | `196.51 us` (✅ **1.00x**)       | `247.53 us` (❌ *1.26x slower*)      | `5.97 ms` (❌ *30.40x slower*)     | `8.00 ms` (❌ *40.70x slower*)         |

### msm_g2_bw6_761

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `200.71 us` (✅ **1.00x**)       | `243.81 us` (❌ *1.21x slower*)     | `6.06 ms` (❌ *30.21x slower*)     | `7.99 ms` (❌ *39.81x slower*)         |

### mul_affine_g1_bw6_761

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `2.52 us` (✅ **1.00x**) | `57.72 us` (❌ *22.92x slower*)    |

### mul_projective_g1_bw6_761

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `2.58 us` (✅ **1.00x**) | `58.15 us` (❌ *22.58x slower*)    |

### pairing_bw6_761

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `9.90 ms` (✅ **1.00x**) | `9.90 ms` (✅ **1.00x faster**)  |

### mul_affine_g2_bw6_761

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `2.58 us` (✅ **1.00x**) | `57.08 us` (❌ *22.16x slower*)    |

### mul_projective_g2_bw6_761

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `2.63 us` (✅ **1.00x**) | `58.39 us` (❌ *22.16x slower*)    |

### msm_10_ed_on_bls12_377

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `723.31 us` (✅ **1.00x**)       | `725.98 us` (✅ **1.00x slower**)   | `8.29 ms` (❌ *11.46x slower*)     | `8.74 ms` (❌ *12.09x slower*)         |

### mul_affine_ed_on_bls12_377

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `728.00 us` (✅ **1.00x**) | `717.17 us` (✅ **1.02x faster**)  |

### mul_projective_ed_on_bls12_377

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `1.27 us` (✅ **1.00x**) | `5.80 us` (❌ *4.56x slower*)    |

### msm_sw_ed_on_bls12_381

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `35.80 us` (✅ **1.00x**)        | `45.69 us` (❌ *1.28x slower*)      | `1.35 ms` (❌ *37.84x slower*)     | `1.84 ms` (❌ *51.46x slower*)         |

### mul_affine_sw_ed_on_bls12_381

|        | `normal`                  | `optimized`                     |
|:-------|:--------------------------|:------------------------------- |
|        | `577.20 ns` (✅ **1.00x**) | `5.34 us` (❌ *9.25x slower*)    |

### mul_projective_sw_ed_on_bls12_381

|        | `normal`                  | `optimized`                     |
|:-------|:--------------------------|:------------------------------- |
|        | `566.71 ns` (✅ **1.00x**) | `5.38 us` (❌ *9.49x slower*)    |

### msm_te_10_ed_on_bls12_381

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `35.90 us` (✅ **1.00x**)        | `45.56 us` (❌ *1.27x slower*)      | `8.22 ms` (❌ *228.91x slower*)    | `8.82 ms` (❌ *245.65x slower*)        |

### mul_affine_te_ed_on_bls12_381

|        | `normal`                  | `optimized`                     |
|:-------|:--------------------------|:------------------------------- |
|        | `570.16 ns` (✅ **1.00x**) | `5.36 us` (❌ *9.40x slower*)    |

### mul_projective_te_ed_on_bls12_381

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `1.27 us` (✅ **1.00x**) | `5.97 us` (❌ *4.72x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

