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
|        | `2.03 ms` (✅ **1.00x**)                 | `2.01 ms` (✅ **1.01x faster**)           |

### arithmetic_for_bw6_761_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebw6_761`          | `g2projectivebw6_761`          | `fq3optimized`                   | `fq6optimized`                    | `fqoptimized`                     | `froptimized`                      |
|:--------------------------------------|:-------------------------------|:--------------------------------|:-------------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `4.82 us` (✅ **1.00x**)        | `4.82 us` (✅ **1.00x slower**) | `79.90 ns` (🚀 **60.28x faster**) | `160.94 ns` (🚀 **29.93x faster**) | `27.68 ns` (🚀 **174.03x faster**) | `12.61 ns` (🚀 **381.91x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `4.88 us` (✅ **1.00x**)        | `4.88 us` (✅ **1.00x slower**) | `78.70 ns` (🚀 **62.02x faster**) | `151.49 ns` (🚀 **32.22x faster**) | `25.85 ns` (🚀 **188.78x faster**) | `13.29 ns` (🚀 **367.22x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `3.35 us` (✅ **1.00x**)        | `3.35 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `3.41 us` (✅ **1.00x**)        | `3.41 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                          | `N/A`                           | `2.22 us` (✅ **1.00x**)        | `2.22 us` (✅ **1.00x slower**) | `54.47 ns` (🚀 **40.70x faster**) | `117.88 ns` (🚀 **18.81x faster**) | `19.17 ns` (🚀 **115.65x faster**) | `7.13 ns` (🚀 **310.82x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `1.81 ms` (✅ **1.00x**)        | `1.81 ms` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `70.11 ns` (❌ *3.81x slower*)    | `118.60 ns` (❌ *6.45x slower*)    | `22.56 ns` (❌ *1.23x slower*)     | `18.39 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `2.49 us` (❌ *32.89x slower*)    | `7.94 us` (❌ *104.82x slower*)    | `310.61 ns` (❌ *4.10x slower*)    | `75.71 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `1.79 us` (❌ *26.91x slower*)    | `5.56 us` (❌ *83.59x slower*)     | `243.46 ns` (❌ *3.66x slower*)    | `66.56 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `51.37 us` (❌ *3.57x slower*)    | `60.52 us` (❌ *4.21x slower*)     | `47.14 us` (❌ *3.28x slower*)     | `14.38 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `5.12 us` (❌ *43.72x slower*)    | `16.15 us` (❌ *137.81x slower*)   | `418.81 ns` (❌ *3.57x slower*)    | `117.16 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `5.08 us` (❌ *31.17x slower*)    | `16.05 us` (❌ *98.60x slower*)    | `653.89 ns` (❌ *4.02x slower*)    | `162.81 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `8.67 ns` (✅ **1.00x**)        | `17.15 ns` (❌ *1.98x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.44 ns` (✅ **1.00x**)       | `21.96 ns` (❌ *2.10x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.87 ns` (✅ **1.00x**)        | `4.95 ns` (✅ **1.02x slower**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `4.54 ns` (✅ **1.00x**)        | `4.53 ns` (✅ **1.00x faster**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761_optimized

|                                          | `g1projectivebw6_761`          | `g2projectivebw6_761`            | `froptimized`                       | `fqoptimized`                       | `fq3optimized`                    | `fq6optimized`                    |
|:-----------------------------------------|:-------------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `519.01 ns` (✅ **1.00x**)      | `519.25 ns` (✅ **1.00x slower**) | `56.41 ns` (🚀 **9.20x faster**)     | `170.81 ns` (🚀 **3.04x faster**)    | `523.55 ns` (✅ **1.01x slower**)  | `1.11 us` (❌ *2.14x slower*)      |
| **`serialize_uncompressed`**             | `705.42 ns` (✅ **1.00x**)      | `705.47 ns` (✅ **1.00x slower**) | `56.31 ns` (🚀 **12.53x faster**)    | `170.19 ns` (🚀 **4.14x faster**)    | `524.03 ns` (✅ **1.35x faster**)  | `1.11 us` (❌ *1.57x slower*)      |
| **`deserialize_compressed`**             | `1.60 ms` (✅ **1.00x**)        | `1.59 ms` (✅ **1.00x faster**)   | `93.48 ns` (🚀 **17082.24x faster**) | `342.76 ns` (🚀 **4658.86x faster**) | `1.07 us` (🚀 **1490.34x faster**) | `2.15 us` (🚀 **742.14x faster**)  |
| **`deserialize_compressed_unchecked`**   | `291.27 us` (✅ **1.00x**)      | `291.28 us` (✅ **1.00x slower**) | `93.45 ns` (🚀 **3116.94x faster**)  | `342.63 ns` (🚀 **850.11x faster**)  | `1.07 us` (🚀 **271.98x faster**)  | `2.15 us` (🚀 **135.37x faster**)  |
| **`deserialize_uncompressed`**           | `1.30 ms` (✅ **1.00x**)        | `1.30 ms` (✅ **1.00x faster**)   | `93.34 ns` (🚀 **13947.88x faster**) | `342.77 ns` (🚀 **3798.21x faster**) | `1.07 us` (🚀 **1216.01x faster**) | `2.17 us` (🚀 **601.24x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `779.06 ns` (✅ **1.00x**)      | `788.72 ns` (✅ **1.01x slower**) | `93.36 ns` (🚀 **8.34x faster**)     | `342.64 ns` (🚀 **2.27x faster**)    | `1.07 us` (❌ *1.37x slower*)      | `2.15 us` (❌ *2.76x slower*)      |

### msm_for_bw6_761_optimized

|        | `g1projectivebw6_761`          | `g2projectivebw6_761`           |
|:-------|:-------------------------------|:------------------------------- |
|        | `12.63 s` (✅ **1.00x**)        | `12.68 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761_optimized

|                          | `froptimized`            | `fqoptimized`                    | `fq3optimized`                    |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `68.03 us` (✅ **1.00x**) | `289.58 us` (❌ *4.26x slower*)   | `6.96 ms` (❌ *102.30x slower*)    |
| **`legendre_for_qr`**    | `31.87 us` (✅ **1.00x**) | `292.33 us` (❌ *9.17x slower*)   | `297.85 us` (❌ *9.35x slower*)    |

### bitwise_operations_for_bw6_761_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `5.02 ns` (✅ **1.00x**)        | `5.13 ns` (✅ **1.02x slower**)    |
| **`from_little-endian_bits`** | `89.74 ns` (✅ **1.00x**)       | `174.01 ns` (❌ *1.94x slower*)    |
| **`from_big-endian_bits`**    | `89.65 ns` (✅ **1.00x**)       | `175.53 ns` (❌ *1.96x slower*)    |
| **`comparison`**              | `5.14 ns` (✅ **1.00x**)        | `5.10 ns` (✅ **1.01x faster**)    |
| **`equality`**                | `5.67 ns` (✅ **1.00x**)        | `5.75 ns` (✅ **1.01x slower**)    |
| **`is_zero`**                 | `5.21 ns` (✅ **1.00x**)        | `5.35 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761_optimized

|                   | `froptimized`            | `fqoptimized`                     |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `75.22 ns` (✅ **1.00x**) | `314.31 ns` (❌ *4.18x slower*)    |
| **`into_bigint`** | `47.25 ns` (✅ **1.00x**) | `157.66 ns` (❌ *3.34x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

