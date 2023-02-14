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
| **`addition`**                        | `N/A`                          | `N/A`                           | `4.14 us` (✅ **1.00x**)        | `4.14 us` (✅ **1.00x slower**) | `88.51 ns` (🚀 **46.78x faster**) | `179.48 ns` (🚀 **23.07x faster**) | `29.90 ns` (🚀 **138.49x faster**) | `19.56 ns` (🚀 **211.65x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `4.20 us` (✅ **1.00x**)        | `4.20 us` (✅ **1.00x slower**) | `83.88 ns` (🚀 **50.06x faster**) | `169.49 ns` (🚀 **24.77x faster**) | `30.29 ns` (🚀 **138.59x faster**) | `15.43 ns` (🚀 **272.18x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `2.92 us` (✅ **1.00x**)        | `2.92 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `2.95 us` (✅ **1.00x**)        | `2.96 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                          | `N/A`                           | `1.92 us` (✅ **1.00x**)        | `1.93 us` (✅ **1.00x slower**) | `67.45 ns` (🚀 **28.52x faster**) | `138.66 ns` (🚀 **13.87x faster**) | `21.11 ns` (🚀 **91.10x faster**)  | `11.17 ns` (🚀 **172.13x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `1.55 ms` (✅ **1.00x**)        | `1.55 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `68.61 ns` (❌ *4.09x slower*)    | `124.37 ns` (❌ *7.42x slower*)    | `24.25 ns` (❌ *1.45x slower*)     | `16.76 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `2.16 us` (❌ *31.34x slower*)    | `6.89 us` (❌ *100.00x slower*)    | `270.61 ns` (❌ *3.93x slower*)    | `68.89 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `1.56 us` (❌ *26.20x slower*)    | `4.85 us` (❌ *81.72x slower*)     | `215.52 ns` (❌ *3.63x slower*)    | `59.41 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `50.53 us` (❌ *3.80x slower*)    | `58.33 us` (❌ *4.39x slower*)     | `46.99 us` (❌ *3.53x slower*)     | `13.30 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `4.46 us` (❌ *41.91x slower*)    | `14.04 us` (❌ *131.92x slower*)   | `401.04 ns` (❌ *3.77x slower*)    | `106.40 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `4.40 us` (❌ *27.82x slower*)    | `13.93 us` (❌ *88.08x slower*)    | `568.52 ns` (❌ *3.59x slower*)    | `158.14 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.84 ns` (✅ **1.00x**)        | `15.67 ns` (❌ *2.00x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.69 ns` (✅ **1.00x**)       | `21.01 ns` (❌ *1.97x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.04 ns` (✅ **1.00x**)        | `4.09 ns` (✅ **1.01x slower**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**)        | `3.74 ns` (✅ **1.00x faster**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761_optimized

|                                          | `g1projectivebw6_761`          | `g2projectivebw6_761`            | `froptimized`                       | `fqoptimized`                       | `fq3optimized`                      | `fq6optimized`                    |
|:-----------------------------------------|:-------------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `472.25 ns` (✅ **1.00x**)      | `468.76 ns` (✅ **1.01x faster**) | `50.17 ns` (🚀 **9.41x faster**)     | `156.27 ns` (🚀 **3.02x faster**)    | `467.15 ns` (✅ **1.01x faster**)    | `983.51 ns` (❌ *2.08x slower*)    |
| **`serialize_uncompressed`**             | `629.25 ns` (✅ **1.00x**)      | `631.17 ns` (✅ **1.00x slower**) | `50.20 ns` (🚀 **12.53x faster**)    | `156.29 ns` (🚀 **4.03x faster**)    | `467.20 ns` (✅ **1.35x faster**)    | `983.47 ns` (❌ *1.56x slower*)    |
| **`deserialize_compressed`**             | `1.37 ms` (✅ **1.00x**)        | `1.37 ms` (✅ **1.00x slower**)   | `94.18 ns` (🚀 **14503.11x faster**) | `306.56 ns` (🚀 **4455.78x faster**) | `945.89 ns` (🚀 **1444.11x faster**) | `1.92 us` (🚀 **712.67x faster**)  |
| **`deserialize_compressed_unchecked`**   | `251.88 us` (✅ **1.00x**)      | `251.92 us` (✅ **1.00x slower**) | `94.23 ns` (🚀 **2672.98x faster**)  | `306.54 ns` (🚀 **821.68x faster**)  | `945.84 ns` (🚀 **266.30x faster**)  | `1.92 us` (🚀 **131.40x faster**)  |
| **`deserialize_uncompressed`**           | `1.11 ms` (✅ **1.00x**)        | `1.11 ms` (✅ **1.00x slower**)   | `94.30 ns` (🚀 **11791.53x faster**) | `305.35 ns` (🚀 **3641.41x faster**) | `945.72 ns` (🚀 **1175.74x faster**) | `1.92 us` (🚀 **580.10x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `697.78 ns` (✅ **1.00x**)      | `694.54 ns` (✅ **1.00x faster**) | `94.20 ns` (🚀 **7.41x faster**)     | `306.88 ns` (🚀 **2.27x faster**)    | `941.58 ns` (❌ *1.35x slower*)      | `1.91 us` (❌ *2.73x slower*)      |

### msm_for_bw6_761_optimized

|        | `g1projectivebw6_761`          | `g2projectivebw6_761`           |
|:-------|:-------------------------------|:------------------------------- |
|        | `11.07 s` (✅ **1.00x**)        | `11.08 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761_optimized

|                          | `froptimized`            | `fqoptimized`                    | `fq3optimized`                    |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `64.91 us` (✅ **1.00x**) | `250.71 us` (❌ *3.86x slower*)   | `5.99 ms` (❌ *92.23x slower*)     |
| **`legendre_for_qr`**    | `29.59 us` (✅ **1.00x**) | `250.80 us` (❌ *8.48x slower*)   | `256.42 us` (❌ *8.67x slower*)    |

### bitwise_operations_for_bw6_761_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.19 ns` (✅ **1.00x**)        | `4.25 ns` (✅ **1.01x slower**)    |
| **`from_little-endian_bits`** | `109.31 ns` (✅ **1.00x**)      | `211.62 ns` (❌ *1.94x slower*)    |
| **`from_big-endian_bits`**    | `109.61 ns` (✅ **1.00x**)      | `212.06 ns` (❌ *1.93x slower*)    |
| **`comparison`**              | `4.20 ns` (✅ **1.00x**)        | `4.20 ns` (✅ **1.00x faster**)    |
| **`equality`**                | `4.66 ns` (✅ **1.00x**)        | `4.63 ns` (✅ **1.01x faster**)    |
| **`is_zero`**                 | `4.00 ns` (✅ **1.00x**)        | `4.11 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761_optimized

|                   | `froptimized`            | `fqoptimized`                     |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `79.36 ns` (✅ **1.00x**) | `278.93 ns` (❌ *3.51x slower*)    |
| **`into_bigint`** | `41.57 ns` (✅ **1.00x**) | `143.85 ns` (❌ *3.46x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

