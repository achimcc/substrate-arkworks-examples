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
|        | `204.83 us` (✅ **1.00x**)        | `1.81 ms` (❌ *8.83x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                   | `fq2`                            | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.22 us` (✅ **1.00x**)   | `4.00 us` (❌ *3.27x slower*)     | `23.42 ns` (🚀 **52.26x faster**) | `191.68 ns` (🚀 **6.38x faster**)  | `12.70 ns` (🚀 **96.38x faster**) | `8.66 ns` (🚀 **141.26x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.26 us` (✅ **1.00x**)   | `4.05 us` (❌ *3.21x slower*)     | `23.41 ns` (🚀 **53.98x faster**) | `158.61 ns` (🚀 **7.97x faster**)  | `12.82 ns` (🚀 **98.60x faster**) | `8.78 ns` (🚀 **143.92x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `892.81 ns` (✅ **1.00x**) | `2.87 us` (❌ *3.21x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `931.00 ns` (✅ **1.00x**) | `2.90 us` (❌ *3.11x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `586.95 ns` (✅ **1.00x**) | `1.81 us` (❌ *3.08x slower*)     | `12.53 ns` (🚀 **46.86x faster**) | `73.87 ns` (🚀 **7.95x faster**)   | `7.26 ns` (🚀 **80.89x faster**)  | `6.11 ns` (🚀 **96.04x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `325.07 us` (✅ **1.00x**) | `968.13 us` (❌ *2.98x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `22.14 ns` (❌ *3.60x slower*)    | `96.44 ns` (❌ *15.70x slower*)    | `18.20 ns` (❌ *2.96x slower*)    | `6.14 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `244.28 ns` (❌ *5.29x slower*)   | `6.26 us` (❌ *135.54x slower*)    | `76.99 ns` (❌ *1.67x slower*)    | `46.20 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `176.55 ns` (❌ *4.66x slower*)   | `4.38 us` (❌ *115.72x slower*)    | `65.55 ns` (❌ *1.73x slower*)    | `37.88 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `15.21 us` (❌ *2.14x slower*)    | `25.44 us` (❌ *3.58x slower*)     | `14.90 us` (❌ *2.09x slower*)    | `7.12 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `528.16 ns` (❌ *6.30x slower*)   | `12.84 us` (❌ *153.07x slower*)   | `116.44 ns` (❌ *1.39x slower*)   | `83.86 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `519.14 ns` (❌ *5.90x slower*)   | `12.75 us` (❌ *144.83x slower*)   | `164.28 ns` (❌ *1.87x slower*)   | `88.01 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.62 ns` (✅ **1.00x**) | `8.64 ns` (❌ *1.13x slower*)    | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.65 ns` (✅ **1.00x**) | `10.36 ns` (❌ *1.20x slower*)   | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.87 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.54 ns` (✅ **1.00x**) | `4.57 ns` (✅ **1.01x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `151.76 ns` (✅ **1.00x**) | `203.94 ns` (❌ *1.34x slower*)   | `31.54 ns` (🚀 **4.81x faster**)    | `54.73 ns` (🚀 **2.77x faster**)    | `109.09 ns` (✅ **1.39x faster**)   | `697.25 ns` (❌ *4.59x slower*)    |
| **`serialize_uncompressed`**             | `192.67 ns` (✅ **1.00x**) | `282.96 ns` (❌ *1.47x slower*)   | `31.71 ns` (🚀 **6.08x faster**)    | `55.17 ns` (🚀 **3.49x faster**)    | `109.07 ns` (✅ **1.77x faster**)   | `698.08 ns` (❌ *3.62x slower*)    |
| **`deserialize_compressed`**             | `133.36 us` (✅ **1.00x**) | `267.18 us` (❌ *2.00x slower*)   | `53.00 ns` (🚀 **2516.35x faster**) | `94.47 ns` (🚀 **1411.78x faster**) | `218.36 ns` (🚀 **610.76x faster**) | `1.31 us` (🚀 **101.95x faster**)  |
| **`deserialize_compressed_unchecked`**   | `39.54 us` (✅ **1.00x**)  | `134.87 us` (❌ *3.41x slower*)   | `53.02 ns` (🚀 **745.72x faster**)  | `94.51 ns` (🚀 **418.35x faster**)  | `215.45 ns` (🚀 **183.51x faster**) | `1.31 us` (🚀 **30.19x faster**)   |
| **`deserialize_uncompressed`**           | `93.77 us` (✅ **1.00x**)  | `131.82 us` (❌ *1.41x slower*)   | `52.93 ns` (🚀 **1771.60x faster**) | `94.24 ns` (🚀 **995.00x faster**)  | `215.60 ns` (🚀 **434.93x faster**) | `1.31 us` (🚀 **71.80x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `194.08 ns` (✅ **1.00x**) | `405.10 ns` (❌ *2.09x slower*)   | `52.93 ns` (🚀 **3.67x faster**)    | `94.42 ns` (🚀 **2.06x faster**)    | `215.62 ns` (✅ **1.11x slower**)   | `1.31 us` (❌ *6.73x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.34 s` (✅ **1.00x**)  | `7.06 s` (❌ *3.02x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `25.33 us` (✅ **1.00x**) | `39.04 us` (❌ *1.54x slower*)   | `133.81 us` (❌ *5.28x slower*)    |
| **`legendre_for_qr`**    | `14.33 us` (✅ **1.00x**) | `39.28 us` (❌ *2.74x slower*)   | `39.31 us` (❌ *2.74x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)  | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.38 ns` (✅ **1.00x**) | `83.27 ns` (❌ *1.72x slower*)    |
| **`from_big-endian_bits`**    | `48.19 ns` (✅ **1.00x**) | `83.18 ns` (❌ *1.73x slower*)    |
| **`comparison`**              | `4.87 ns` (✅ **1.00x**)  | `5.13 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.45 ns` (✅ **1.00x**)  | `5.67 ns` (✅ **1.04x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.30 ns` (✅ **1.00x**) | `76.37 ns` (❌ *1.85x slower*)    |
| **`into_bigint`** | `22.42 ns` (✅ **1.00x**) | `47.94 ns` (❌ *2.14x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

