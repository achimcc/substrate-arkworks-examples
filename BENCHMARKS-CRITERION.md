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
|        | `6.07 ms` (✅ **1.00x**) | `6.91 ms` (❌ *1.14x slower*)    |

### pairing_bls12_381

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `1.91 ms` (✅ **1.00x**) | `2.12 ms` (✅ **1.11x slower**)  |

### msm_g1_bls12_381

|        | `normal`               | `optimized`                    |
|:-------|:-----------------------|:------------------------------ |
|        | `2.39 s` (✅ **1.00x**) | `7.45 s` (❌ *3.12x slower*)    |

### msm_g2_bls12_381

|        | `normal`               | `optimized`                     |
|:-------|:-----------------------|:------------------------------- |
|        | `7.35 s` (✅ **1.00x**) | `23.49 s` (❌ *3.19x slower*)    |

### mul_affine_g1_bls12_381

|        | `normal`                  | `optimized`                        |
|:-------|:--------------------------|:---------------------------------- |
|        | `653.10 ns` (✅ **1.00x**) | `87.27 us` (❌ *133.63x slower*)    |

### mul_projective_g1_bls12_381

|        | `normal`                  | `optimized`                        |
|:-------|:--------------------------|:---------------------------------- |
|        | `649.31 ns` (✅ **1.00x**) | `87.35 us` (❌ *134.53x slower*)    |

### mul_affine_g2_bls12_381

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `1.76 us` (✅ **1.00x**) | `259.11 us` (❌ *147.54x slower*)    |

### mul_projective_g2_bls12_381

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `1.75 us` (✅ **1.00x**) | `259.19 us` (❌ *147.90x slower*)    |

### mul_affine_g1_bls12_377

|        | `normal`                  | `optimized`                         |
|:-------|:--------------------------|:----------------------------------- |
|        | `640.59 ns` (✅ **1.00x**) | `141.14 us` (❌ *220.33x slower*)    |

### mul_affine_g2_bls12_377

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `2.00 us` (✅ **1.00x**) | `371.17 us` (❌ *185.34x slower*)    |

### mul_projective_g1_bls12_377

|        | `normal`                  | `optimized`                         |
|:-------|:--------------------------|:----------------------------------- |
|        | `643.43 ns` (✅ **1.00x**) | `141.16 us` (❌ *219.39x slower*)    |

### mul_projective_g2_bls12_377

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `2.01 us` (✅ **1.00x**) | `371.30 us` (❌ *184.30x slower*)    |

### pairing_bls12_377

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `2.22 ms` (✅ **1.00x**) | `2.50 ms` (❌ *1.12x slower*)    |

### pairing_msm_g1_bls12_377

|        | `normal`               | `optimized`                     |
|:-------|:-----------------------|:------------------------------- |
|        | `2.38 s` (✅ **1.00x**) | `11.38 s` (❌ *4.79x slower*)    |

### pairing_msm_g2_bls12_377

|        | `normal`               | `optimized`                     |
|:-------|:-----------------------|:------------------------------- |
|        | `8.46 s` (✅ **1.00x**) | `31.55 s` (❌ *3.73x slower*)    |

### msm_g1_bw6_761

|        | `normal`                 | `optimized`                       |
|:-------|:-------------------------|:--------------------------------- |
|        | `99.26 us` (✅ **1.00x**) | `727.43 us` (❌ *7.33x slower*)    |

### msm_g2_bw6_761

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `100.09 us` (✅ **1.00x**) | `726.87 us` (❌ *7.26x slower*)    |

### mul_affine_g1_bw6_761

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `2.26 us` (✅ **1.00x**) | `628.29 us` (❌ *277.44x slower*)    |

### mul_projective_g1_bw6_761

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `2.24 us` (✅ **1.00x**) | `628.51 us` (❌ *279.98x slower*)    |

### pairing_bw6_761

|        | `normal`                | `optimized`                     |
|:-------|:------------------------|:------------------------------- |
|        | `8.68 ms` (✅ **1.00x**) | `9.30 ms` (✅ **1.07x slower**)  |

### mul_affine_g2_bw6_761

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `628.57 us` (✅ **1.00x**) | `627.79 us` (✅ **1.00x faster**)  |

### mul_projective_g2_bw6_761

|        | `normal`                | `optimized`                         |
|:-------|:------------------------|:----------------------------------- |
|        | `2.24 us` (✅ **1.00x**) | `628.09 us` (❌ *279.84x slower*)    |

### msm_ed_on_bls12_377

|        | `normal`                  | `optimized`                      | `normal #2`                      | `optimized #2`                    |
|:-------|:--------------------------|:---------------------------------|:---------------------------------|:--------------------------------- |
|        | `665.27 us` (✅ **1.00x**) | `665.93 us` (✅ **1.00x slower**) | `667.01 us` (✅ **1.00x slower**) | `665.99 us` (✅ **1.00x slower**)  |

### mul_affine_ed_on_bls12_377

|        | `normal`                  | `optimized`                       |
|:-------|:--------------------------|:--------------------------------- |
|        | `665.37 us` (✅ **1.00x**) | `665.91 us` (✅ **1.00x slower**)  |

### mul_projective_ed_on_bls12_377

|        | `normal`                | `optimized`                       |
|:-------|:------------------------|:--------------------------------- |
|        | `1.14 us` (✅ **1.00x**) | `68.35 us` (❌ *59.84x slower*)    |

### msm_ed_on_bls12_381

|        | `normal`                 | `optimized`                      |
|:-------|:-------------------------|:-------------------------------- |
|        | `14.74 us` (✅ **1.00x**) | `61.84 us` (❌ *4.20x slower*)    |

### mul_affine_ed_on_bls12_381

|        | `normal`                  | `optimized`                        |
|:-------|:--------------------------|:---------------------------------- |
|        | `468.46 ns` (✅ **1.00x**) | `47.00 us` (❌ *100.33x slower*)    |

### mul_projective_ed_on_bls12_381

|        | `normal`                  | `optimized`                        |
|:-------|:--------------------------|:---------------------------------- |
|        | `460.41 ns` (✅ **1.00x**) | `47.02 us` (❌ *102.12x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

