# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [sample_bls12_381](#sample_bls12_381)
    - [arithmetic_for_bls12_381](#arithmetic_for_bls12_381)
    - [serialization_for_bls12_381](#serialization_for_bls12_381)
    - [msm_for_bls12_381](#msm_for_bls12_381)
    - [squareroot_for_bls12_381](#squareroot_for_bls12_381)
    - [bitwise_operations_for_bls12_381](#bitwise_operations_for_bls12_381)
    - [conversions_for_bls12_381](#conversions_for_bls12_381)

## Benchmark Results

### sample_bls12_381

|        | `g1projective_elements`          | `g2projective_elements`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `218.03 us` (✅ **1.00x**)        | `1.95 ms` (❌ *8.92x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                            | `fq12`                            | `fq`                             | `fr`                               |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.34 us` (✅ **1.00x**)   | `4.36 us` (❌ *3.24x slower*)   | `33.74 ns` (🚀 **39.85x faster**) | `215.75 ns` (🚀 **6.23x faster**)  | `23.33 ns` (🚀 **57.63x faster**) | `9.83 ns` (🚀 **136.79x faster**)   |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.39 us` (✅ **1.00x**)   | `4.42 us` (❌ *3.19x slower*)   | `33.15 ns` (🚀 **41.78x faster**) | `202.55 ns` (🚀 **6.84x faster**)  | `18.08 ns` (🚀 **76.59x faster**) | `10.36 ns` (🚀 **133.65x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `981.64 ns` (✅ **1.00x**) | `3.14 us` (❌ *3.20x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `1.01 us` (✅ **1.00x**)   | `3.18 us` (❌ *3.14x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`double`**                          | `N/A`                   | `N/A`                           | `713.95 ns` (✅ **1.00x**) | `1.99 us` (❌ *2.79x slower*)   | `15.70 ns` (🚀 **45.48x faster**) | `126.24 ns` (🚀 **5.66x faster**)  | `9.14 ns` (🚀 **78.13x faster**)  | `6.53 ns` (🚀 **109.41x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `341.17 us` (✅ **1.00x**) | `1.04 ms` (❌ *3.06x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `27.16 ns` (❌ *3.80x slower*)    | `132.09 ns` (❌ *18.48x slower*)   | `20.13 ns` (❌ *2.82x slower*)    | `7.15 ns` (✅ **1.00x**)            |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `268.00 ns` (❌ *5.77x slower*)   | `6.90 us` (❌ *148.38x slower*)    | `84.39 ns` (❌ *1.82x slower*)    | `46.48 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `208.84 ns` (❌ *4.91x slower*)   | `4.84 us` (❌ *113.81x slower*)    | `69.81 ns` (❌ *1.64x slower*)    | `42.52 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `16.73 us` (❌ *2.17x slower*)    | `27.69 us` (❌ *3.59x slower*)     | `16.26 us` (❌ *2.11x slower*)    | `7.72 us` (✅ **1.00x**)            |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `593.53 ns` (❌ *6.03x slower*)   | `14.03 us` (❌ *142.48x slower*)   | `129.28 ns` (❌ *1.31x slower*)   | `98.50 ns` (✅ **1.00x**)           |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `581.61 ns` (❌ *6.01x slower*)   | `13.98 us` (❌ *144.48x slower*)   | `186.46 ns` (❌ *1.93x slower*)   | `96.76 ns` (✅ **1.00x**)           |
| **`addition_with_carry`**             | `7.86 ns` (✅ **1.00x**) | `9.40 ns` (❌ *1.20x slower*)    | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`subtraction_with_borrow`**         | `9.40 ns` (✅ **1.00x**) | `12.90 ns` (❌ *1.37x slower*)   | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`multiplication_by_2`**             | `4.67 ns` (✅ **1.00x**) | `4.85 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`division_by_2`**                   | `4.50 ns` (✅ **1.00x**) | `4.50 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                                | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:------------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `170.53 ns` (✅ **1.00x**) | `231.31 ns` (❌ *1.36x slower*)   | `35.96 ns` (🚀 **4.74x faster**)    | `59.54 ns` (🚀 **2.86x faster**)     | `117.48 ns` (✅ **1.45x faster**)   | `756.41 ns` (❌ *4.44x slower*)    |
| **`serialize_uncompressed`**             | `217.54 ns` (✅ **1.00x**) | `323.00 ns` (❌ *1.48x slower*)   | `35.84 ns` (🚀 **6.07x faster**)    | `59.39 ns` (🚀 **3.66x faster**)     | `117.47 ns` (🚀 **1.85x faster**)   | `756.23 ns` (❌ *3.48x slower*)    |
| **`deserialize_compressed`**             | `141.80 us` (✅ **1.00x**) | `290.30 us` (❌ *2.05x slower*)   | `55.83 ns` (🚀 **2539.80x faster**) | `113.46 ns` (🚀 **1249.82x faster**) | `250.35 ns` (🚀 **566.42x faster**) | `1.52 us` (🚀 **93.57x faster**)   |
| **`deserialize_compressed_unchecked`**   | `43.46 us` (✅ **1.00x**)  | `147.57 us` (❌ *3.40x slower*)   | `55.88 ns` (🚀 **777.69x faster**)  | `112.96 ns` (🚀 **384.70x faster**)  | `250.43 ns` (🚀 **173.53x faster**) | `1.52 us` (🚀 **28.67x faster**)   |
| **`deserialize_uncompressed`**           | `98.39 us` (✅ **1.00x**)  | `141.88 us` (❌ *1.44x slower*)   | `55.74 ns` (🚀 **1765.34x faster**) | `113.73 ns` (🚀 **865.17x faster**)  | `249.49 ns` (🚀 **394.38x faster**) | `1.52 us` (🚀 **64.64x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `241.14 ns` (✅ **1.00x**) | `511.08 ns` (❌ *2.12x slower*)   | `55.74 ns` (🚀 **4.33x faster**)    | `112.95 ns` (🚀 **2.13x faster**)    | `250.46 ns` (✅ **1.04x slower**)   | `1.53 us` (❌ *6.33x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.68 s` (✅ **1.00x**)  | `8.02 s` (❌ *2.99x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `27.53 us` (✅ **1.00x**) | `42.98 us` (❌ *1.56x slower*)   | `146.50 us` (❌ *5.32x slower*)    |
| **`legendre_for_qr`**    | `14.80 us` (✅ **1.00x**) | `42.90 us` (❌ *2.90x slower*)   | `43.04 us` (❌ *2.91x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.78 ns` (✅ **1.00x**)  | `5.03 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `72.93 ns` (✅ **1.00x**) | `129.66 ns` (❌ *1.78x slower*)    |
| **`from_big-endian_bits`**    | `72.89 ns` (✅ **1.00x**) | `129.56 ns` (❌ *1.78x slower*)    |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)  | `5.17 ns` (✅ **1.06x slower**)    |
| **`equality`**                | `5.38 ns` (✅ **1.00x**)  | `5.60 ns` (✅ **1.04x slower**)    |
| **`is_zero`**                 | `4.69 ns` (✅ **1.00x**)  | `4.81 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `43.31 ns` (✅ **1.00x**) | `95.30 ns` (❌ *2.20x slower*)    |
| **`into_bigint`** | `25.90 ns` (✅ **1.00x**) | `49.75 ns` (❌ *1.92x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

