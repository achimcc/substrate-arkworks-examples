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
| **`addition`**                        | `N/A`                          | `N/A`                           | `4.81 us` (✅ **1.00x**)        | `4.81 us` (✅ **1.00x slower**) | `78.06 ns` (🚀 **61.56x faster**) | `160.43 ns` (🚀 **29.95x faster**) | `27.68 ns` (🚀 **173.59x faster**) | `12.69 ns` (🚀 **378.82x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `4.87 us` (✅ **1.00x**)        | `4.88 us` (✅ **1.00x slower**) | `78.63 ns` (🚀 **61.97x faster**) | `153.21 ns` (🚀 **31.81x faster**) | `25.90 ns` (🚀 **188.13x faster**) | `13.28 ns` (🚀 **366.95x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `3.35 us` (✅ **1.00x**)        | `3.35 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `3.39 us` (✅ **1.00x**)        | `3.39 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                          | `N/A`                           | `2.22 us` (✅ **1.00x**)        | `2.22 us` (✅ **1.00x slower**) | `53.73 ns` (🚀 **41.27x faster**) | `120.35 ns` (🚀 **18.42x faster**) | `19.20 ns` (🚀 **115.46x faster**) | `7.16 ns` (🚀 **309.52x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `1.80 ms` (✅ **1.00x**)        | `1.80 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `70.15 ns` (❌ *3.88x slower*)    | `119.14 ns` (❌ *6.58x slower*)    | `22.60 ns` (❌ *1.25x slower*)     | `18.09 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `2.48 us` (❌ *32.66x slower*)    | `7.88 us` (❌ *103.94x slower*)    | `304.03 ns` (❌ *4.01x slower*)    | `75.81 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `1.78 us` (❌ *26.68x slower*)    | `5.53 us` (❌ *82.88x slower*)     | `244.91 ns` (❌ *3.67x slower*)    | `66.67 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `51.66 us` (❌ *3.57x slower*)    | `60.73 us` (❌ *4.19x slower*)     | `47.48 us` (❌ *3.28x slower*)     | `14.49 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `5.10 us` (❌ *43.52x slower*)    | `16.09 us` (❌ *137.40x slower*)   | `417.71 ns` (❌ *3.57x slower*)    | `117.14 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `5.03 us` (❌ *30.72x slower*)    | `16.03 us` (❌ *97.99x slower*)    | `647.54 ns` (❌ *3.96x slower*)    | `163.58 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `8.63 ns` (✅ **1.00x**)        | `17.14 ns` (❌ *1.98x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.39 ns` (✅ **1.00x**)       | `21.65 ns` (❌ *2.08x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.87 ns` (✅ **1.00x**)        | `4.95 ns` (✅ **1.02x slower**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `4.55 ns` (✅ **1.00x**)        | `4.56 ns` (✅ **1.00x slower**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761_optimized

|                                          | `g1projectivebw6_761`          | `g2projectivebw6_761`            | `froptimized`                       | `fqoptimized`                       | `fq3optimized`                    | `fq6optimized`                    |
|:-----------------------------------------|:-------------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `515.78 ns` (✅ **1.00x**)      | `514.59 ns` (✅ **1.00x faster**) | `55.96 ns` (🚀 **9.22x faster**)     | `170.03 ns` (🚀 **3.03x faster**)    | `522.46 ns` (✅ **1.01x slower**)  | `1.06 us` (❌ *2.05x slower*)      |
| **`serialize_uncompressed`**             | `697.48 ns` (✅ **1.00x**)      | `697.41 ns` (✅ **1.00x faster**) | `55.94 ns` (🚀 **12.47x faster**)    | `172.51 ns` (🚀 **4.04x faster**)    | `521.66 ns` (✅ **1.34x faster**)  | `1.07 us` (❌ *1.53x slower*)      |
| **`deserialize_compressed`**             | `1.59 ms` (✅ **1.00x**)        | `1.59 ms` (✅ **1.00x faster**)   | `93.55 ns` (🚀 **16990.89x faster**) | `346.77 ns` (🚀 **4583.69x faster**) | `1.05 us` (🚀 **1513.51x faster**) | `2.13 us` (🚀 **747.73x faster**)  |
| **`deserialize_compressed_unchecked`**   | `292.23 us` (✅ **1.00x**)      | `292.18 us` (✅ **1.00x faster**) | `93.54 ns` (🚀 **3124.18x faster**)  | `346.11 ns` (🚀 **844.33x faster**)  | `1.05 us` (🚀 **278.31x faster**)  | `2.13 us` (🚀 **137.45x faster**)  |
| **`deserialize_uncompressed`**           | `1.30 ms` (✅ **1.00x**)        | `1.30 ms` (✅ **1.00x slower**)   | `93.44 ns` (🚀 **13883.11x faster**) | `346.31 ns` (🚀 **3745.97x faster**) | `1.05 us` (🚀 **1235.64x faster**) | `2.13 us` (🚀 **610.41x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `771.79 ns` (✅ **1.00x**)      | `763.42 ns` (✅ **1.01x faster**) | `93.41 ns` (🚀 **8.26x faster**)     | `346.89 ns` (🚀 **2.22x faster**)    | `1.05 us` (❌ *1.36x slower*)      | `2.13 us` (❌ *2.75x slower*)      |

### msm_for_bw6_761_optimized

|        | `g1projectivebw6_761`          | `g2projectivebw6_761`           |
|:-------|:-------------------------------|:------------------------------- |
|        | `12.27 s` (✅ **1.00x**)        | `12.30 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761_optimized

|                          | `froptimized`            | `fqoptimized`                    | `fq3optimized`                    |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `67.44 us` (✅ **1.00x**) | `290.75 us` (❌ *4.31x slower*)   | `6.94 ms` (❌ *102.97x slower*)    |
| **`legendre_for_qr`**    | `32.05 us` (✅ **1.00x**) | `291.70 us` (❌ *9.10x slower*)   | `297.15 us` (❌ *9.27x slower*)    |

### bitwise_operations_for_bw6_761_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `5.02 ns` (✅ **1.00x**)        | `5.12 ns` (✅ **1.02x slower**)    |
| **`from_little-endian_bits`** | `83.55 ns` (✅ **1.00x**)       | `168.83 ns` (❌ *2.02x slower*)    |
| **`from_big-endian_bits`**    | `83.44 ns` (✅ **1.00x**)       | `169.64 ns` (❌ *2.03x slower*)    |
| **`comparison`**              | `5.08 ns` (✅ **1.00x**)        | `5.07 ns` (✅ **1.00x faster**)    |
| **`equality`**                | `5.77 ns` (✅ **1.00x**)        | `5.76 ns` (✅ **1.00x faster**)    |
| **`is_zero`**                 | `5.21 ns` (✅ **1.00x**)        | `5.34 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761_optimized

|                   | `froptimized`            | `fqoptimized`                     |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `74.87 ns` (✅ **1.00x**) | `315.10 ns` (❌ *4.21x slower*)    |
| **`into_bigint`** | `46.92 ns` (✅ **1.00x**) | `157.57 ns` (❌ *3.36x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

