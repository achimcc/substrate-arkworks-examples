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
| **`addition`**                        | `N/A`                          | `N/A`                           | `4.82 us` (✅ **1.00x**)        | `4.82 us` (✅ **1.00x slower**) | `77.46 ns` (🚀 **62.17x faster**) | `161.48 ns` (🚀 **29.82x faster**) | `27.72 ns` (🚀 **173.74x faster**) | `12.63 ns` (🚀 **381.30x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `4.88 us` (✅ **1.00x**)        | `4.89 us` (✅ **1.00x slower**) | `77.70 ns` (🚀 **62.85x faster**) | `153.82 ns` (🚀 **31.75x faster**) | `25.93 ns` (🚀 **188.33x faster**) | `13.35 ns` (🚀 **365.76x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `3.36 us` (✅ **1.00x**)        | `3.36 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `3.40 us` (✅ **1.00x**)        | `3.41 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                          | `N/A`                           | `2.21 us` (✅ **1.00x**)        | `2.22 us` (✅ **1.00x slower**) | `54.42 ns` (🚀 **40.62x faster**) | `118.10 ns` (🚀 **18.72x faster**) | `19.17 ns` (🚀 **115.31x faster**) | `7.17 ns` (🚀 **308.40x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `1.80 ms` (✅ **1.00x**)        | `1.80 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `72.18 ns` (❌ *3.79x slower*)    | `120.32 ns` (❌ *6.32x slower*)    | `22.84 ns` (❌ *1.20x slower*)     | `19.03 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `2.47 us` (❌ *32.53x slower*)    | `7.89 us` (❌ *104.01x slower*)    | `313.27 ns` (❌ *4.13x slower*)    | `75.84 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `1.78 us` (❌ *26.89x slower*)    | `5.53 us` (❌ *83.31x slower*)     | `244.62 ns` (❌ *3.69x slower*)    | `66.36 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `51.68 us` (❌ *3.56x slower*)    | `60.76 us` (❌ *4.18x slower*)     | `47.49 us` (❌ *3.27x slower*)     | `14.54 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `5.12 us` (❌ *43.71x slower*)    | `16.13 us` (❌ *137.68x slower*)   | `418.17 ns` (❌ *3.57x slower*)    | `117.13 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `5.03 us` (❌ *30.81x slower*)    | `16.02 us` (❌ *98.12x slower*)    | `648.66 ns` (❌ *3.97x slower*)    | `163.29 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `8.64 ns` (✅ **1.00x**)        | `17.21 ns` (❌ *1.99x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.34 ns` (✅ **1.00x**)       | `21.80 ns` (❌ *2.11x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.87 ns` (✅ **1.00x**)        | `4.95 ns` (✅ **1.02x slower**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `4.57 ns` (✅ **1.00x**)        | `4.56 ns` (✅ **1.00x faster**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761_optimized

|                                          | `g1projectivebw6_761`          | `g2projectivebw6_761`            | `froptimized`                       | `fqoptimized`                       | `fq3optimized`                    | `fq6optimized`                    |
|:-----------------------------------------|:-------------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `513.18 ns` (✅ **1.00x**)      | `514.52 ns` (✅ **1.00x slower**) | `58.14 ns` (🚀 **8.83x faster**)     | `171.80 ns` (🚀 **2.99x faster**)    | `515.92 ns` (✅ **1.01x slower**)  | `1.08 us` (❌ *2.11x slower*)      |
| **`serialize_uncompressed`**             | `695.35 ns` (✅ **1.00x**)      | `695.37 ns` (✅ **1.00x slower**) | `56.08 ns` (🚀 **12.40x faster**)    | `169.99 ns` (🚀 **4.09x faster**)    | `515.83 ns` (✅ **1.35x faster**)  | `1.08 us` (❌ *1.56x slower*)      |
| **`deserialize_compressed`**             | `1.59 ms` (✅ **1.00x**)        | `1.59 ms` (✅ **1.00x faster**)   | `92.32 ns` (🚀 **17192.86x faster**) | `342.02 ns` (🚀 **4641.01x faster**) | `1.05 us` (🚀 **1512.04x faster**) | `2.10 us` (🚀 **756.76x faster**)  |
| **`deserialize_compressed_unchecked`**   | `291.80 us` (✅ **1.00x**)      | `291.81 us` (✅ **1.00x slower**) | `92.42 ns` (🚀 **3157.49x faster**)  | `341.99 ns` (🚀 **853.26x faster**)  | `1.05 us` (🚀 **278.00x faster**)  | `2.10 us` (🚀 **139.13x faster**)  |
| **`deserialize_uncompressed`**           | `1.30 ms` (✅ **1.00x**)        | `1.29 ms` (✅ **1.00x faster**)   | `92.21 ns` (🚀 **14045.69x faster**) | `342.09 ns` (🚀 **3785.92x faster**) | `1.05 us` (🚀 **1233.66x faster**) | `2.10 us` (🚀 **617.53x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `788.93 ns` (✅ **1.00x**)      | `793.83 ns` (✅ **1.01x slower**) | `92.23 ns` (🚀 **8.55x faster**)     | `342.28 ns` (🚀 **2.30x faster**)    | `1.05 us` (❌ *1.33x slower*)      | `2.10 us` (❌ *2.66x slower*)      |

### msm_for_bw6_761_optimized

|        | `g1projectivebw6_761`          | `g2projectivebw6_761`           |
|:-------|:-------------------------------|:------------------------------- |
|        | `12.31 s` (✅ **1.00x**)        | `12.33 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761_optimized

|                          | `froptimized`            | `fqoptimized`                    | `fq3optimized`                    |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `67.25 us` (✅ **1.00x**) | `290.29 us` (❌ *4.32x slower*)   | `6.98 ms` (❌ *103.77x slower*)    |
| **`legendre_for_qr`**    | `32.29 us` (✅ **1.00x**) | `292.41 us` (❌ *9.06x slower*)   | `296.91 us` (❌ *9.20x slower*)    |

### bitwise_operations_for_bw6_761_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `5.02 ns` (✅ **1.00x**)        | `5.12 ns` (✅ **1.02x slower**)    |
| **`from_little-endian_bits`** | `83.33 ns` (✅ **1.00x**)       | `165.93 ns` (❌ *1.99x slower*)    |
| **`from_big-endian_bits`**    | `83.27 ns` (✅ **1.00x**)       | `165.83 ns` (❌ *1.99x slower*)    |
| **`comparison`**              | `5.08 ns` (✅ **1.00x**)        | `5.10 ns` (✅ **1.00x slower**)    |
| **`equality`**                | `5.67 ns` (✅ **1.00x**)        | `5.71 ns` (✅ **1.01x slower**)    |
| **`is_zero`**                 | `5.21 ns` (✅ **1.00x**)        | `5.34 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761_optimized

|                   | `froptimized`            | `fqoptimized`                     |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `75.11 ns` (✅ **1.00x**) | `309.45 ns` (❌ *4.12x slower*)    |
| **`into_bigint`** | `47.03 ns` (✅ **1.00x**) | `158.73 ns` (❌ *3.38x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

