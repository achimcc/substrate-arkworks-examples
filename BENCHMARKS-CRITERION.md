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
|        | `6.11 ms` (✅ **1.00x**) | `6.86 ms` (❌ *1.12x slower*)    |

### pairing_bls12_381

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `1.94 ms` (✅ **1.00x**) | `2.10 ms` (✅ **1.09x slower**)  |

### msm_g1_bls12_381

|        | `normal`               | `optimized`                    |
|:-------|:-----------------------|:------------------------------ |
|        | `2.42 s` (✅ **1.00x**) | `7.44 s` (❌ *3.08x slower*)    |

### msm_g2_bls12_381

|        | `normal`               | `optimized`                     |
|:-------|:-----------------------|:------------------------------- |
|        | `7.37 s` (✅ **1.00x**) | `23.61 s` (❌ *3.20x slower*)    |

### mul_affine_g1_bls12_381

|        | `normal`                  | `optimized`                        |
|:-------|:--------------------------|:---------------------------------- |
|        | `658.39 ns` (✅ **1.00x**) | `86.81 us` (❌ *131.86x slower*)    |

### mul_projective_g1_bls12_381

|        | `normal`                  | `optimized`                        |
|:-------|:--------------------------|:---------------------------------- |
|        | `661.39 ns` (✅ **1.00x**) | `86.98 us` (❌ *131.52x slower*)    |

### mul_affine_g2_bls12_381

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `1.76 us` (✅ **1.00x**) | `259.46 us` (❌ *147.72x slower*)    |

### mul_projective_g2_bls12_381

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `1.76 us` (✅ **1.00x**) | `259.47 us` (❌ *147.73x slower*)    |

### mul_affine_g1_bls12_377

|        | `normal`                  | `optimized`                         |
|:-------|:--------------------------|:----------------------------------- |
|        | `639.38 ns` (✅ **1.00x**) | `141.12 us` (❌ *220.71x slower*)    |

### mul_affine_g2_bls12_377

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `2.00 us` (✅ **1.00x**) | `371.33 us` (❌ *185.64x slower*)    |

### mul_projective_g1_bls12_377

|        | `normal`                  | `optimized`                         |
|:-------|:--------------------------|:----------------------------------- |
|        | `639.16 ns` (✅ **1.00x**) | `141.11 us` (❌ *220.77x slower*)    |

### mul_projective_g2_bls12_377

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `2.00 us` (✅ **1.00x**) | `370.82 us` (❌ *185.20x slower*)    |

### pairing_bls12_377

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `2.21 ms` (✅ **1.00x**) | `2.49 ms` (❌ *1.13x slower*)    |

### pairing_msm_g1_bls12_377

|        | `normal`               | `optimized`                     |
|:-------|:-----------------------|:------------------------------- |
|        | `2.46 s` (✅ **1.00x**) | `11.40 s` (❌ *4.64x slower*)    |

### pairing_msm_g2_bls12_377

|        | `normal`               | `optimized`                     |
|:-------|:-----------------------|:------------------------------- |
|        | `8.41 s` (✅ **1.00x**) | `31.55 s` (❌ *3.75x slower*)    |

### msm_g1_bw6_761

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `105.45 us` (✅ **1.00x**) | `723.81 us` (❌ *6.86x slower*)    |

### msm_g2_bw6_761

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `105.47 us` (✅ **1.00x**) | `723.49 us` (❌ *6.86x slower*)    |

### mul_affine_g1_bw6_761

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `2.26 us` (✅ **1.00x**) | `626.92 us` (❌ *277.63x slower*)    |

### mul_projective_g1_bw6_761

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `2.24 us` (✅ **1.00x**) | `627.28 us` (❌ *280.02x slower*)    |

### pairing_bw6_761

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `8.71 ms` (✅ **1.00x**) | `9.33 ms` (✅ **1.07x slower**)  |

### mul_affine_g2_bw6_761

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `627.26 us` (✅ **1.00x**) | `626.43 us` (✅ **1.00x faster**)  |

### mul_projective_g2_bw6_761

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `2.24 us` (✅ **1.00x**) | `626.54 us` (❌ *280.01x slower*)    |

### msm_ed_on_bls12_377

|        | `normal`                  | `optimized`                      | `normal #2`                      | `optimized #2`                    |
|:-------|:--------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
|        | `663.72 us` (✅ **1.00x**) | `666.60 us` (✅ **1.00x slower**) | `663.74 us` (✅ **1.00x slower**) | `666.69 us` (✅ **1.00x slower**)  |

### mul_affine_ed_on_bls12_377

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `663.61 us` (✅ **1.00x**) | `666.59 us` (✅ **1.00x slower**)  |

### mul_projective_ed_on_bls12_377

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `1.14 us` (✅ **1.00x**) | `67.93 us` (❌ *59.60x slower*)    |

### msm_ed_on_bls12_381

|        | `normal`                 | `optimized`                      |
|:-------|:-------------------------|:-------------------------------- |
|        | `14.64 us` (✅ **1.00x**) | `62.64 us` (❌ *4.28x slower*)    |

### mul_affine_ed_on_bls12_381

|        | `normal`                  | `optimized`                        |
|:-------|:--------------------------|:---------------------------------- |
|        | `467.65 ns` (✅ **1.00x**) | `47.16 us` (❌ *100.85x slower*)    |

### mul_projective_ed_on_bls12_381

|        | `normal`                  | `optimized`                        |
|:-------|:--------------------------|:---------------------------------- |
|        | `466.08 ns` (✅ **1.00x**) | `47.24 us` (❌ *101.35x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

