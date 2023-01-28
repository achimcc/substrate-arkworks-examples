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
|        | `6.86 ms` (✅ **1.00x**) | `8.20 ms` (❌ *1.19x slower*)    |

### pairing_bls12_381

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `2.19 ms` (✅ **1.00x**) | `2.46 ms` (❌ *1.12x slower*)    |

### msm_g1_bls12_381

|        | `normal`               | `optimized`                     |
|:-------|:-----------------------|:------------------------------- |
|        | `3.48 s` (✅ **1.00x**) | `11.13 s` (❌ *3.20x slower*)    |

### msm_g2_bls12_381

|        | `normal`               | `optimized`                     |
|:-------|:-----------------------|:------------------------------- |
|        | `9.18 s` (✅ **1.00x**) | `33.29 s` (❌ *3.63x slower*)    |

### mul_affine_g1_bls12_381

|        | `normal`                  | `optimized`                         |
|:-------|:--------------------------|:----------------------------------- |
|        | `907.15 ns` (✅ **1.00x**) | `124.59 us` (❌ *137.35x slower*)    |

### mul_projective_g1_bls12_381

|        | `normal`                  | `optimized`                         |
|:-------|:--------------------------|:----------------------------------- |
|        | `907.51 ns` (✅ **1.00x**) | `130.26 us` (❌ *143.54x slower*)    |

### mul_affine_g2_bls12_381

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `2.36 us` (✅ **1.00x**) | `392.86 us` (❌ *166.76x slower*)    |

### mul_projective_g2_bls12_381

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `2.34 us` (✅ **1.00x**) | `384.95 us` (❌ *164.55x slower*)    |

### mul_affine_g1_bls12_377

|        | `normal`                  | `optimized`                         |
|:-------|:--------------------------|:----------------------------------- |
|        | `887.78 ns` (✅ **1.00x**) | `200.46 us` (❌ *225.79x slower*)    |

### mul_affine_g2_bls12_377

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `2.77 us` (✅ **1.00x**) | `510.99 us` (❌ *184.80x slower*)    |

### mul_projective_g1_bls12_377

|        | `normal`                  | `optimized`                         |
|:-------|:--------------------------|:----------------------------------- |
|        | `869.67 ns` (✅ **1.00x**) | `191.85 us` (❌ *220.60x slower*)    |

### mul_projective_g2_bls12_377

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `2.67 us` (✅ **1.00x**) | `517.66 us` (❌ *193.53x slower*)    |

### pairing_bls12_377

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `2.79 ms` (✅ **1.00x**) | `3.06 ms` (✅ **1.09x slower**)  |

### pairing_msm_g1_bls12_377

|        | `normal`               | `optimized`                     |
|:-------|:-----------------------|:------------------------------- |
|        | `3.61 s` (✅ **1.00x**) | `15.93 s` (❌ *4.41x slower*)    |

### pairing_msm_g2_bls12_377

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `11.22 s` (✅ **1.00x**) | `42.89 s` (❌ *3.82x slower*)    |

### msm_g1_bw6_761

|        | `normal`                  | `optimized`                     |
|:-------|:--------------------------|:------------------------------- |
|        | `125.54 us` (✅ **1.00x**) | `1.02 ms` (❌ *8.12x slower*)    |

### msm_g2_bw6_761

|        | `normal`                  | `optimized`                     |
|:-------|:--------------------------|:------------------------------- |
|        | `122.49 us` (✅ **1.00x**) | `1.01 ms` (❌ *8.27x slower*)    |

### mul_affine_g1_bw6_761

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `3.20 us` (✅ **1.00x**) | `898.96 us` (❌ *280.70x slower*)    |

### mul_projective_g1_bw6_761

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `3.12 us` (✅ **1.00x**) | `891.37 us` (❌ *285.96x slower*)    |

### pairing_bw6_761

|        | `normal`                 | `optimized`                      |
|:-------|:-------------------------|:-------------------------------- |
|        | `12.36 ms` (✅ **1.00x**) | `12.66 ms` (✅ **1.02x slower**)  |

### mul_affine_g2_bw6_761

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `897.15 us` (✅ **1.00x**) | `869.20 us` (✅ **1.03x faster**)  |

### mul_projective_g2_bw6_761

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `3.07 us` (✅ **1.00x**) | `861.96 us` (❌ *280.61x slower*)    |

### msm_ed_on_bls12_377

|        | `normal`                  | `optimized`                      | `normal #2`                      | `optimized #2`                    |
|:-------|:--------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
|        | `718.50 us` (✅ **1.00x**) | `768.18 us` (✅ **1.07x slower**) | `740.10 us` (✅ **1.03x slower**) | `796.42 us` (✅ **1.11x slower**)  |

### mul_affine_ed_on_bls12_377

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `754.01 us` (✅ **1.00x**) | `781.77 us` (✅ **1.04x slower**)  |

### mul_projective_ed_on_bls12_377

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `1.35 us` (✅ **1.00x**) | `79.94 us` (❌ *59.00x slower*)    |

### msm_ed_on_bls12_381

|        | `normal`                 | `optimized`                      |
|:-------|:-------------------------|:-------------------------------- |
|        | `22.00 us` (✅ **1.00x**) | `78.47 us` (❌ *3.57x slower*)    |

### mul_affine_ed_on_bls12_381

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `594.21 ns` (✅ **1.00x**) | `54.61 us` (❌ *91.90x slower*)    |

### mul_projective_ed_on_bls12_381

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `580.74 ns` (✅ **1.00x**) | `52.19 us` (❌ *89.87x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

