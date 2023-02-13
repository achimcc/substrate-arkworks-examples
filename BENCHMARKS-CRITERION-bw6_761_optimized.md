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
| **`addition`**                        | `N/A`                          | `N/A`                           | `4.81 us` (✅ **1.00x**)        | `4.82 us` (✅ **1.00x slower**) | `75.08 ns` (🚀 **64.13x faster**) | `158.08 ns` (🚀 **30.46x faster**) | `27.70 ns` (🚀 **173.80x faster**) | `12.61 ns` (🚀 **381.68x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `4.88 us` (✅ **1.00x**)        | `4.89 us` (✅ **1.00x slower**) | `75.82 ns` (🚀 **64.41x faster**) | `151.89 ns` (🚀 **32.15x faster**) | `25.92 ns` (🚀 **188.41x faster**) | `13.35 ns` (🚀 **365.72x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `3.36 us` (✅ **1.00x**)        | `3.36 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `3.40 us` (✅ **1.00x**)        | `3.41 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                          | `N/A`                           | `2.21 us` (✅ **1.00x**)        | `2.22 us` (✅ **1.00x slower**) | `53.90 ns` (🚀 **41.03x faster**) | `121.01 ns` (🚀 **18.28x faster**) | `19.42 ns` (🚀 **113.90x faster**) | `7.16 ns` (🚀 **308.70x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `1.80 ms` (✅ **1.00x**)        | `1.79 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `70.90 ns` (❌ *3.74x slower*)    | `120.30 ns` (❌ *6.34x slower*)    | `22.86 ns` (❌ *1.20x slower*)     | `18.97 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `2.47 us` (❌ *32.44x slower*)    | `7.89 us` (❌ *103.75x slower*)    | `313.39 ns` (❌ *4.12x slower*)    | `76.05 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `1.78 us` (❌ *26.86x slower*)    | `5.53 us` (❌ *83.28x slower*)     | `244.95 ns` (❌ *3.69x slower*)    | `66.42 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `51.78 us` (❌ *3.56x slower*)    | `60.79 us` (❌ *4.18x slower*)     | `47.52 us` (❌ *3.27x slower*)     | `14.54 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `5.11 us` (❌ *43.66x slower*)    | `16.11 us` (❌ *137.50x slower*)   | `418.64 ns` (❌ *3.57x slower*)    | `117.13 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `5.03 us` (❌ *30.79x slower*)    | `16.07 us` (❌ *98.39x slower*)    | `648.05 ns` (❌ *3.97x slower*)    | `163.34 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `8.65 ns` (✅ **1.00x**)        | `17.19 ns` (❌ *1.99x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.34 ns` (✅ **1.00x**)       | `21.79 ns` (❌ *2.11x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.87 ns` (✅ **1.00x**)        | `4.95 ns` (✅ **1.02x slower**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `4.54 ns` (✅ **1.00x**)        | `4.54 ns` (✅ **1.00x slower**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761_optimized

|                                          | `g1projectivebw6_761`          | `g2projectivebw6_761`            | `froptimized`                       | `fqoptimized`                       | `fq3optimized`                    | `fq6optimized`                    |
|:-----------------------------------------|:-------------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `513.10 ns` (✅ **1.00x**)      | `514.57 ns` (✅ **1.00x slower**) | `58.11 ns` (🚀 **8.83x faster**)     | `171.70 ns` (🚀 **2.99x faster**)    | `516.06 ns` (✅ **1.01x slower**)  | `1.08 us` (❌ *2.11x slower*)      |
| **`serialize_uncompressed`**             | `695.22 ns` (✅ **1.00x**)      | `695.30 ns` (✅ **1.00x slower**) | `56.08 ns` (🚀 **12.40x faster**)    | `169.87 ns` (🚀 **4.09x faster**)    | `515.97 ns` (✅ **1.35x faster**)  | `1.08 us` (❌ *1.56x slower*)      |
| **`deserialize_compressed`**             | `1.59 ms` (✅ **1.00x**)        | `1.59 ms` (✅ **1.00x faster**)   | `92.41 ns` (🚀 **17176.41x faster**) | `342.31 ns` (🚀 **4637.07x faster**) | `1.05 us` (🚀 **1511.92x faster**) | `2.10 us` (🚀 **756.71x faster**)  |
| **`deserialize_compressed_unchecked`**   | `291.83 us` (✅ **1.00x**)      | `291.82 us` (✅ **1.00x faster**) | `92.36 ns` (🚀 **3159.84x faster**)  | `342.27 ns` (🚀 **852.64x faster**)  | `1.05 us` (🚀 **278.01x faster**)  | `2.10 us` (🚀 **139.11x faster**)  |
| **`deserialize_uncompressed`**           | `1.30 ms` (✅ **1.00x**)        | `1.29 ms` (✅ **1.00x faster**)   | `92.21 ns` (🚀 **14048.80x faster**) | `342.19 ns` (🚀 **3785.57x faster**) | `1.05 us` (🚀 **1234.12x faster**) | `2.10 us` (🚀 **617.50x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `764.93 ns` (✅ **1.00x**)      | `770.31 ns` (✅ **1.01x slower**) | `92.16 ns` (🚀 **8.30x faster**)     | `342.12 ns` (🚀 **2.24x faster**)    | `1.05 us` (❌ *1.37x slower*)      | `2.10 us` (❌ *2.74x slower*)      |

### msm_for_bw6_761_optimized

|        | `g1projectivebw6_761`          | `g2projectivebw6_761`           |
|:-------|:-------------------------------|:------------------------------- |
|        | `12.35 s` (✅ **1.00x**)        | `12.36 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761_optimized

|                          | `froptimized`            | `fqoptimized`                    | `fq3optimized`                    |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `67.25 us` (✅ **1.00x**) | `290.36 us` (❌ *4.32x slower*)   | `6.97 ms` (❌ *103.57x slower*)    |
| **`legendre_for_qr`**    | `32.29 us` (✅ **1.00x**) | `292.47 us` (❌ *9.06x slower*)   | `296.96 us` (❌ *9.20x slower*)    |

### bitwise_operations_for_bw6_761_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `5.03 ns` (✅ **1.00x**)        | `5.12 ns` (✅ **1.02x slower**)    |
| **`from_little-endian_bits`** | `83.11 ns` (✅ **1.00x**)       | `166.06 ns` (❌ *2.00x slower*)    |
| **`from_big-endian_bits`**    | `83.16 ns` (✅ **1.00x**)       | `167.11 ns` (❌ *2.01x slower*)    |
| **`comparison`**              | `5.08 ns` (✅ **1.00x**)        | `5.10 ns` (✅ **1.00x slower**)    |
| **`equality`**                | `5.67 ns` (✅ **1.00x**)        | `5.65 ns` (✅ **1.00x faster**)    |
| **`is_zero`**                 | `5.21 ns` (✅ **1.00x**)        | `5.35 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761_optimized

|                   | `froptimized`            | `fqoptimized`                     |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `86.81 ns` (✅ **1.00x**) | `309.46 ns` (❌ *3.56x slower*)    |
| **`into_bigint`** | `47.05 ns` (✅ **1.00x**) | `158.76 ns` (❌ *3.37x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

