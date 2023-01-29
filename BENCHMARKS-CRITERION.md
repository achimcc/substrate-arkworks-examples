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
|        | `6.04 ms` (✅ **1.00x**) | `6.86 ms` (❌ *1.14x slower*)    |

### pairing_bls12_381

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `1.91 ms` (✅ **1.00x**) | `2.09 ms` (✅ **1.10x slower**)  |

### msm_g1_bls12_381

|        | `normal`               | `optimized`                    |
|:-------|:-----------------------|:------------------------------ |
|        | `2.39 s` (✅ **1.00x**) | `7.42 s` (❌ *3.11x slower*)    |

### msm_g2_bls12_381

|        | `normal`               | `optimized`                     |
|:-------|:-----------------------|:------------------------------- |
|        | `7.29 s` (✅ **1.00x**) | `23.41 s` (❌ *3.21x slower*)    |

### mul_affine_g1_bls12_381

|        | `normal`                  | `optimized`                        |
|:-------|:--------------------------|:---------------------------------- |
|        | `661.85 ns` (✅ **1.00x**) | `86.30 us` (❌ *130.39x slower*)    |

### mul_projective_g1_bls12_381

|        | `normal`                  | `optimized`                        |
|:-------|:--------------------------|:---------------------------------- |
|        | `654.69 ns` (✅ **1.00x**) | `86.34 us` (❌ *131.87x slower*)    |

### mul_affine_g2_bls12_381

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `1.75 us` (✅ **1.00x**) | `256.13 us` (❌ *146.51x slower*)    |

### mul_projective_g2_bls12_381

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `1.75 us` (✅ **1.00x**) | `256.25 us` (❌ *146.22x slower*)    |

### mul_affine_g1_bls12_377

|        | `normal`                  | `optimized`                         |
|:-------|:--------------------------|:----------------------------------- |
|        | `646.98 ns` (✅ **1.00x**) | `141.26 us` (❌ *218.33x slower*)    |

### mul_affine_g2_bls12_377

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `2.02 us` (✅ **1.00x**) | `372.26 us` (❌ *183.85x slower*)    |

### mul_projective_g1_bls12_377

|        | `normal`                  | `optimized`                         |
|:-------|:--------------------------|:----------------------------------- |
|        | `650.42 ns` (✅ **1.00x**) | `141.30 us` (❌ *217.24x slower*)    |

### mul_projective_g2_bls12_377

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `2.01 us` (✅ **1.00x**) | `372.59 us` (❌ *185.34x slower*)    |

### pairing_bls12_377

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `2.23 ms` (✅ **1.00x**) | `2.50 ms` (❌ *1.12x slower*)    |

### pairing_msm_g1_bls12_377

|        | `normal`               | `optimized`                     |
|:-------|:-----------------------|:------------------------------- |
|        | `2.39 s` (✅ **1.00x**) | `11.39 s` (❌ *4.75x slower*)    |

### pairing_msm_g2_bls12_377

|        | `normal`               | `optimized`                     |
|:-------|:-----------------------|:------------------------------- |
|        | `8.43 s` (✅ **1.00x**) | `31.66 s` (❌ *3.76x slower*)    |

### msm_g1_bw6_761

|        | `normal`                 | `optimized`                       |
|:-------|:-------------------------|:--------------------------------- |
|        | `86.70 us` (✅ **1.00x**) | `719.24 us` (❌ *8.30x slower*)    |

### msm_g2_bw6_761

|        | `normal`                 | `optimized`                       |
|:-------|:-------------------------|:--------------------------------- |
|        | `81.39 us` (✅ **1.00x**) | `727.16 us` (❌ *8.93x slower*)    |

### mul_affine_g1_bw6_761

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `2.30 us` (✅ **1.00x**) | `627.08 us` (❌ *273.20x slower*)    |

### mul_projective_g1_bw6_761

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `2.27 us` (✅ **1.00x**) | `627.21 us` (❌ *276.14x slower*)    |

### pairing_bw6_761

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `8.73 ms` (✅ **1.00x**) | `9.32 ms` (✅ **1.07x slower**)  |

### mul_affine_g2_bw6_761

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `627.20 us` (✅ **1.00x**) | `626.44 us` (✅ **1.00x faster**)  |

### mul_projective_g2_bw6_761

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `2.26 us` (✅ **1.00x**) | `626.50 us` (❌ *277.36x slower*)    |

### msm_ed_on_bls12_377

|        | `normal`                  | `optimized`                      | `normal #2`                      | `optimized #2`                    |
|:-------|:--------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
|        | `662.74 us` (✅ **1.00x**) | `667.22 us` (✅ **1.01x slower**) | `662.76 us` (✅ **1.00x slower**) | `666.72 us` (✅ **1.01x slower**)  |

### mul_affine_ed_on_bls12_377

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `662.81 us` (✅ **1.00x**) | `666.80 us` (✅ **1.01x slower**)  |

### mul_projective_ed_on_bls12_377

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `1.13 us` (✅ **1.00x**) | `67.85 us` (❌ *60.13x slower*)    |

### msm_ed_on_bls12_381

|        | `normal`                 | `optimized`                      |
|:-------|:-------------------------|:-------------------------------- |
|        | `14.30 us` (✅ **1.00x**) | `62.58 us` (❌ *4.38x slower*)    |

### mul_affine_ed_on_bls12_381

|        | `normal`                  | `optimized`                        |
|:-------|:--------------------------|:---------------------------------- |
|        | `459.00 ns` (✅ **1.00x**) | `46.70 us` (❌ *101.73x slower*)    |

### mul_projective_ed_on_bls12_381

|        | `normal`                  | `optimized`                        |
|:-------|:--------------------------|:---------------------------------- |
|        | `460.17 ns` (✅ **1.00x**) | `46.69 us` (❌ *101.47x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

