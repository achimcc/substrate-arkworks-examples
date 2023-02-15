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
|        | `2.02 ms` (✅ **1.00x**)                 | `2.01 ms` (✅ **1.01x faster**)           |

### arithmetic_for_bw6_761_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebw6_761`          | `g2projectivebw6_761`          | `fq3optimized`                   | `fq6optimized`                    | `fqoptimized`                     | `froptimized`                      |
|:--------------------------------------|:-------------------------------|:--------------------------------|:-------------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `4.82 us` (✅ **1.00x**)        | `4.82 us` (✅ **1.00x slower**) | `77.68 ns` (🚀 **62.01x faster**) | `163.83 ns` (🚀 **29.40x faster**) | `27.71 ns` (🚀 **173.85x faster**) | `12.63 ns` (🚀 **381.28x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `4.89 us` (✅ **1.00x**)        | `4.89 us` (✅ **1.00x faster**) | `77.91 ns` (🚀 **62.72x faster**) | `154.43 ns` (🚀 **31.64x faster**) | `25.93 ns` (🚀 **188.42x faster**) | `13.36 ns` (🚀 **365.85x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `3.36 us` (✅ **1.00x**)        | `3.36 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `3.40 us` (✅ **1.00x**)        | `3.41 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                          | `N/A`                           | `2.22 us` (✅ **1.00x**)        | `2.22 us` (✅ **1.00x slower**) | `53.65 ns` (🚀 **41.31x faster**) | `121.17 ns` (🚀 **18.29x faster**) | `19.38 ns` (🚀 **114.36x faster**) | `7.17 ns` (🚀 **309.30x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `1.80 ms` (✅ **1.00x**)        | `1.80 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `70.89 ns` (❌ *3.41x slower*)    | `120.97 ns` (❌ *5.82x slower*)    | `22.86 ns` (✅ **1.10x slower**)   | `20.79 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `2.47 us` (❌ *32.61x slower*)    | `7.90 us` (❌ *104.19x slower*)    | `313.50 ns` (❌ *4.13x slower*)    | `75.83 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `1.78 us` (❌ *26.88x slower*)    | `5.54 us` (❌ *83.50x slower*)     | `244.67 ns` (❌ *3.69x slower*)    | `66.38 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `51.74 us` (❌ *3.56x slower*)    | `60.82 us` (❌ *4.19x slower*)     | `47.56 us` (❌ *3.27x slower*)     | `14.53 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `5.11 us` (❌ *43.63x slower*)    | `16.09 us` (❌ *137.39x slower*)   | `418.07 ns` (❌ *3.57x slower*)    | `117.14 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `5.04 us` (❌ *30.88x slower*)    | `16.03 us` (❌ *98.15x slower*)    | `648.10 ns` (❌ *3.97x slower*)    | `163.34 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `8.63 ns` (✅ **1.00x**)        | `17.20 ns` (❌ *1.99x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.34 ns` (✅ **1.00x**)       | `21.75 ns` (❌ *2.10x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.87 ns` (✅ **1.00x**)        | `4.95 ns` (✅ **1.02x slower**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `4.54 ns` (✅ **1.00x**)        | `4.55 ns` (✅ **1.00x slower**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761_optimized

|                                          | `g1projectivebw6_761`          | `g2projectivebw6_761`            | `froptimized`                       | `fqoptimized`                       | `fq3optimized`                    | `fq6optimized`                    |
|:-----------------------------------------|:-------------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `513.34 ns` (✅ **1.00x**)      | `514.54 ns` (✅ **1.00x slower**) | `58.17 ns` (🚀 **8.82x faster**)     | `171.66 ns` (🚀 **2.99x faster**)    | `516.01 ns` (✅ **1.01x slower**)  | `1.08 us` (❌ *2.11x slower*)      |
| **`serialize_uncompressed`**             | `695.34 ns` (✅ **1.00x**)      | `695.28 ns` (✅ **1.00x faster**) | `56.07 ns` (🚀 **12.40x faster**)    | `169.93 ns` (🚀 **4.09x faster**)    | `516.08 ns` (✅ **1.35x faster**)  | `1.08 us` (❌ *1.56x slower*)      |
| **`deserialize_compressed`**             | `1.59 ms` (✅ **1.00x**)        | `1.59 ms` (✅ **1.00x faster**)   | `92.22 ns` (🚀 **17217.40x faster**) | `349.02 ns` (🚀 **4549.17x faster**) | `1.05 us` (🚀 **1511.81x faster**) | `2.11 us` (🚀 **753.59x faster**)  |
| **`deserialize_compressed_unchecked`**   | `291.86 us` (✅ **1.00x**)      | `291.82 us` (✅ **1.00x faster**) | `92.25 ns` (🚀 **3163.77x faster**)  | `349.00 ns` (🚀 **836.29x faster**)  | `1.05 us` (🚀 **277.90x faster**)  | `2.11 us` (🚀 **138.51x faster**)  |
| **`deserialize_uncompressed`**           | `1.30 ms` (✅ **1.00x**)        | `1.30 ms` (✅ **1.00x faster**)   | `92.32 ns` (🚀 **14032.56x faster**) | `348.98 ns` (🚀 **3712.09x faster**) | `1.05 us` (🚀 **1233.52x faster**) | `2.11 us` (🚀 **614.79x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `774.75 ns` (✅ **1.00x**)      | `780.65 ns` (✅ **1.01x slower**) | `92.22 ns` (🚀 **8.40x faster**)     | `348.98 ns` (🚀 **2.22x faster**)    | `1.05 us` (❌ *1.36x slower*)      | `2.11 us` (❌ *2.72x slower*)      |

### msm_for_bw6_761_optimized

|        | `g1projectivebw6_761`          | `g2projectivebw6_761`           |
|:-------|:-------------------------------|:------------------------------- |
|        | `12.38 s` (✅ **1.00x**)        | `12.37 s` (✅ **1.00x faster**)  |

### squareroot_for_bw6_761_optimized

|                          | `froptimized`            | `fqoptimized`                    | `fq3optimized`                    |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `67.28 us` (✅ **1.00x**) | `290.33 us` (❌ *4.32x slower*)   | `6.97 ms` (❌ *103.57x slower*)    |
| **`legendre_for_qr`**    | `32.29 us` (✅ **1.00x**) | `292.44 us` (❌ *9.06x slower*)   | `296.97 us` (❌ *9.20x slower*)    |

### bitwise_operations_for_bw6_761_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `5.02 ns` (✅ **1.00x**)        | `5.13 ns` (✅ **1.02x slower**)    |
| **`from_little-endian_bits`** | `83.23 ns` (✅ **1.00x**)       | `166.32 ns` (❌ *2.00x slower*)    |
| **`from_big-endian_bits`**    | `83.07 ns` (✅ **1.00x**)       | `167.36 ns` (❌ *2.01x slower*)    |
| **`comparison`**              | `5.08 ns` (✅ **1.00x**)        | `5.10 ns` (✅ **1.00x slower**)    |
| **`equality`**                | `5.68 ns` (✅ **1.00x**)        | `5.65 ns` (✅ **1.00x faster**)    |
| **`is_zero`**                 | `5.21 ns` (✅ **1.00x**)        | `5.35 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761_optimized

|                   | `froptimized`            | `fqoptimized`                     |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `75.16 ns` (✅ **1.00x**) | `309.48 ns` (❌ *4.12x slower*)    |
| **`into_bigint`** | `47.05 ns` (✅ **1.00x**) | `158.78 ns` (❌ *3.37x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

