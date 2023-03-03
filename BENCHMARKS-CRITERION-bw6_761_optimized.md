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
| **`addition`**                        | `N/A`                          | `N/A`                           | `4.82 us` (✅ **1.00x**)        | `4.82 us` (✅ **1.00x faster**) | `78.43 ns` (🚀 **61.46x faster**) | `162.58 ns` (🚀 **29.65x faster**) | `27.83 ns` (🚀 **173.21x faster**) | `12.60 ns` (🚀 **382.65x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `4.87 us` (✅ **1.00x**)        | `4.87 us` (✅ **1.00x faster**) | `77.87 ns` (🚀 **62.55x faster**) | `153.01 ns` (🚀 **31.83x faster**) | `25.91 ns` (🚀 **187.99x faster**) | `13.27 ns` (🚀 **367.05x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `3.35 us` (✅ **1.00x**)        | `3.35 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `3.41 us` (✅ **1.00x**)        | `3.41 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                          | `N/A`                           | `2.22 us` (✅ **1.00x**)        | `2.22 us` (✅ **1.00x slower**) | `56.34 ns` (🚀 **39.37x faster**) | `117.78 ns` (🚀 **18.83x faster**) | `19.20 ns` (🚀 **115.53x faster**) | `7.14 ns` (🚀 **310.44x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `1.81 ms` (✅ **1.00x**)        | `1.81 ms` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `71.04 ns` (❌ *3.91x slower*)    | `118.26 ns` (❌ *6.51x slower*)    | `22.32 ns` (❌ *1.23x slower*)     | `18.16 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `2.49 us` (❌ *32.94x slower*)    | `7.95 us` (❌ *105.28x slower*)    | `310.73 ns` (❌ *4.11x slower*)    | `75.52 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `1.79 us` (❌ *26.82x slower*)    | `5.53 us` (❌ *83.07x slower*)     | `243.48 ns` (❌ *3.66x slower*)    | `66.57 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `51.23 us` (❌ *3.57x slower*)    | `60.35 us` (❌ *4.20x slower*)     | `47.07 us` (❌ *3.28x slower*)     | `14.37 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `5.12 us` (❌ *43.67x slower*)    | `16.15 us` (❌ *137.84x slower*)   | `418.92 ns` (❌ *3.58x slower*)    | `117.16 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `5.08 us` (❌ *31.19x slower*)    | `16.06 us` (❌ *98.70x slower*)    | `651.55 ns` (❌ *4.00x slower*)    | `162.70 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `8.65 ns` (✅ **1.00x**)        | `17.15 ns` (❌ *1.98x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.43 ns` (✅ **1.00x**)       | `22.14 ns` (❌ *2.12x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.88 ns` (✅ **1.00x**)        | `4.95 ns` (✅ **1.02x slower**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `4.54 ns` (✅ **1.00x**)        | `4.53 ns` (✅ **1.00x faster**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761_optimized

|                                          | `g1projectivebw6_761`          | `g2projectivebw6_761`            | `froptimized`                       | `fqoptimized`                       | `fq3optimized`                    | `fq6optimized`                    |
|:-----------------------------------------|:-------------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `519.66 ns` (✅ **1.00x**)      | `518.53 ns` (✅ **1.00x faster**) | `56.67 ns` (🚀 **9.17x faster**)     | `171.49 ns` (🚀 **3.03x faster**)    | `523.58 ns` (✅ **1.01x slower**)  | `1.11 us` (❌ *2.13x slower*)      |
| **`serialize_uncompressed`**             | `711.92 ns` (✅ **1.00x**)      | `706.57 ns` (✅ **1.01x faster**) | `56.23 ns` (🚀 **12.66x faster**)    | `170.09 ns` (🚀 **4.19x faster**)    | `523.53 ns` (✅ **1.36x faster**)  | `1.11 us` (❌ *1.56x slower*)      |
| **`deserialize_compressed`**             | `1.59 ms` (✅ **1.00x**)        | `1.59 ms` (✅ **1.00x faster**)   | `94.11 ns` (🚀 **16920.22x faster**) | `342.57 ns` (🚀 **4648.20x faster**) | `1.07 us` (🚀 **1484.28x faster**) | `2.15 us` (🚀 **742.03x faster**)  |
| **`deserialize_compressed_unchecked`**   | `293.21 us` (✅ **1.00x**)      | `291.29 us` (✅ **1.01x faster**) | `94.30 ns` (🚀 **3109.30x faster**)  | `342.36 ns` (🚀 **856.42x faster**)  | `1.07 us` (🚀 **273.32x faster**)  | `2.15 us` (🚀 **136.57x faster**)  |
| **`deserialize_uncompressed`**           | `1.30 ms` (✅ **1.00x**)        | `1.30 ms` (✅ **1.00x faster**)   | `93.93 ns` (🚀 **13850.25x faster**) | `342.55 ns` (🚀 **3797.68x faster**) | `1.07 us` (🚀 **1212.97x faster**) | `2.15 us` (🚀 **606.44x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `771.25 ns` (✅ **1.00x**)      | `780.81 ns` (✅ **1.01x slower**) | `93.90 ns` (🚀 **8.21x faster**)     | `343.04 ns` (🚀 **2.25x faster**)    | `1.07 us` (❌ *1.39x slower*)      | `2.15 us` (❌ *2.78x slower*)      |

### msm_for_bw6_761_optimized

|        | `g1projectivebw6_761`          | `g2projectivebw6_761`           |
|:-------|:-------------------------------|:------------------------------- |
|        | `12.66 s` (✅ **1.00x**)        | `12.72 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761_optimized

|                          | `froptimized`            | `fqoptimized`                    | `fq3optimized`                    |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `68.02 us` (✅ **1.00x**) | `289.64 us` (❌ *4.26x slower*)   | `6.95 ms` (❌ *102.18x slower*)    |
| **`legendre_for_qr`**    | `31.85 us` (✅ **1.00x**) | `292.44 us` (❌ *9.18x slower*)   | `297.92 us` (❌ *9.35x slower*)    |

### bitwise_operations_for_bw6_761_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `5.03 ns` (✅ **1.00x**)        | `5.13 ns` (✅ **1.02x slower**)    |
| **`from_little-endian_bits`** | `89.83 ns` (✅ **1.00x**)       | `171.48 ns` (❌ *1.91x slower*)    |
| **`from_big-endian_bits`**    | `89.77 ns` (✅ **1.00x**)       | `176.11 ns` (❌ *1.96x slower*)    |
| **`comparison`**              | `5.14 ns` (✅ **1.00x**)        | `5.10 ns` (✅ **1.01x faster**)    |
| **`equality`**                | `5.67 ns` (✅ **1.00x**)        | `5.66 ns` (✅ **1.00x faster**)    |
| **`is_zero`**                 | `5.21 ns` (✅ **1.00x**)        | `5.35 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761_optimized

|                   | `froptimized`            | `fqoptimized`                     |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `75.18 ns` (✅ **1.00x**) | `313.78 ns` (❌ *4.17x slower*)    |
| **`into_bigint`** | `47.23 ns` (✅ **1.00x**) | `157.61 ns` (❌ *3.34x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

