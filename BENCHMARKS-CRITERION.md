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
|        | `6.25 ms` (✅ **1.00x**) | `7.20 ms` (❌ *1.15x slower*)    |

### pairing_bls12_381

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `2.02 ms` (✅ **1.00x**) | `2.27 ms` (❌ *1.12x slower*)    |

### msm_g1_bls12_381

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `1.34 ms` (✅ **1.00x**)         | `1.37 ms` (✅ **1.02x slower**)     | `32.80 ms` (❌ *24.43x slower*)    | `33.39 ms` (❌ *24.87x slower*)        |

### msm_g2_bls12_381

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `5.49 ms` (✅ **1.00x**)         | `5.65 ms` (✅ **1.03x slower**)     | `101.98 ms` (❌ *18.57x slower*)   | `102.83 ms` (❌ *18.73x slower*)       |

### mul_affine_g1_bls12_381

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `733.01 ns` (✅ **1.00x**) | `10.85 us` (❌ *14.80x slower*)    |

### mul_projective_g1_bls12_381

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `699.90 ns` (✅ **1.00x**) | `10.82 us` (❌ *15.47x slower*)    |

### mul_affine_g2_bls12_381

|        | `normal`                | `optimized`                      |
|:-------|:------------------------|:-------------------------------- |
|        | `1.92 us` (✅ **1.00x**) | `13.59 us` (❌ *7.09x slower*)    |

### mul_projective_g2_bls12_381

|        | `normal`                | `optimized`                      |
|:-------|:------------------------|:-------------------------------- |
|        | `1.84 us` (✅ **1.00x**) | `13.59 us` (❌ *7.38x slower*)    |

### mul_affine_g1_bls12_377

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `722.70 ns` (✅ **1.00x**) | `10.91 us` (❌ *15.10x slower*)    |

### mul_affine_g2_bls12_377

|        | `normal`                | `optimized`                      |
|:-------|:------------------------|:-------------------------------- |
|        | `2.25 us` (✅ **1.00x**) | `14.51 us` (❌ *6.46x slower*)    |

### mul_projective_g1_bls12_377

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `689.95 ns` (✅ **1.00x**) | `10.76 us` (❌ *15.60x slower*)    |

### mul_projective_g2_bls12_377

|        | `normal`                | `optimized`                      |
|:-------|:------------------------|:-------------------------------- |
|        | `2.19 us` (✅ **1.00x**) | `14.31 us` (❌ *6.55x slower*)    |

### pairing_bls12_377

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `2.41 ms` (✅ **1.00x**) | `2.43 ms` (✅ **1.00x slower**)  |

### pairing_msm_g1_bls12_377

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `1.33 ms` (✅ **1.00x**)         | `1.36 ms` (✅ **1.02x slower**)     | `33.15 ms` (❌ *24.85x slower*)    | `33.86 ms` (❌ *25.39x slower*)        |

### pairing_msm_g2_bls12_377

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `6.21 ms` (✅ **1.00x**)         | `6.23 ms` (✅ **1.00x slower**)     | `121.86 ms` (❌ *19.63x slower*)   | `123.22 ms` (❌ *19.85x slower*)       |

### msm_g1_bw6_761

|        | `normal, 10 arguments`          | `optimized,, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:------------------------------------|:----------------------------------|:------------------------------------- |
|        | `8.27 ms` (✅ **1.00x**)         | `8.28 ms` (✅ **1.00x slower**)      | `175.14 ms` (❌ *21.18x slower*)   | `175.48 ms` (❌ *21.22x slower*)       |

### msm_g2_bw6_761

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `7.97 ms` (✅ **1.00x**)         | `8.06 ms` (✅ **1.01x slower**)     | `174.54 ms` (❌ *21.91x slower*)   | `177.52 ms` (❌ *22.28x slower*)       |

### mul_affine_g1_bw6_761

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `2.32 us` (✅ **1.00x**) | `50.27 us` (❌ *21.70x slower*)    |

### mul_projective_g1_bw6_761

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `2.26 us` (✅ **1.00x**) | `49.91 us` (❌ *22.07x slower*)    |

### pairing_bw6_761

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `8.83 ms` (✅ **1.00x**) | `8.95 ms` (✅ **1.01x slower**)  |

### mul_affine_g2_bw6_761

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `2.36 us` (✅ **1.00x**) | `49.49 us` (❌ *20.99x slower*)    |

### mul_projective_g2_bw6_761

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `2.28 us` (✅ **1.00x**) | `49.37 us` (❌ *21.67x slower*)    |

### msm_10_ed_on_bls12_377

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `1.07 ms` (✅ **1.00x**)         | `1.09 ms` (✅ **1.02x slower**)     | `21.96 ms` (❌ *20.54x slower*)    | `22.41 ms` (❌ *20.96x slower*)        |

### mul_affine_ed_on_bls12_377

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `643.85 us` (✅ **1.00x**) | `643.45 us` (✅ **1.00x faster**)  |

### mul_projective_ed_on_bls12_377

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `1.12 us` (✅ **1.00x**) | `5.44 us` (❌ *4.84x slower*)    |

### msm_sw_ed_on_bls12_381

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `697.67 us` (✅ **1.00x**)       | `713.34 us` (✅ **1.02x slower**)   | `18.69 ms` (❌ *26.79x slower*)    | `19.15 ms` (❌ *27.44x slower*)        |

### mul_affine_sw_ed_on_bls12_381

|        | `normal`                  | `optimized`                     |
|:-------|:--------------------------|:------------------------------- |
|        | `524.04 ns` (✅ **1.00x**) | `4.85 us` (❌ *9.25x slower*)    |

### mul_projective_sw_ed_on_bls12_381

|        | `normal`                  | `optimized`                     |
|:-------|:--------------------------|:------------------------------- |
|        | `501.40 ns` (✅ **1.00x**) | `4.80 us` (❌ *9.58x slower*)    |

### msm_te_10_ed_on_bls12_381

|        | `normal, 10 arguments`          | `optimized, 10 arguments`          | `normal, 1000 arguments`          | `optimized, 1000 arguments`           |
|:-------|:--------------------------------|:-----------------------------------|:----------------------------------|:------------------------------------- |
|        | `690.33 us` (✅ **1.00x**)       | `701.92 us` (✅ **1.02x slower**)   | `22.54 ms` (❌ *32.65x slower*)    | `22.96 ms` (❌ *33.26x slower*)        |

### mul_affine_te_ed_on_bls12_381

|        | `normal`                  | `optimized`                     |
|:-------|:--------------------------|:------------------------------- |
|        | `529.85 ns` (✅ **1.00x**) | `4.84 us` (❌ *9.13x slower*)    |

### mul_projective_te_ed_on_bls12_381

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `1.14 us` (✅ **1.00x**) | `5.44 us` (❌ *4.77x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

