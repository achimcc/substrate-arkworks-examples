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
|        | `6.69 ms` (✅ **1.00x**) | `7.91 ms` (❌ *1.18x slower*)    |

### pairing_bls12_381

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `2.36 ms` (✅ **1.00x**) | `2.60 ms` (✅ **1.10x slower**)  |

### msm_g1_bls12_381

|        | `normal`                 | `optimized`                       |
|:-------|:-------------------------|:--------------------------------- |
|        | `68.90 us` (✅ **1.00x**) | `185.93 us` (❌ *2.70x slower*)    |

### msm_g2_bls12_381

|        | `normal`                 | `optimized`                       |
|:-------|:-------------------------|:--------------------------------- |
|        | `88.27 us` (✅ **1.00x**) | `422.85 us` (❌ *4.79x slower*)    |

### mul_affine_g1_bls12_381

|        | `normal`                  | `optimized`                         |
|:-------|:--------------------------|:----------------------------------- |
|        | `817.15 ns` (✅ **1.00x**) | `108.76 us` (❌ *133.10x slower*)    |

### mul_projective_g1_bls12_381

|        | `normal`                  | `optimized`                         |
|:-------|:--------------------------|:----------------------------------- |
|        | `814.06 ns` (✅ **1.00x**) | `114.79 us` (❌ *141.01x slower*)    |

### mul_affine_g2_bls12_381

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `2.14 us` (✅ **1.00x**) | `346.90 us` (❌ *161.78x slower*)    |

### mul_projective_g2_bls12_381

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `1.87 us` (✅ **1.00x**) | `348.63 us` (❌ *186.12x slower*)    |

### mul_affine_g1_bls12_377

|        | `normal`                  | `optimized`                         |
|:-------|:--------------------------|:----------------------------------- |
|        | `800.72 ns` (✅ **1.00x**) | `197.52 us` (❌ *246.67x slower*)    |

### mul_affine_g2_bls12_377

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `3.02 us` (✅ **1.00x**) | `539.52 us` (❌ *178.56x slower*)    |

### mul_projective_g1_bls12_377

|        | `normal`                  | `optimized`                         |
|:-------|:--------------------------|:----------------------------------- |
|        | `716.97 ns` (✅ **1.00x**) | `173.72 us` (❌ *242.30x slower*)    |

### mul_projective_g2_bls12_377

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `2.32 us` (✅ **1.00x**) | `488.15 us` (❌ *210.24x slower*)    |

### pairing_bls12_377

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `2.90 ms` (✅ **1.00x**) | `2.87 ms` (✅ **1.01x faster**)  |

### pairing_msm_g1_bls12_377

|        | `normal`                 | `optimized`                       |
|:-------|:-------------------------|:--------------------------------- |
|        | `79.39 us` (✅ **1.00x**) | `282.90 us` (❌ *3.56x slower*)    |

### pairing_msm_g2_bls12_377

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `100.42 us` (✅ **1.00x**) | `643.05 us` (❌ *6.40x slower*)    |

### msm_g1_bw6_761

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `136.93 us` (✅ **1.00x**) | `869.68 us` (❌ *6.35x slower*)    |

### msm_g2_bw6_761

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `138.64 us` (✅ **1.00x**) | `929.66 us` (❌ *6.71x slower*)    |

### mul_affine_g1_bw6_761

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `2.54 us` (✅ **1.00x**) | `760.99 us` (❌ *299.24x slower*)    |

### mul_projective_g1_bw6_761

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `2.61 us` (✅ **1.00x**) | `680.76 us` (❌ *260.93x slower*)    |

### pairing_bw6_761

|        | `normal`                 | `optimized`                      |
|:-------|:-------------------------|:-------------------------------- |
|        | `10.24 ms` (✅ **1.00x**) | `10.91 ms` (✅ **1.07x slower**)  |

### mul_affine_g2_bw6_761

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `755.10 us` (✅ **1.00x**) | `757.19 us` (✅ **1.00x slower**)  |

### mul_projective_g2_bw6_761

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `2.68 us` (✅ **1.00x**) | `734.53 us` (❌ *273.88x slower*)    |

### msm_ed_on_bls12_377

|        | `normal`                  | `optimized`                      | `normal #2`                      | `optimized #2`                    |
|:-------|:--------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
|        | `871.15 us` (✅ **1.00x**) | `802.84 us` (✅ **1.09x faster**) | `819.67 us` (✅ **1.06x faster**) | `824.25 us` (✅ **1.06x faster**)  |

### mul_affine_ed_on_bls12_377

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `817.53 us` (✅ **1.00x**) | `865.86 us` (✅ **1.06x slower**)  |

### mul_projective_ed_on_bls12_377

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `1.45 us` (✅ **1.00x**) | `84.69 us` (❌ *58.55x slower*)    |

### msm_ed_on_bls12_381

|        | `normal`                 | `optimized`                      |
|:-------|:-------------------------|:-------------------------------- |
|        | `25.10 us` (✅ **1.00x**) | `78.19 us` (❌ *3.11x slower*)    |

### mul_affine_ed_on_bls12_381

|        | `normal`                  | `optimized`                        |
|:-------|:--------------------------|:---------------------------------- |
|        | `522.30 ns` (✅ **1.00x**) | `54.22 us` (❌ *103.81x slower*)    |

### mul_projective_ed_on_bls12_381

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `579.66 ns` (✅ **1.00x**) | `47.09 us` (❌ *81.24x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

