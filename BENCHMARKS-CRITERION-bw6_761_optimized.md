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
|        | `2.02 ms` (✅ **1.00x**)                 | `2.01 ms` (✅ **1.00x faster**)           |

### arithmetic_for_bw6_761_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebw6_761`          | `g2projectivebw6_761`          | `fq3optimized`                   | `fq6optimized`                    | `fqoptimized`                     | `froptimized`                      |
|:--------------------------------------|:-------------------------------|:--------------------------------|:-------------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `4.82 us` (✅ **1.00x**)        | `4.82 us` (✅ **1.00x slower**) | `80.11 ns` (🚀 **60.11x faster**) | `165.29 ns` (🚀 **29.13x faster**) | `27.66 ns` (🚀 **174.12x faster**) | `12.60 ns` (🚀 **382.17x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `4.87 us` (✅ **1.00x**)        | `4.87 us` (✅ **1.00x slower**) | `79.13 ns` (🚀 **61.52x faster**) | `153.67 ns` (🚀 **31.68x faster**) | `25.84 ns` (🚀 **188.38x faster**) | `13.28 ns` (🚀 **366.58x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `3.35 us` (✅ **1.00x**)        | `3.35 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `3.43 us` (✅ **1.00x**)        | `3.41 us` (✅ **1.01x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                          | `N/A`                           | `2.22 us` (✅ **1.00x**)        | `2.22 us` (✅ **1.00x slower**) | `55.92 ns` (🚀 **39.64x faster**) | `119.94 ns` (🚀 **18.48x faster**) | `19.19 ns` (🚀 **115.53x faster**) | `7.15 ns` (🚀 **309.96x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `1.81 ms` (✅ **1.00x**)        | `1.81 ms` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `71.98 ns` (❌ *3.84x slower*)    | `122.08 ns` (❌ *6.51x slower*)    | `23.46 ns` (❌ *1.25x slower*)     | `18.74 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `2.49 us` (❌ *32.95x slower*)    | `7.93 us` (❌ *104.94x slower*)    | `310.64 ns` (❌ *4.11x slower*)    | `75.54 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `1.78 us` (❌ *26.77x slower*)    | `5.55 us` (❌ *83.24x slower*)     | `243.47 ns` (❌ *3.65x slower*)    | `66.65 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `51.49 us` (❌ *3.58x slower*)    | `60.40 us` (❌ *4.20x slower*)     | `47.13 us` (❌ *3.28x slower*)     | `14.37 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `5.12 us` (❌ *43.71x slower*)    | `16.13 us` (❌ *137.68x slower*)   | `418.35 ns` (❌ *3.57x slower*)    | `117.15 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `5.08 us` (❌ *31.18x slower*)    | `16.05 us` (❌ *98.58x slower*)    | `653.48 ns` (❌ *4.01x slower*)    | `162.84 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `8.65 ns` (✅ **1.00x**)        | `17.16 ns` (❌ *1.98x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.43 ns` (✅ **1.00x**)       | `21.89 ns` (❌ *2.10x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.87 ns` (✅ **1.00x**)        | `4.95 ns` (✅ **1.02x slower**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `4.55 ns` (✅ **1.00x**)        | `4.55 ns` (✅ **1.00x faster**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761_optimized

|                                          | `g1projectivebw6_761`          | `g2projectivebw6_761`            | `froptimized`                       | `fqoptimized`                       | `fq3optimized`                    | `fq6optimized`                    |
|:-----------------------------------------|:-------------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `518.87 ns` (✅ **1.00x**)      | `520.61 ns` (✅ **1.00x slower**) | `56.44 ns` (🚀 **9.19x faster**)     | `170.92 ns` (🚀 **3.04x faster**)    | `525.17 ns` (✅ **1.01x slower**)  | `1.11 us` (❌ *2.14x slower*)      |
| **`serialize_uncompressed`**             | `705.11 ns` (✅ **1.00x**)      | `706.45 ns` (✅ **1.00x slower**) | `56.31 ns` (🚀 **12.52x faster**)    | `170.21 ns` (🚀 **4.14x faster**)    | `525.14 ns` (✅ **1.34x faster**)  | `1.11 us` (❌ *1.57x slower*)      |
| **`deserialize_compressed`**             | `1.59 ms` (✅ **1.00x**)        | `1.59 ms` (✅ **1.00x faster**)   | `92.88 ns` (🚀 **17128.81x faster**) | `342.77 ns` (🚀 **4641.36x faster**) | `1.08 us` (🚀 **1475.39x faster**) | `2.15 us` (🚀 **741.14x faster**)  |
| **`deserialize_compressed_unchecked`**   | `291.33 us` (✅ **1.00x**)      | `291.39 us` (✅ **1.00x slower**) | `92.78 ns` (🚀 **3139.83x faster**)  | `342.63 ns` (🚀 **850.27x faster**)  | `1.08 us` (🚀 **270.20x faster**)  | `2.15 us` (🚀 **135.71x faster**)  |
| **`deserialize_uncompressed`**           | `1.30 ms` (✅ **1.00x**)        | `1.30 ms` (✅ **1.00x faster**)   | `92.80 ns` (🚀 **14007.99x faster**) | `342.68 ns` (🚀 **3793.33x faster**) | `1.08 us` (🚀 **1205.24x faster**) | `2.15 us` (🚀 **605.55x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `771.61 ns` (✅ **1.00x**)      | `781.56 ns` (✅ **1.01x slower**) | `93.23 ns` (🚀 **8.28x faster**)     | `342.67 ns` (🚀 **2.25x faster**)    | `1.08 us` (❌ *1.40x slower*)      | `2.15 us` (❌ *2.78x slower*)      |

### msm_for_bw6_761_optimized

|        | `g1projectivebw6_761`          | `g2projectivebw6_761`           |
|:-------|:-------------------------------|:------------------------------- |
|        | `12.54 s` (✅ **1.00x**)        | `12.56 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761_optimized

|                          | `froptimized`            | `fqoptimized`                    | `fq3optimized`                    |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `68.01 us` (✅ **1.00x**) | `289.54 us` (❌ *4.26x slower*)   | `6.95 ms` (❌ *102.17x slower*)    |
| **`legendre_for_qr`**    | `31.85 us` (✅ **1.00x**) | `292.24 us` (❌ *9.18x slower*)   | `297.81 us` (❌ *9.35x slower*)    |

### bitwise_operations_for_bw6_761_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `5.02 ns` (✅ **1.00x**)        | `5.13 ns` (✅ **1.02x slower**)    |
| **`from_little-endian_bits`** | `89.65 ns` (✅ **1.00x**)       | `165.58 ns` (❌ *1.85x slower*)    |
| **`from_big-endian_bits`**    | `89.51 ns` (✅ **1.00x**)       | `167.16 ns` (❌ *1.87x slower*)    |
| **`comparison`**              | `5.14 ns` (✅ **1.00x**)        | `5.10 ns` (✅ **1.01x faster**)    |
| **`equality`**                | `5.67 ns` (✅ **1.00x**)        | `5.76 ns` (✅ **1.02x slower**)    |
| **`is_zero`**                 | `5.21 ns` (✅ **1.00x**)        | `5.35 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761_optimized

|                   | `froptimized`            | `fqoptimized`                     |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `75.55 ns` (✅ **1.00x**) | `313.76 ns` (❌ *4.15x slower*)    |
| **`into_bigint`** | `47.23 ns` (✅ **1.00x**) | `157.59 ns` (❌ *3.34x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

