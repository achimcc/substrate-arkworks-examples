# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [sample_bls12_377_optimized](#sample_bls12_377_optimized)
    - [arithmetic_for_bls12_377_optimized](#arithmetic_for_bls12_377_optimized)
    - [serialization_for_bls12_377_optimized](#serialization_for_bls12_377_optimized)
    - [msm_for_bls12_377_optimized](#msm_for_bls12_377_optimized)
    - [squareroot_for_bls12_377_optimized](#squareroot_for_bls12_377_optimized)
    - [bitwise_operations_for_bls12_377_optimized](#bitwise_operations_for_bls12_377_optimized)
    - [conversions_for_bls12_377_optimized](#conversions_for_bls12_377_optimized)

## Benchmark Results

### sample_bls12_377_optimized

|        | `g1projectivebls12_377_elements`          | `g2projectivebls12_377_elements`           |
|:-------|:------------------------------------------|:------------------------------------------ |
|        | `193.13 us` (✅ **1.00x**)                 | `1.88 ms` (❌ *9.72x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                    | `fq12optimized`                   | `fqoptimized`                    | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.13 us` (✅ **1.00x**)          | `4.27 us` (❌ *3.77x slower*)     | `28.89 ns` (🚀 **39.19x faster**)  | `178.96 ns` (🚀 **6.33x faster**)  | `19.13 ns` (🚀 **59.18x faster**) | `8.28 ns` (🚀 **136.68x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.16 us` (✅ **1.00x**)          | `4.31 us` (❌ *3.70x slower*)     | `27.60 ns` (🚀 **42.21x faster**)  | `172.24 ns` (🚀 **6.76x faster**)  | `14.94 ns` (🚀 **77.96x faster**) | `8.61 ns` (🚀 **135.26x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `827.28 ns` (✅ **1.00x**)        | `3.07 us` (❌ *3.71x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `854.13 ns` (✅ **1.00x**)        | `3.10 us` (❌ *3.63x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `572.44 ns` (✅ **1.00x**)        | `2.04 us` (❌ *3.56x slower*)     | `12.81 ns` (🚀 **44.69x faster**)  | `104.55 ns` (🚀 **5.48x faster**)  | `11.06 ns` (🚀 **51.75x faster**) | `9.11 ns` (🚀 **62.86x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `299.25 us` (✅ **1.00x**)        | `1.08 ms` (❌ *3.61x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `22.60 ns` (❌ *3.80x slower*)     | `104.78 ns` (❌ *17.63x slower*)   | `16.75 ns` (❌ *2.82x slower*)    | `5.94 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `267.09 ns` (❌ *7.15x slower*)    | `6.67 us` (❌ *178.66x slower*)    | `69.36 ns` (❌ *1.86x slower*)    | `37.34 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `246.62 ns` (❌ *7.55x slower*)    | `4.69 us` (❌ *143.68x slower*)    | `59.13 ns` (❌ *1.81x slower*)    | `32.67 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `13.68 us` (❌ *2.16x slower*)     | `24.93 us` (❌ *3.94x slower*)     | `13.35 us` (❌ *2.11x slower*)    | `6.32 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `570.07 ns` (❌ *10.75x slower*)   | `13.61 us` (❌ *256.82x slower*)   | `111.78 ns` (❌ *2.11x slower*)   | `53.01 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `550.99 ns` (❌ *6.72x slower*)    | `13.53 us` (❌ *164.93x slower*)   | `156.74 ns` (❌ *1.91x slower*)   | `82.01 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.52 ns` (✅ **1.00x**)        | `7.84 ns` (❌ *1.20x slower*)    | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**)        | `10.62 ns` (❌ *1.35x slower*)   | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**)        | `4.04 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**)        | `3.74 ns` (✅ **1.00x faster**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `150.83 ns` (✅ **1.00x**)        | `210.97 ns` (❌ *1.40x slower*)   | `28.04 ns` (🚀 **5.38x faster**)    | `50.29 ns` (🚀 **3.00x faster**)    | `100.18 ns` (✅ **1.51x faster**)    | `627.09 ns` (❌ *4.16x slower*)    |
| **`serialize_uncompressed`**             | `196.49 ns` (✅ **1.00x**)        | `319.73 ns` (❌ *1.63x slower*)   | `27.91 ns` (🚀 **7.04x faster**)    | `50.08 ns` (🚀 **3.92x faster**)    | `100.13 ns` (🚀 **1.96x faster**)    | `626.87 ns` (❌ *3.19x slower*)    |
| **`deserialize_compressed`**             | `280.97 us` (✅ **1.00x**)        | `975.30 us` (❌ *3.47x slower*)   | `47.08 ns` (🚀 **5967.98x faster**) | `96.67 ns` (🚀 **2906.50x faster**) | `205.15 ns` (🚀 **1369.59x faster**) | `1.26 us` (🚀 **222.26x faster**)  |
| **`deserialize_compressed_unchecked`**   | `65.15 us` (✅ **1.00x**)         | `174.48 us` (❌ *2.68x slower*)   | `47.08 ns` (🚀 **1384.00x faster**) | `96.03 ns` (🚀 **678.43x faster**)  | `205.14 ns` (🚀 **317.61x faster**)  | `1.27 us` (🚀 **51.43x faster**)   |
| **`deserialize_uncompressed`**           | `216.19 us` (✅ **1.00x**)        | `796.94 us` (❌ *3.69x slower*)   | `47.03 ns` (🚀 **4596.59x faster**) | `95.70 ns` (🚀 **2259.00x faster**) | `205.36 ns` (🚀 **1052.77x faster**) | `1.26 us` (🚀 **171.05x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `225.19 ns` (✅ **1.00x**)        | `468.70 ns` (❌ *2.08x slower*)   | `47.03 ns` (🚀 **4.79x faster**)    | `95.70 ns` (🚀 **2.35x faster**)    | `205.35 ns` (✅ **1.10x faster**)    | `1.26 us` (❌ *5.61x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.26 s` (✅ **1.00x**)           | `7.90 s` (❌ *3.50x slower*)       |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `27.73 us` (✅ **1.00x**) | `64.81 us` (❌ *2.34x slower*)   | `173.24 us` (❌ *6.25x slower*)    |
| **`legendre_for_qr`**    | `9.54 us` (✅ **1.00x**)  | `29.35 us` (❌ *3.08x slower*)   | `29.77 us` (❌ *3.12x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)        | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `60.53 ns` (✅ **1.00x**)       | `107.61 ns` (❌ *1.78x slower*)    |
| **`from_big-endian_bits`**    | `60.57 ns` (✅ **1.00x**)       | `107.68 ns` (❌ *1.78x slower*)    |
| **`comparison`**              | `4.09 ns` (✅ **1.00x**)        | `4.31 ns` (✅ **1.05x slower**)    |
| **`equality`**                | `4.51 ns` (✅ **1.00x**)        | `4.65 ns` (✅ **1.03x slower**)    |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)        | `4.00 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.95 ns` (✅ **1.00x**) | `79.35 ns` (❌ *2.21x slower*)    |
| **`into_bigint`** | `21.66 ns` (✅ **1.00x**) | `41.50 ns` (❌ *1.92x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

