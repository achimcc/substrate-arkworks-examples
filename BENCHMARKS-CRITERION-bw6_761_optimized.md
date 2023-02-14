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
| **`addition`**                        | `N/A`                          | `N/A`                           | `4.14 us` (✅ **1.00x**)        | `4.14 us` (✅ **1.00x slower**) | `89.00 ns` (🚀 **46.48x faster**) | `176.54 ns` (🚀 **23.43x faster**) | `29.84 ns` (🚀 **138.62x faster**) | `19.52 ns` (🚀 **211.89x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `4.19 us` (✅ **1.00x**)        | `4.19 us` (✅ **1.00x slower**) | `84.16 ns` (🚀 **49.79x faster**) | `165.40 ns` (🚀 **25.34x faster**) | `29.71 ns` (🚀 **141.03x faster**) | `16.28 ns` (🚀 **257.41x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `2.92 us` (✅ **1.00x**)        | `2.92 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `2.95 us` (✅ **1.00x**)        | `2.96 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                          | `N/A`                           | `1.93 us` (✅ **1.00x**)        | `1.93 us` (✅ **1.00x slower**) | `67.70 ns` (🚀 **28.44x faster**) | `137.58 ns` (🚀 **14.00x faster**) | `21.32 ns` (🚀 **90.33x faster**)  | `11.18 ns` (🚀 **172.25x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `1.55 ms` (✅ **1.00x**)        | `1.54 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `69.53 ns` (❌ *4.07x slower*)    | `123.01 ns` (❌ *7.19x slower*)    | `24.29 ns` (❌ *1.42x slower*)     | `17.10 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `2.16 us` (❌ *31.10x slower*)    | `6.88 us` (❌ *99.12x slower*)     | `271.36 ns` (❌ *3.91x slower*)    | `69.40 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `1.56 us` (❌ *26.30x slower*)    | `4.85 us` (❌ *82.01x slower*)     | `216.56 ns` (❌ *3.66x slower*)    | `59.18 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `50.46 us` (❌ *3.80x slower*)    | `58.23 us` (❌ *4.39x slower*)     | `46.93 us` (❌ *3.54x slower*)     | `13.27 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `4.45 us` (❌ *41.98x slower*)    | `14.03 us` (❌ *132.47x slower*)   | `400.76 ns` (❌ *3.78x slower*)    | `105.91 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `4.39 us` (❌ *27.59x slower*)    | `13.92 us` (❌ *87.44x slower*)    | `569.74 ns` (❌ *3.58x slower*)    | `159.19 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.83 ns` (✅ **1.00x**)        | `15.69 ns` (❌ *2.00x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.62 ns` (✅ **1.00x**)       | `21.05 ns` (❌ *1.98x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.04 ns` (✅ **1.00x**)        | `4.09 ns` (✅ **1.01x slower**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**)        | `3.75 ns` (✅ **1.00x slower**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761_optimized

|                                          | `g1projectivebw6_761`          | `g2projectivebw6_761`            | `froptimized`                       | `fqoptimized`                       | `fq3optimized`                      | `fq6optimized`                    |
|:-----------------------------------------|:-------------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `467.74 ns` (✅ **1.00x**)      | `468.52 ns` (✅ **1.00x slower**) | `50.15 ns` (🚀 **9.33x faster**)     | `156.53 ns` (🚀 **2.99x faster**)    | `466.96 ns` (✅ **1.00x faster**)    | `982.70 ns` (❌ *2.10x slower*)    |
| **`serialize_uncompressed`**             | `628.83 ns` (✅ **1.00x**)      | `628.67 ns` (✅ **1.00x faster**) | `50.17 ns` (🚀 **12.53x faster**)    | `156.97 ns` (🚀 **4.01x faster**)    | `468.68 ns` (✅ **1.34x faster**)    | `982.72 ns` (❌ *1.56x slower*)    |
| **`deserialize_compressed`**             | `1.37 ms` (✅ **1.00x**)        | `1.36 ms` (✅ **1.00x faster**)   | `93.82 ns` (🚀 **14550.21x faster**) | `304.11 ns` (🚀 **4488.76x faster**) | `945.34 ns` (🚀 **1444.03x faster**) | `1.90 us` (🚀 **717.82x faster**)  |
| **`deserialize_compressed_unchecked`**   | `251.86 us` (✅ **1.00x**)      | `251.85 us` (✅ **1.00x faster**) | `93.81 ns` (🚀 **2684.69x faster**)  | `304.12 ns` (🚀 **828.16x faster**)  | `941.27 ns` (🚀 **267.58x faster**)  | `1.90 us` (🚀 **132.44x faster**)  |
| **`deserialize_uncompressed`**           | `1.11 ms` (✅ **1.00x**)        | `1.11 ms` (✅ **1.00x faster**)   | `93.75 ns` (🚀 **11866.42x faster**) | `304.34 ns` (🚀 **3655.45x faster**) | `941.33 ns` (🚀 **1181.83x faster**) | `1.90 us` (🚀 **585.38x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `698.21 ns` (✅ **1.00x**)      | `698.52 ns` (✅ **1.00x slower**) | `93.75 ns` (🚀 **7.45x faster**)     | `304.31 ns` (🚀 **2.29x faster**)    | `941.36 ns` (❌ *1.35x slower*)      | `1.90 us` (❌ *2.72x slower*)      |

### msm_for_bw6_761_optimized

|        | `g1projectivebw6_761`          | `g2projectivebw6_761`           |
|:-------|:-------------------------------|:------------------------------- |
|        | `10.94 s` (✅ **1.00x**)        | `10.94 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761_optimized

|                          | `froptimized`            | `fqoptimized`                    | `fq3optimized`                    |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `64.80 us` (✅ **1.00x**) | `250.68 us` (❌ *3.87x slower*)   | `5.98 ms` (❌ *92.34x slower*)     |
| **`legendre_for_qr`**    | `29.54 us` (✅ **1.00x**) | `250.66 us` (❌ *8.49x slower*)   | `256.29 us` (❌ *8.68x slower*)    |

### bitwise_operations_for_bw6_761_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.19 ns` (✅ **1.00x**)        | `4.24 ns` (✅ **1.01x slower**)    |
| **`from_little-endian_bits`** | `107.64 ns` (✅ **1.00x**)      | `211.43 ns` (❌ *1.96x slower*)    |
| **`from_big-endian_bits`**    | `107.45 ns` (✅ **1.00x**)      | `211.47 ns` (❌ *1.97x slower*)    |
| **`comparison`**              | `4.20 ns` (✅ **1.00x**)        | `4.19 ns` (✅ **1.00x faster**)    |
| **`equality`**                | `4.66 ns` (✅ **1.00x**)        | `4.65 ns` (✅ **1.00x faster**)    |
| **`is_zero`**                 | `4.00 ns` (✅ **1.00x**)        | `4.11 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761_optimized

|                   | `froptimized`            | `fqoptimized`                     |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `79.30 ns` (✅ **1.00x**) | `277.58 ns` (❌ *3.50x slower*)    |
| **`into_bigint`** | `41.52 ns` (✅ **1.00x**) | `143.71 ns` (❌ *3.46x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

