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
|        | `6.07 ms` (✅ **1.00x**) | `6.87 ms` (❌ *1.13x slower*)    |

### pairing_bls12_381

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `1.91 ms` (✅ **1.00x**) | `2.11 ms` (✅ **1.11x slower**)  |

### msm_g1_bls12_381

|        | `normal`               | `optimized`                    |
|:-------|:-----------------------|:------------------------------ |
|        | `2.40 s` (✅ **1.00x**) | `7.41 s` (❌ *3.08x slower*)    |

### msm_g2_bls12_381

|        | `normal`               | `optimized`                     |
|:-------|:-----------------------|:------------------------------- |
|        | `7.29 s` (✅ **1.00x**) | `23.46 s` (❌ *3.22x slower*)    |

### mul_affine_g1_bls12_381

|        | `normal`                  | `optimized`                        |
|:-------|:--------------------------|:---------------------------------- |
|        | `655.74 ns` (✅ **1.00x**) | `86.65 us` (❌ *132.14x slower*)    |

### mul_projective_g1_bls12_381

|        | `normal`                  | `optimized`                        |
|:-------|:--------------------------|:---------------------------------- |
|        | `656.00 ns` (✅ **1.00x**) | `86.55 us` (❌ *131.93x slower*)    |

### mul_affine_g2_bls12_381

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `1.76 us` (✅ **1.00x**) | `256.98 us` (❌ *146.36x slower*)    |

### mul_projective_g2_bls12_381

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `1.75 us` (✅ **1.00x**) | `256.89 us` (❌ *147.00x slower*)    |

### mul_affine_g1_bls12_377

|        | `normal`                  | `optimized`                         |
|:-------|:--------------------------|:----------------------------------- |
|        | `644.65 ns` (✅ **1.00x**) | `141.49 us` (❌ *219.48x slower*)    |

### mul_affine_g2_bls12_377

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `2.03 us` (✅ **1.00x**) | `372.35 us` (❌ *183.79x slower*)    |

### mul_projective_g1_bls12_377

|        | `normal`                  | `optimized`                         |
|:-------|:--------------------------|:----------------------------------- |
|        | `649.71 ns` (✅ **1.00x**) | `141.52 us` (❌ *217.82x slower*)    |

### mul_projective_g2_bls12_377

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `2.01 us` (✅ **1.00x**) | `372.44 us` (❌ *185.70x slower*)    |

### pairing_bls12_377

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `2.23 ms` (✅ **1.00x**) | `2.51 ms` (❌ *1.13x slower*)    |

### pairing_msm_g1_bls12_377

|        | `normal`               | `optimized`                     |
|:-------|:-----------------------|:------------------------------- |
|        | `2.40 s` (✅ **1.00x**) | `11.40 s` (❌ *4.76x slower*)    |

### pairing_msm_g2_bls12_377

|        | `normal`               | `optimized`                     |
|:-------|:-----------------------|:------------------------------- |
|        | `8.40 s` (✅ **1.00x**) | `31.69 s` (❌ *3.77x slower*)    |

### msm_g1_bw6_761

|        | `normal`                 | `optimized`                       |
|:-------|:-------------------------|:--------------------------------- |
|        | `88.20 us` (✅ **1.00x**) | `723.46 us` (❌ *8.20x slower*)    |

### msm_g2_bw6_761

|        | `normal`                 | `optimized`                       |
|:-------|:-------------------------|:--------------------------------- |
|        | `83.08 us` (✅ **1.00x**) | `729.97 us` (❌ *8.79x slower*)    |

### mul_affine_g1_bw6_761

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `2.27 us` (✅ **1.00x**) | `627.13 us` (❌ *275.68x slower*)    |

### mul_projective_g1_bw6_761

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `2.27 us` (✅ **1.00x**) | `626.95 us` (❌ *276.16x slower*)    |

### pairing_bw6_761

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `8.70 ms` (✅ **1.00x**) | `9.32 ms` (✅ **1.07x slower**)  |

### mul_affine_g2_bw6_761

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `626.91 us` (✅ **1.00x**) | `626.55 us` (✅ **1.00x faster**)  |

### mul_projective_g2_bw6_761

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `2.26 us` (✅ **1.00x**) | `626.76 us` (❌ *277.80x slower*)    |

### msm_ed_on_bls12_377

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `662.16 us` (✅ **1.00x**) | `669.89 us` (✅ **1.01x slower**)  |

### mul_affine_ed_on_bls12_377

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `662.96 us` (✅ **1.00x**) | `670.02 us` (✅ **1.01x slower**)  |

### mul_projective_ed_on_bls12_377

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `1.13 us` (✅ **1.00x**) | `68.10 us` (❌ *60.22x slower*)    |

### msm_ed_on_bls12_381

|        | `normal`                 | `optimized`                      |
|:-------|:-------------------------|:-------------------------------- |
|        | `13.68 us` (✅ **1.00x**) | `63.03 us` (❌ *4.61x slower*)    |

### mul_affine_ed_on_bls12_381

|        | `normal`                  | `optimized`                        |
|:-------|:--------------------------|:---------------------------------- |
|        | `460.45 ns` (✅ **1.00x**) | `47.27 us` (❌ *102.66x slower*)    |

### mul_projective_ed_on_bls12_381

|        | `normal`                  | `optimized`                        |
|:-------|:--------------------------|:---------------------------------- |
|        | `458.72 ns` (✅ **1.00x**) | `47.28 us` (❌ *103.06x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

