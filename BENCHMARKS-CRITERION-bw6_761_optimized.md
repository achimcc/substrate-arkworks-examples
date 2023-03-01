# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [sample_bw6_761_optimized](#sample_bw6_761_optimized)
    - [arithmetic_for_bw6_761_optimized](#arithmetic_for_bw6_761_optimized)
    - [serialization_for_bw6_761_optimized](#serialization_for_bw6_761_optimized)
    - [msm_for_bw6_761_optimized](#msm_for_bw6_761_optimized)
    - [squareroot_for_bw6_761_optimized](#squareroot_for_bw6_761_optimized)
    - [bitwise_operations_for_bw6_761_optimized](#bitwise_operations_for_bw6_761_optimized)
    - [conversions_for_bw6_761_optimized](#conversions_for_bw6_761_optimized)

## Benchmark Results

### sample_bw6_761_optimized

|        | `g1projectivebw6_761_elements`          | `g2projectivebw6_761_elements`           |
|:-------|:----------------------------------------|:---------------------------------------- |
|        | `1.75 ms` (✅ **1.00x**)                 | `1.73 ms` (✅ **1.01x faster**)           |

### arithmetic_for_bw6_761_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebw6_761`          | `g2projectivebw6_761`          | `fq3optimized`                   | `fq6optimized`                    | `fqoptimized`                     | `froptimized`                      |
|:--------------------------------------|:-------------------------------|:--------------------------------|:-------------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `4.14 us` (✅ **1.00x**)        | `4.14 us` (✅ **1.00x slower**) | `90.85 ns` (🚀 **45.54x faster**) | `178.87 ns` (🚀 **23.13x faster**) | `30.01 ns` (🚀 **137.87x faster**) | `19.27 ns` (🚀 **214.69x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `4.18 us` (✅ **1.00x**)        | `4.19 us` (✅ **1.00x slower**) | `81.73 ns` (🚀 **51.20x faster**) | `158.73 ns` (🚀 **26.36x faster**) | `27.81 ns` (🚀 **150.47x faster**) | `15.18 ns` (🚀 **275.67x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `2.92 us` (✅ **1.00x**)        | `2.92 us` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `2.97 us` (✅ **1.00x**)        | `2.97 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                          | `N/A`                           | `1.92 us` (✅ **1.00x**)        | `1.92 us` (✅ **1.00x slower**) | `67.80 ns` (🚀 **28.37x faster**) | `138.35 ns` (🚀 **13.90x faster**) | `22.47 ns` (🚀 **85.59x faster**)  | `11.06 ns` (🚀 **173.88x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `1.54 ms` (✅ **1.00x**)        | `1.54 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `75.00 ns` (❌ *4.48x slower*)    | `123.58 ns` (❌ *7.38x slower*)    | `24.04 ns` (❌ *1.43x slower*)     | `16.76 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `2.16 us` (❌ *31.14x slower*)    | `6.86 us` (❌ *98.97x slower*)     | `269.67 ns` (❌ *3.89x slower*)    | `69.36 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `1.55 us` (❌ *26.15x slower*)    | `4.84 us` (❌ *81.59x slower*)     | `216.14 ns` (❌ *3.65x slower*)    | `59.30 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `50.06 us` (❌ *3.74x slower*)    | `57.74 us` (❌ *4.32x slower*)     | `46.49 us` (❌ *3.47x slower*)     | `13.38 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `4.44 us` (❌ *41.96x slower*)    | `13.98 us` (❌ *132.04x slower*)   | `400.84 ns` (❌ *3.78x slower*)    | `105.91 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `4.39 us` (❌ *28.03x slower*)    | `13.88 us` (❌ *88.64x slower*)    | `568.48 ns` (❌ *3.63x slower*)    | `156.64 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.83 ns` (✅ **1.00x**)        | `15.62 ns` (❌ *2.00x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.62 ns` (✅ **1.00x**)       | `21.16 ns` (❌ *1.99x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.04 ns` (✅ **1.00x**)        | `4.09 ns` (✅ **1.01x slower**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**)        | `3.74 ns` (✅ **1.00x slower**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761_optimized

|                                          | `g1projectivebw6_761`          | `g2projectivebw6_761`            | `froptimized`                       | `fqoptimized`                       | `fq3optimized`                      | `fq6optimized`                    |
|:-----------------------------------------|:-------------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `469.54 ns` (✅ **1.00x**)      | `468.85 ns` (✅ **1.00x faster**) | `50.29 ns` (🚀 **9.34x faster**)     | `155.79 ns` (🚀 **3.01x faster**)    | `465.48 ns` (✅ **1.01x faster**)    | `987.26 ns` (❌ *2.10x slower*)    |
| **`serialize_uncompressed`**             | `631.52 ns` (✅ **1.00x**)      | `629.63 ns` (✅ **1.00x faster**) | `50.16 ns` (🚀 **12.59x faster**)    | `155.79 ns` (🚀 **4.05x faster**)    | `465.43 ns` (✅ **1.36x faster**)    | `987.05 ns` (❌ *1.56x slower*)    |
| **`deserialize_compressed`**             | `1.36 ms` (✅ **1.00x**)        | `1.36 ms` (✅ **1.00x slower**)   | `93.36 ns` (🚀 **14595.43x faster**) | `305.21 ns` (🚀 **4464.37x faster**) | `942.11 ns` (🚀 **1446.32x faster**) | `1.90 us` (🚀 **715.95x faster**)  |
| **`deserialize_compressed_unchecked`**   | `252.23 us` (✅ **1.00x**)      | `252.27 us` (✅ **1.00x slower**) | `93.36 ns` (🚀 **2701.51x faster**)  | `305.23 ns` (🚀 **826.35x faster**)  | `941.63 ns` (🚀 **267.86x faster**)  | `1.90 us` (🚀 **132.53x faster**)  |
| **`deserialize_uncompressed`**           | `1.11 ms` (✅ **1.00x**)        | `1.11 ms` (✅ **1.00x faster**)   | `93.36 ns` (🚀 **11899.38x faster**) | `304.81 ns` (🚀 **3644.63x faster**) | `941.95 ns` (🚀 **1179.37x faster**) | `1.90 us` (🚀 **583.70x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `708.67 ns` (✅ **1.00x**)      | `703.44 ns` (✅ **1.01x faster**) | `93.37 ns` (🚀 **7.59x faster**)     | `304.75 ns` (🚀 **2.33x faster**)    | `942.04 ns` (❌ *1.33x slower*)      | `1.90 us` (❌ *2.69x slower*)      |

### msm_for_bw6_761_optimized

|        | `g1projectivebw6_761`          | `g2projectivebw6_761`           |
|:-------|:-------------------------------|:------------------------------- |
|        | `10.93 s` (✅ **1.00x**)        | `10.96 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761_optimized

|                          | `froptimized`            | `fqoptimized`                    | `fq3optimized`                    |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `64.95 us` (✅ **1.00x**) | `250.96 us` (❌ *3.86x slower*)   | `5.96 ms` (❌ *91.80x slower*)     |
| **`legendre_for_qr`**    | `29.29 us` (✅ **1.00x**) | `252.05 us` (❌ *8.60x slower*)   | `257.62 us` (❌ *8.79x slower*)    |

### bitwise_operations_for_bw6_761_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.19 ns` (✅ **1.00x**)        | `4.25 ns` (✅ **1.01x slower**)    |
| **`from_little-endian_bits`** | `108.68 ns` (✅ **1.00x**)      | `210.87 ns` (❌ *1.94x slower*)    |
| **`from_big-endian_bits`**    | `108.64 ns` (✅ **1.00x**)      | `210.04 ns` (❌ *1.93x slower*)    |
| **`comparison`**              | `4.22 ns` (✅ **1.00x**)        | `4.19 ns` (✅ **1.01x faster**)    |
| **`equality`**                | `4.65 ns` (✅ **1.00x**)        | `5.03 ns` (✅ **1.08x slower**)    |
| **`is_zero`**                 | `4.00 ns` (✅ **1.00x**)        | `4.11 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761_optimized

|                   | `froptimized`            | `fqoptimized`                     |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `79.35 ns` (✅ **1.00x**) | `278.49 ns` (❌ *3.51x slower*)    |
| **`into_bigint`** | `41.54 ns` (✅ **1.00x**) | `141.80 ns` (❌ *3.41x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

