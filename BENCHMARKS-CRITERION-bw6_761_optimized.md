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
| **`addition`**                        | `N/A`                          | `N/A`                           | `4.83 us` (✅ **1.00x**)        | `4.82 us` (✅ **1.00x faster**) | `77.66 ns` (🚀 **62.17x faster**) | `160.21 ns` (🚀 **30.14x faster**) | `27.71 ns` (🚀 **174.26x faster**) | `12.62 ns` (🚀 **382.62x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `4.88 us` (✅ **1.00x**)        | `4.89 us` (✅ **1.00x slower**) | `78.32 ns` (🚀 **62.37x faster**) | `152.47 ns` (🚀 **32.04x faster**) | `25.94 ns` (🚀 **188.28x faster**) | `13.36 ns` (🚀 **365.74x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `3.36 us` (✅ **1.00x**)        | `3.36 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `3.40 us` (✅ **1.00x**)        | `3.41 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                          | `N/A`                           | `2.21 us` (✅ **1.00x**)        | `2.21 us` (✅ **1.00x slower**) | `54.47 ns` (🚀 **40.59x faster**) | `119.05 ns` (🚀 **18.57x faster**) | `19.28 ns` (🚀 **114.66x faster**) | `7.13 ns` (🚀 **310.25x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `1.80 ms` (✅ **1.00x**)        | `1.80 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `72.05 ns` (❌ *3.80x slower*)    | `120.38 ns` (❌ *6.35x slower*)    | `22.87 ns` (❌ *1.21x slower*)     | `18.97 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `2.47 us` (❌ *32.48x slower*)    | `7.89 us` (❌ *103.80x slower*)    | `313.39 ns` (❌ *4.12x slower*)    | `76.00 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `1.79 us` (❌ *26.88x slower*)    | `5.53 us` (❌ *83.26x slower*)     | `244.55 ns` (❌ *3.68x slower*)    | `66.43 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `51.76 us` (❌ *3.56x slower*)    | `60.82 us` (❌ *4.19x slower*)     | `47.52 us` (❌ *3.27x slower*)     | `14.52 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `5.12 us` (❌ *43.69x slower*)    | `16.09 us` (❌ *137.37x slower*)   | `418.44 ns` (❌ *3.57x slower*)    | `117.11 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `5.03 us` (❌ *30.80x slower*)    | `16.13 us` (❌ *98.70x slower*)    | `648.62 ns` (❌ *3.97x slower*)    | `163.42 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `8.64 ns` (✅ **1.00x**)        | `17.20 ns` (❌ *1.99x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.34 ns` (✅ **1.00x**)       | `21.81 ns` (❌ *2.11x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.87 ns` (✅ **1.00x**)        | `4.95 ns` (✅ **1.02x slower**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `4.55 ns` (✅ **1.00x**)        | `4.55 ns` (✅ **1.00x faster**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761_optimized

|                                          | `g1projectivebw6_761`          | `g2projectivebw6_761`            | `froptimized`                       | `fqoptimized`                       | `fq3optimized`                    | `fq6optimized`                    |
|:-----------------------------------------|:-------------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `516.31 ns` (✅ **1.00x**)      | `514.48 ns` (✅ **1.00x faster**) | `57.93 ns` (🚀 **8.91x faster**)     | `171.72 ns` (🚀 **3.01x faster**)    | `516.90 ns` (✅ **1.00x slower**)  | `1.08 us` (❌ *2.10x slower*)      |
| **`serialize_uncompressed`**             | `695.04 ns` (✅ **1.00x**)      | `694.49 ns` (✅ **1.00x faster**) | `56.25 ns` (🚀 **12.36x faster**)    | `169.94 ns` (🚀 **4.09x faster**)    | `516.88 ns` (✅ **1.34x faster**)  | `1.08 us` (❌ *1.56x slower*)      |
| **`deserialize_compressed`**             | `1.59 ms` (✅ **1.00x**)        | `1.59 ms` (✅ **1.00x faster**)   | `92.49 ns` (🚀 **17161.27x faster**) | `342.73 ns` (🚀 **4631.20x faster**) | `1.05 us` (🚀 **1507.99x faster**) | `2.10 us` (🚀 **756.23x faster**)  |
| **`deserialize_compressed_unchecked`**   | `291.81 us` (✅ **1.00x**)      | `291.87 us` (✅ **1.00x slower**) | `92.41 ns` (🚀 **3157.76x faster**)  | `342.67 ns` (🚀 **851.58x faster**)  | `1.05 us` (🚀 **277.27x faster**)  | `2.11 us` (🚀 **138.31x faster**)  |
| **`deserialize_uncompressed`**           | `1.30 ms` (✅ **1.00x**)        | `1.30 ms` (✅ **1.00x faster**)   | `92.38 ns` (🚀 **14022.99x faster**) | `344.07 ns` (🚀 **3764.91x faster**) | `1.05 us` (🚀 **1231.11x faster**) | `2.10 us` (🚀 **617.17x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `764.20 ns` (✅ **1.00x**)      | `770.01 ns` (✅ **1.01x slower**) | `92.40 ns` (🚀 **8.27x faster**)     | `342.70 ns` (🚀 **2.23x faster**)    | `1.05 us` (❌ *1.38x slower*)      | `2.10 us` (❌ *2.75x slower*)      |

### msm_for_bw6_761_optimized

|        | `g1projectivebw6_761`          | `g2projectivebw6_761`           |
|:-------|:-------------------------------|:------------------------------- |
|        | `12.34 s` (✅ **1.00x**)        | `12.37 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761_optimized

|                          | `froptimized`            | `fqoptimized`                    | `fq3optimized`                    |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `67.29 us` (✅ **1.00x**) | `290.37 us` (❌ *4.32x slower*)   | `6.97 ms` (❌ *103.53x slower*)    |
| **`legendre_for_qr`**    | `32.29 us` (✅ **1.00x**) | `292.46 us` (❌ *9.06x slower*)   | `296.95 us` (❌ *9.20x slower*)    |

### bitwise_operations_for_bw6_761_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `5.06 ns` (✅ **1.00x**)        | `5.14 ns` (✅ **1.02x slower**)    |
| **`from_little-endian_bits`** | `83.35 ns` (✅ **1.00x**)       | `168.55 ns` (❌ *2.02x slower*)    |
| **`from_big-endian_bits`**    | `83.35 ns` (✅ **1.00x**)       | `167.40 ns` (❌ *2.01x slower*)    |
| **`comparison`**              | `5.08 ns` (✅ **1.00x**)        | `5.10 ns` (✅ **1.00x slower**)    |
| **`equality`**                | `5.67 ns` (✅ **1.00x**)        | `5.65 ns` (✅ **1.00x faster**)    |
| **`is_zero`**                 | `5.21 ns` (✅ **1.00x**)        | `5.34 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761_optimized

|                   | `froptimized`            | `fqoptimized`                     |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `75.14 ns` (✅ **1.00x**) | `311.66 ns` (❌ *4.15x slower*)    |
| **`into_bigint`** | `47.04 ns` (✅ **1.00x**) | `158.75 ns` (❌ *3.37x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

