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
| **`addition`**                        | `N/A`                          | `N/A`                           | `4.14 us` (✅ **1.00x**)        | `4.14 us` (✅ **1.00x slower**) | `88.74 ns` (🚀 **46.64x faster**) | `177.91 ns` (🚀 **23.26x faster**) | `31.49 ns` (🚀 **131.44x faster**) | `19.53 ns` (🚀 **211.92x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `4.19 us` (✅ **1.00x**)        | `4.19 us` (✅ **1.00x slower**) | `84.09 ns` (🚀 **49.80x faster**) | `166.38 ns` (🚀 **25.17x faster**) | `29.69 ns` (🚀 **141.02x faster**) | `17.90 ns` (🚀 **233.90x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `2.91 us` (✅ **1.00x**)        | `2.92 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `2.95 us` (✅ **1.00x**)        | `2.96 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                          | `N/A`                           | `1.93 us` (✅ **1.00x**)        | `1.93 us` (✅ **1.00x slower**) | `67.37 ns` (🚀 **28.57x faster**) | `138.06 ns` (🚀 **13.94x faster**) | `21.43 ns` (🚀 **89.83x faster**)  | `11.18 ns` (🚀 **172.25x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `1.55 ms` (✅ **1.00x**)        | `1.54 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `69.55 ns` (❌ *4.15x slower*)    | `124.27 ns` (❌ *7.41x slower*)    | `24.26 ns` (❌ *1.45x slower*)     | `16.76 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `2.16 us` (❌ *31.10x slower*)    | `6.89 us` (❌ *99.20x slower*)     | `271.46 ns` (❌ *3.91x slower*)    | `69.44 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `1.56 us` (❌ *26.21x slower*)    | `4.84 us` (❌ *81.37x slower*)     | `216.79 ns` (❌ *3.65x slower*)    | `59.43 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `50.50 us` (❌ *3.79x slower*)    | `58.24 us` (❌ *4.37x slower*)     | `46.95 us` (❌ *3.52x slower*)     | `13.33 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `4.45 us` (❌ *41.99x slower*)    | `14.05 us` (❌ *132.61x slower*)   | `400.78 ns` (❌ *3.78x slower*)    | `105.94 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `4.39 us` (❌ *27.59x slower*)    | `13.93 us` (❌ *87.46x slower*)    | `569.21 ns` (❌ *3.57x slower*)    | `159.27 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.84 ns` (✅ **1.00x**)        | `15.72 ns` (❌ *2.01x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.62 ns` (✅ **1.00x**)       | `20.95 ns` (❌ *1.97x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.04 ns` (✅ **1.00x**)        | `4.09 ns` (✅ **1.01x slower**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `3.75 ns` (✅ **1.00x**)        | `3.75 ns` (✅ **1.00x slower**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761_optimized

|                                          | `g1projectivebw6_761`          | `g2projectivebw6_761`            | `froptimized`                       | `fqoptimized`                       | `fq3optimized`                      | `fq6optimized`                    |
|:-----------------------------------------|:-------------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `469.14 ns` (✅ **1.00x**)      | `468.81 ns` (✅ **1.00x faster**) | `50.18 ns` (🚀 **9.35x faster**)     | `156.79 ns` (🚀 **2.99x faster**)    | `467.41 ns` (✅ **1.00x faster**)    | `983.03 ns` (❌ *2.10x slower*)    |
| **`serialize_uncompressed`**             | `630.50 ns` (✅ **1.00x**)      | `629.24 ns` (✅ **1.00x faster**) | `50.20 ns` (🚀 **12.56x faster**)    | `157.14 ns` (🚀 **4.01x faster**)    | `467.39 ns` (✅ **1.35x faster**)    | `983.07 ns` (❌ *1.56x slower*)    |
| **`deserialize_compressed`**             | `1.37 ms` (✅ **1.00x**)        | `1.37 ms` (✅ **1.00x faster**)   | `96.18 ns` (🚀 **14207.52x faster**) | `305.61 ns` (🚀 **4471.50x faster**) | `943.00 ns` (🚀 **1449.11x faster**) | `1.90 us` (🚀 **718.01x faster**)  |
| **`deserialize_compressed_unchecked`**   | `252.09 us` (✅ **1.00x**)      | `252.13 us` (✅ **1.00x slower**) | `96.23 ns` (🚀 **2619.72x faster**)  | `305.58 ns` (🚀 **824.95x faster**)  | `942.93 ns` (🚀 **267.35x faster**)  | `1.90 us` (🚀 **132.46x faster**)  |
| **`deserialize_uncompressed`**           | `1.11 ms` (✅ **1.00x**)        | `1.11 ms` (✅ **1.00x faster**)   | `96.23 ns` (🚀 **11569.97x faster**) | `305.96 ns` (🚀 **3639.04x faster**) | `942.69 ns` (🚀 **1181.09x faster**) | `1.90 us` (🚀 **585.19x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `692.29 ns` (✅ **1.00x**)      | `694.48 ns` (✅ **1.00x slower**) | `96.23 ns` (🚀 **7.19x faster**)     | `305.69 ns` (🚀 **2.26x faster**)    | `942.84 ns` (❌ *1.36x slower*)      | `1.91 us` (❌ *2.76x slower*)      |

### msm_for_bw6_761_optimized

|        | `g1projectivebw6_761`          | `g2projectivebw6_761`           |
|:-------|:-------------------------------|:------------------------------- |
|        | `11.21 s` (✅ **1.00x**)        | `11.22 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761_optimized

|                          | `froptimized`            | `fqoptimized`                    | `fq3optimized`                    |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `65.09 us` (✅ **1.00x**) | `251.07 us` (❌ *3.86x slower*)   | `5.99 ms` (❌ *91.96x slower*)     |
| **`legendre_for_qr`**    | `29.57 us` (✅ **1.00x**) | `250.76 us` (❌ *8.48x slower*)   | `256.47 us` (❌ *8.67x slower*)    |

### bitwise_operations_for_bw6_761_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.19 ns` (✅ **1.00x**)        | `4.25 ns` (✅ **1.01x slower**)    |
| **`from_little-endian_bits`** | `107.25 ns` (✅ **1.00x**)      | `211.98 ns` (❌ *1.98x slower*)    |
| **`from_big-endian_bits`**    | `107.20 ns` (✅ **1.00x**)      | `211.79 ns` (❌ *1.98x slower*)    |
| **`comparison`**              | `4.20 ns` (✅ **1.00x**)        | `4.20 ns` (✅ **1.00x faster**)    |
| **`equality`**                | `4.66 ns` (✅ **1.00x**)        | `5.06 ns` (✅ **1.09x slower**)    |
| **`is_zero`**                 | `4.01 ns` (✅ **1.00x**)        | `4.11 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761_optimized

|                   | `froptimized`            | `fqoptimized`                     |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `79.41 ns` (✅ **1.00x**) | `278.03 ns` (❌ *3.50x slower*)    |
| **`into_bigint`** | `41.54 ns` (✅ **1.00x**) | `141.69 ns` (❌ *3.41x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

