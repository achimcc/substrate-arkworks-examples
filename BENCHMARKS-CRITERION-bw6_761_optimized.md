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
| **`addition`**                        | `N/A`                          | `N/A`                           | `4.82 us` (✅ **1.00x**)        | `4.82 us` (✅ **1.00x slower**) | `78.41 ns` (🚀 **61.43x faster**) | `157.74 ns` (🚀 **30.54x faster**) | `27.72 ns` (🚀 **173.76x faster**) | `12.64 ns` (🚀 **381.14x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `4.88 us` (✅ **1.00x**)        | `4.88 us` (✅ **1.00x slower**) | `78.73 ns` (🚀 **62.00x faster**) | `152.79 ns` (🚀 **31.95x faster**) | `26.01 ns` (🚀 **187.69x faster**) | `13.36 ns` (🚀 **365.50x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `3.37 us` (✅ **1.00x**)        | `3.36 us` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `3.40 us` (✅ **1.00x**)        | `3.41 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                          | `N/A`                           | `2.22 us` (✅ **1.00x**)        | `2.22 us` (✅ **1.00x faster**) | `54.38 ns` (🚀 **40.90x faster**) | `117.33 ns` (🚀 **18.96x faster**) | `19.23 ns` (🚀 **115.68x faster**) | `7.18 ns` (🚀 **309.96x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `1.80 ms` (✅ **1.00x**)        | `1.80 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `70.60 ns` (❌ *3.89x slower*)    | `119.86 ns` (❌ *6.60x slower*)    | `22.22 ns` (❌ *1.22x slower*)     | `18.17 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `2.47 us` (❌ *32.53x slower*)    | `7.89 us` (❌ *104.05x slower*)    | `313.33 ns` (❌ *4.13x slower*)    | `75.85 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `1.78 us` (❌ *26.88x slower*)    | `5.52 us` (❌ *83.23x slower*)     | `244.80 ns` (❌ *3.69x slower*)    | `66.35 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `51.80 us` (❌ *3.56x slower*)    | `60.89 us` (❌ *4.18x slower*)     | `47.62 us` (❌ *3.27x slower*)     | `14.56 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `5.12 us` (❌ *43.69x slower*)    | `16.14 us` (❌ *137.73x slower*)   | `418.55 ns` (❌ *3.57x slower*)    | `117.17 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `5.03 us` (❌ *30.77x slower*)    | `16.03 us` (❌ *98.07x slower*)    | `648.36 ns` (❌ *3.97x slower*)    | `163.49 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `8.64 ns` (✅ **1.00x**)        | `17.20 ns` (❌ *1.99x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.34 ns` (✅ **1.00x**)       | `21.78 ns` (❌ *2.11x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.88 ns` (✅ **1.00x**)        | `4.95 ns` (✅ **1.01x slower**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `4.53 ns` (✅ **1.00x**)        | `4.55 ns` (✅ **1.00x slower**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761_optimized

|                                          | `g1projectivebw6_761`          | `g2projectivebw6_761`            | `froptimized`                       | `fqoptimized`                       | `fq3optimized`                    | `fq6optimized`                    |
|:-----------------------------------------|:-------------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `513.55 ns` (✅ **1.00x**)      | `514.52 ns` (✅ **1.00x slower**) | `58.17 ns` (🚀 **8.83x faster**)     | `171.16 ns` (🚀 **3.00x faster**)    | `524.32 ns` (✅ **1.02x slower**)  | `1.08 us` (❌ *2.11x slower*)      |
| **`serialize_uncompressed`**             | `695.08 ns` (✅ **1.00x**)      | `695.10 ns` (✅ **1.00x slower**) | `56.17 ns` (🚀 **12.37x faster**)    | `169.92 ns` (🚀 **4.09x faster**)    | `515.90 ns` (✅ **1.35x faster**)  | `1.09 us` (❌ *1.56x slower*)      |
| **`deserialize_compressed`**             | `1.59 ms` (✅ **1.00x**)        | `1.59 ms` (✅ **1.00x slower**)   | `92.48 ns` (🚀 **17171.71x faster**) | `341.93 ns` (🚀 **4644.30x faster**) | `1.04 us` (🚀 **1525.00x faster**) | `2.10 us` (🚀 **756.78x faster**)  |
| **`deserialize_compressed_unchecked`**   | `291.84 us` (✅ **1.00x**)      | `291.79 us` (✅ **1.00x faster**) | `92.33 ns` (🚀 **3160.77x faster**)  | `341.86 ns` (🚀 **853.68x faster**)  | `1.04 us` (🚀 **280.29x faster**)  | `2.10 us` (🚀 **139.10x faster**)  |
| **`deserialize_uncompressed`**           | `1.30 ms` (✅ **1.00x**)        | `1.30 ms` (✅ **1.00x faster**)   | `92.38 ns` (🚀 **14033.48x faster**) | `341.84 ns` (🚀 **3792.30x faster**) | `1.04 us` (🚀 **1245.18x faster**) | `2.10 us` (🚀 **617.82x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `768.31 ns` (✅ **1.00x**)      | `773.24 ns` (✅ **1.01x slower**) | `92.52 ns` (🚀 **8.30x faster**)     | `341.86 ns` (🚀 **2.25x faster**)    | `1.04 us` (❌ *1.36x slower*)      | `2.10 us` (❌ *2.73x slower*)      |

### msm_for_bw6_761_optimized

|        | `g1projectivebw6_761`          | `g2projectivebw6_761`           |
|:-------|:-------------------------------|:------------------------------- |
|        | `12.39 s` (✅ **1.00x**)        | `12.37 s` (✅ **1.00x faster**)  |

### squareroot_for_bw6_761_optimized

|                          | `froptimized`            | `fqoptimized`                    | `fq3optimized`                    |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `67.24 us` (✅ **1.00x**) | `290.30 us` (❌ *4.32x slower*)   | `7.03 ms` (❌ *104.61x slower*)    |
| **`legendre_for_qr`**    | `32.29 us` (✅ **1.00x**) | `292.54 us` (❌ *9.06x slower*)   | `296.93 us` (❌ *9.20x slower*)    |

### bitwise_operations_for_bw6_761_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `5.02 ns` (✅ **1.00x**)        | `5.12 ns` (✅ **1.02x slower**)    |
| **`from_little-endian_bits`** | `83.33 ns` (✅ **1.00x**)       | `168.97 ns` (❌ *2.03x slower*)    |
| **`from_big-endian_bits`**    | `83.32 ns` (✅ **1.00x**)       | `169.61 ns` (❌ *2.04x slower*)    |
| **`comparison`**              | `5.08 ns` (✅ **1.00x**)        | `5.10 ns` (✅ **1.00x slower**)    |
| **`equality`**                | `5.67 ns` (✅ **1.00x**)        | `5.74 ns` (✅ **1.01x slower**)    |
| **`is_zero`**                 | `5.21 ns` (✅ **1.00x**)        | `5.37 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761_optimized

|                   | `froptimized`            | `fqoptimized`                     |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `75.27 ns` (✅ **1.00x**) | `309.75 ns` (❌ *4.12x slower*)    |
| **`into_bigint`** | `46.90 ns` (✅ **1.00x**) | `158.73 ns` (❌ *3.38x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

