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
|        | `6.06 ms` (✅ **1.00x**) | `6.88 ms` (❌ *1.14x slower*)    |

### pairing_bls12_381

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `1.91 ms` (✅ **1.00x**) | `2.12 ms` (✅ **1.11x slower**)  |

### msm_g1_bls12_381

|        | `normal`               | `optimized`                    |
|:-------|:-----------------------|:------------------------------ |
|        | `2.41 s` (✅ **1.00x**) | `7.41 s` (❌ *3.08x slower*)    |

### msm_g2_bls12_381

|        | `normal`               | `optimized`                     |
|:-------|:-----------------------|:------------------------------- |
|        | `7.30 s` (✅ **1.00x**) | `23.45 s` (❌ *3.21x slower*)    |

### mul_affine_g1_bls12_381

|        | `normal`                  | `optimized`                        |
|:-------|:--------------------------|:---------------------------------- |
|        | `656.94 ns` (✅ **1.00x**) | `86.60 us` (❌ *131.82x slower*)    |

### mul_projective_g1_bls12_381

|        | `normal`                  | `optimized`                        |
|:-------|:--------------------------|:---------------------------------- |
|        | `656.01 ns` (✅ **1.00x**) | `86.61 us` (❌ *132.03x slower*)    |

### mul_affine_g2_bls12_381

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `1.75 us` (✅ **1.00x**) | `257.03 us` (❌ *146.50x slower*)    |

### mul_projective_g2_bls12_381

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `1.75 us` (✅ **1.00x**) | `257.04 us` (❌ *146.82x slower*)    |

### mul_affine_g1_bls12_377

|        | `normal`                  | `optimized`                         |
|:-------|:--------------------------|:----------------------------------- |
|        | `646.38 ns` (✅ **1.00x**) | `141.58 us` (❌ *219.03x slower*)    |

### mul_affine_g2_bls12_377

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `2.03 us` (✅ **1.00x**) | `372.59 us` (❌ *183.86x slower*)    |

### mul_projective_g1_bls12_377

|        | `normal`                  | `optimized`                         |
|:-------|:--------------------------|:----------------------------------- |
|        | `650.01 ns` (✅ **1.00x**) | `141.52 us` (❌ *217.72x slower*)    |

### mul_projective_g2_bls12_377

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `2.01 us` (✅ **1.00x**) | `372.76 us` (❌ *185.27x slower*)    |

### pairing_bls12_377

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `2.24 ms` (✅ **1.00x**) | `2.51 ms` (❌ *1.12x slower*)    |

### pairing_msm_g1_bls12_377

|        | `normal`               | `optimized`                     |
|:-------|:-----------------------|:------------------------------- |
|        | `2.40 s` (✅ **1.00x**) | `11.39 s` (❌ *4.75x slower*)    |

### pairing_msm_g2_bls12_377

|        | `normal`               | `optimized`                     |
|:-------|:-----------------------|:------------------------------- |
|        | `8.51 s` (✅ **1.00x**) | `31.74 s` (❌ *3.73x slower*)    |

### msm_g1_bw6_761

|        | `normal`                 | `optimized`                       |
|:-------|:-------------------------|:--------------------------------- |
|        | `81.63 us` (✅ **1.00x**) | `729.12 us` (❌ *8.93x slower*)    |

### msm_g2_bw6_761

|        | `normal`                 | `optimized`                       |
|:-------|:-------------------------|:--------------------------------- |
|        | `86.63 us` (✅ **1.00x**) | `732.40 us` (❌ *8.45x slower*)    |

### mul_affine_g1_bw6_761

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `2.27 us` (✅ **1.00x**) | `627.13 us` (❌ *276.30x slower*)    |

### mul_projective_g1_bw6_761

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `2.29 us` (✅ **1.00x**) | `627.02 us` (❌ *273.85x slower*)    |

### pairing_bw6_761

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `8.71 ms` (✅ **1.00x**) | `9.33 ms` (✅ **1.07x slower**)  |

### mul_affine_g2_bw6_761

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `627.09 us` (✅ **1.00x**) | `626.45 us` (✅ **1.00x faster**)  |

### mul_projective_g2_bw6_761

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `2.26 us` (✅ **1.00x**) | `626.59 us` (❌ *277.01x slower*)    |

### msm_ed_on_bls12_377

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `662.15 us` (✅ **1.00x**) | `668.21 us` (✅ **1.01x slower**)  |

### mul_affine_ed_on_bls12_377

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `662.37 us` (✅ **1.00x**) | `668.65 us` (✅ **1.01x slower**)  |

### mul_projective_ed_on_bls12_377

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `1.13 us` (✅ **1.00x**) | `68.13 us` (❌ *60.23x slower*)    |

### msm_ed_on_bls12_381

|        | `normal`                 | `optimized`                      |
|:-------|:-------------------------|:-------------------------------- |
|        | `13.69 us` (✅ **1.00x**) | `63.47 us` (❌ *4.64x slower*)    |

### mul_affine_ed_on_bls12_381

|        | `normal`                  | `optimized`                        |
|:-------|:--------------------------|:---------------------------------- |
|        | `460.68 ns` (✅ **1.00x**) | `47.29 us` (❌ *102.64x slower*)    |

### mul_projective_ed_on_bls12_381

|        | `normal`                  | `optimized`                        |
|:-------|:--------------------------|:---------------------------------- |
|        | `459.04 ns` (✅ **1.00x**) | `47.30 us` (❌ *103.03x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

