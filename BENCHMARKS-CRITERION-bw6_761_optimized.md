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
| **`addition`**                        | `N/A`                          | `N/A`                           | `4.82 us` (✅ **1.00x**)        | `4.82 us` (✅ **1.00x faster**) | `78.89 ns` (🚀 **61.07x faster**) | `159.30 ns` (🚀 **30.25x faster**) | `27.71 ns` (🚀 **173.86x faster**) | `12.64 ns` (🚀 **381.30x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `4.89 us` (✅ **1.00x**)        | `4.89 us` (✅ **1.00x faster**) | `78.84 ns` (🚀 **61.99x faster**) | `152.42 ns` (🚀 **32.07x faster**) | `25.93 ns` (🚀 **188.47x faster**) | `13.36 ns` (🚀 **365.72x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `3.36 us` (✅ **1.00x**)        | `3.36 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `3.40 us` (✅ **1.00x**)        | `3.41 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                          | `N/A`                           | `2.21 us` (✅ **1.00x**)        | `2.22 us` (✅ **1.00x slower**) | `54.24 ns` (🚀 **40.78x faster**) | `120.60 ns` (🚀 **18.34x faster**) | `19.17 ns` (🚀 **115.40x faster**) | `7.17 ns` (🚀 **308.62x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `1.80 ms` (✅ **1.00x**)        | `1.80 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `72.00 ns` (❌ *3.80x slower*)    | `121.44 ns` (❌ *6.41x slower*)    | `22.88 ns` (❌ *1.21x slower*)     | `18.95 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `2.47 us` (❌ *32.53x slower*)    | `7.89 us` (❌ *104.01x slower*)    | `313.45 ns` (❌ *4.13x slower*)    | `75.90 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `1.78 us` (❌ *26.88x slower*)    | `5.54 us` (❌ *83.40x slower*)     | `244.67 ns` (❌ *3.69x slower*)    | `66.37 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `51.74 us` (❌ *3.56x slower*)    | `60.81 us` (❌ *4.18x slower*)     | `47.70 us` (❌ *3.28x slower*)     | `14.54 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `5.11 us` (❌ *43.52x slower*)    | `16.10 us` (❌ *137.05x slower*)   | `418.67 ns` (❌ *3.56x slower*)    | `117.46 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `5.03 us` (❌ *30.78x slower*)    | `16.05 us` (❌ *98.17x slower*)    | `648.17 ns` (❌ *3.97x slower*)    | `163.46 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `8.65 ns` (✅ **1.00x**)        | `17.20 ns` (❌ *1.99x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.34 ns` (✅ **1.00x**)       | `21.81 ns` (❌ *2.11x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.90 ns` (✅ **1.00x**)        | `4.95 ns` (✅ **1.01x slower**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `4.54 ns` (✅ **1.00x**)        | `4.53 ns` (✅ **1.00x faster**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761_optimized

|                                          | `g1projectivebw6_761`          | `g2projectivebw6_761`            | `froptimized`                       | `fqoptimized`                       | `fq3optimized`                    | `fq6optimized`                    |
|:-----------------------------------------|:-------------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `512.84 ns` (✅ **1.00x**)      | `514.75 ns` (✅ **1.00x slower**) | `58.18 ns` (🚀 **8.81x faster**)     | `171.15 ns` (🚀 **3.00x faster**)    | `515.99 ns` (✅ **1.01x slower**)  | `1.09 us` (❌ *2.12x slower*)      |
| **`serialize_uncompressed`**             | `695.73 ns` (✅ **1.00x**)      | `695.25 ns` (✅ **1.00x faster**) | `56.28 ns` (🚀 **12.36x faster**)    | `170.37 ns` (🚀 **4.08x faster**)    | `515.96 ns` (✅ **1.35x faster**)  | `1.09 us` (❌ *1.56x slower*)      |
| **`deserialize_compressed`**             | `1.59 ms` (✅ **1.00x**)        | `1.59 ms` (✅ **1.00x faster**)   | `92.39 ns` (🚀 **17188.02x faster**) | `342.18 ns` (🚀 **4640.72x faster**) | `1.05 us` (🚀 **1511.37x faster**) | `2.10 us` (🚀 **756.28x faster**)  |
| **`deserialize_compressed_unchecked`**   | `291.87 us` (✅ **1.00x**)      | `291.87 us` (✅ **1.00x slower**) | `92.58 ns` (🚀 **3152.50x faster**)  | `342.13 ns` (🚀 **853.10x faster**)  | `1.05 us` (🚀 **277.78x faster**)  | `2.10 us` (🚀 **139.11x faster**)  |
| **`deserialize_uncompressed`**           | `1.30 ms` (✅ **1.00x**)        | `1.30 ms` (✅ **1.00x faster**)   | `92.39 ns` (🚀 **14024.72x faster**) | `342.11 ns` (🚀 **3787.69x faster**) | `1.05 us` (🚀 **1233.81x faster**) | `2.10 us` (🚀 **617.45x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `765.72 ns` (✅ **1.00x**)      | `772.67 ns` (✅ **1.01x slower**) | `92.51 ns` (🚀 **8.28x faster**)     | `342.18 ns` (🚀 **2.24x faster**)    | `1.05 us` (❌ *1.37x slower*)      | `2.10 us` (❌ *2.74x slower*)      |

### msm_for_bw6_761_optimized

|        | `g1projectivebw6_761`          | `g2projectivebw6_761`           |
|:-------|:-------------------------------|:------------------------------- |
|        | `12.43 s` (✅ **1.00x**)        | `12.44 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761_optimized

|                          | `froptimized`            | `fqoptimized`                    | `fq3optimized`                    |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `67.41 us` (✅ **1.00x**) | `290.44 us` (❌ *4.31x slower*)   | `6.97 ms` (❌ *103.45x slower*)    |
| **`legendre_for_qr`**    | `32.32 us` (✅ **1.00x**) | `292.52 us` (❌ *9.05x slower*)   | `297.04 us` (❌ *9.19x slower*)    |

### bitwise_operations_for_bw6_761_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `5.04 ns` (✅ **1.00x**)        | `5.13 ns` (✅ **1.02x slower**)    |
| **`from_little-endian_bits`** | `83.36 ns` (✅ **1.00x**)       | `176.40 ns` (❌ *2.12x slower*)    |
| **`from_big-endian_bits`**    | `83.35 ns` (✅ **1.00x**)       | `169.67 ns` (❌ *2.04x slower*)    |
| **`comparison`**              | `5.45 ns` (✅ **1.00x**)        | `5.10 ns` (✅ **1.07x faster**)    |
| **`equality`**                | `5.67 ns` (✅ **1.00x**)        | `5.76 ns` (✅ **1.02x slower**)    |
| **`is_zero`**                 | `5.21 ns` (✅ **1.00x**)        | `5.35 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761_optimized

|                   | `froptimized`            | `fqoptimized`                     |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `75.14 ns` (✅ **1.00x**) | `309.57 ns` (❌ *4.12x slower*)    |
| **`into_bigint`** | `47.03 ns` (✅ **1.00x**) | `158.73 ns` (❌ *3.38x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

