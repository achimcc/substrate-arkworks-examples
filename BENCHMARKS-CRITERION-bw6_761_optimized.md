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
| **`addition`**                        | `N/A`                          | `N/A`                           | `4.83 us` (✅ **1.00x**)        | `4.82 us` (✅ **1.00x faster**) | `77.64 ns` (🚀 **62.18x faster**) | `157.57 ns` (🚀 **30.64x faster**) | `27.73 ns` (🚀 **174.09x faster**) | `12.63 ns` (🚀 **382.24x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `4.88 us` (✅ **1.00x**)        | `4.89 us` (✅ **1.00x slower**) | `78.16 ns` (🚀 **62.45x faster**) | `150.35 ns` (🚀 **32.47x faster**) | `25.90 ns` (🚀 **188.45x faster**) | `13.37 ns` (🚀 **365.24x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `3.36 us` (✅ **1.00x**)        | `3.36 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `3.40 us` (✅ **1.00x**)        | `3.41 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                          | `N/A`                           | `2.21 us` (✅ **1.00x**)        | `2.22 us` (✅ **1.00x slower**) | `54.38 ns` (🚀 **40.68x faster**) | `116.47 ns` (🚀 **18.99x faster**) | `19.06 ns` (🚀 **116.04x faster**) | `7.13 ns` (🚀 **310.12x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `1.80 ms` (✅ **1.00x**)        | `1.80 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `70.49 ns` (❌ *3.85x slower*)    | `119.24 ns` (❌ *6.51x slower*)    | `22.23 ns` (❌ *1.21x slower*)     | `18.31 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `2.47 us` (❌ *32.51x slower*)    | `7.89 us` (❌ *103.99x slower*)    | `313.29 ns` (❌ *4.13x slower*)    | `75.91 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `1.78 us` (❌ *26.84x slower*)    | `5.52 us` (❌ *83.16x slower*)     | `244.52 ns` (❌ *3.68x slower*)    | `66.39 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `51.78 us` (❌ *3.56x slower*)    | `60.89 us` (❌ *4.18x slower*)     | `47.61 us` (❌ *3.27x slower*)     | `14.56 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `5.12 us` (❌ *43.71x slower*)    | `16.09 us` (❌ *137.38x slower*)   | `418.57 ns` (❌ *3.57x slower*)    | `117.11 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `5.03 us` (❌ *30.86x slower*)    | `16.02 us` (❌ *98.20x slower*)    | `650.19 ns` (❌ *3.99x slower*)    | `163.11 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `8.64 ns` (✅ **1.00x**)        | `17.19 ns` (❌ *1.99x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.33 ns` (✅ **1.00x**)       | `21.79 ns` (❌ *2.11x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.87 ns` (✅ **1.00x**)        | `4.95 ns` (✅ **1.02x slower**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `4.56 ns` (✅ **1.00x**)        | `4.56 ns` (✅ **1.00x slower**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761_optimized

|                                          | `g1projectivebw6_761`          | `g2projectivebw6_761`            | `froptimized`                       | `fqoptimized`                       | `fq3optimized`                    | `fq6optimized`                    |
|:-----------------------------------------|:-------------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `514.39 ns` (✅ **1.00x**)      | `516.98 ns` (✅ **1.01x slower**) | `58.07 ns` (🚀 **8.86x faster**)     | `171.78 ns` (🚀 **2.99x faster**)    | `515.86 ns` (✅ **1.00x slower**)  | `1.08 us` (❌ *2.11x slower*)      |
| **`serialize_uncompressed`**             | `695.43 ns` (✅ **1.00x**)      | `698.53 ns` (✅ **1.00x slower**) | `56.13 ns` (🚀 **12.39x faster**)    | `170.40 ns` (🚀 **4.08x faster**)    | `516.00 ns` (✅ **1.35x faster**)  | `1.08 us` (❌ *1.56x slower*)      |
| **`deserialize_compressed`**             | `1.59 ms` (✅ **1.00x**)        | `1.59 ms` (✅ **1.00x faster**)   | `92.38 ns` (🚀 **17216.84x faster**) | `341.82 ns` (🚀 **4652.92x faster**) | `1.04 us` (🚀 **1527.86x faster**) | `2.10 us` (🚀 **757.69x faster**)  |
| **`deserialize_compressed_unchecked`**   | `291.74 us` (✅ **1.00x**)      | `291.75 us` (✅ **1.00x slower**) | `92.36 ns` (🚀 **3158.85x faster**)  | `341.84 ns` (🚀 **853.46x faster**)  | `1.04 us` (🚀 **280.32x faster**)  | `2.10 us` (🚀 **139.00x faster**)  |
| **`deserialize_uncompressed`**           | `1.30 ms` (✅ **1.00x**)        | `1.30 ms` (✅ **1.00x faster**)   | `92.22 ns` (🚀 **14048.37x faster**) | `341.85 ns` (🚀 **3789.59x faster**) | `1.04 us` (🚀 **1244.75x faster**) | `2.10 us` (🚀 **615.61x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `763.66 ns` (✅ **1.00x**)      | `769.33 ns` (✅ **1.01x slower**) | `92.42 ns` (🚀 **8.26x faster**)     | `341.75 ns` (🚀 **2.23x faster**)    | `1.04 us` (❌ *1.36x slower*)      | `2.10 us` (❌ *2.75x slower*)      |

### msm_for_bw6_761_optimized

|        | `g1projectivebw6_761`          | `g2projectivebw6_761`           |
|:-------|:-------------------------------|:------------------------------- |
|        | `12.34 s` (✅ **1.00x**)        | `12.37 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761_optimized

|                          | `froptimized`            | `fqoptimized`                    | `fq3optimized`                    |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `67.27 us` (✅ **1.00x**) | `290.26 us` (❌ *4.31x slower*)   | `6.96 ms` (❌ *103.52x slower*)    |
| **`legendre_for_qr`**    | `32.29 us` (✅ **1.00x**) | `292.45 us` (❌ *9.06x slower*)   | `296.86 us` (❌ *9.19x slower*)    |

### bitwise_operations_for_bw6_761_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `5.02 ns` (✅ **1.00x**)        | `5.12 ns` (✅ **1.02x slower**)    |
| **`from_little-endian_bits`** | `83.28 ns` (✅ **1.00x**)       | `168.57 ns` (❌ *2.02x slower*)    |
| **`from_big-endian_bits`**    | `83.53 ns` (✅ **1.00x**)       | `167.17 ns` (❌ *2.00x slower*)    |
| **`comparison`**              | `5.08 ns` (✅ **1.00x**)        | `5.11 ns` (✅ **1.01x slower**)    |
| **`equality`**                | `5.67 ns` (✅ **1.00x**)        | `5.65 ns` (✅ **1.00x faster**)    |
| **`is_zero`**                 | `5.21 ns` (✅ **1.00x**)        | `5.34 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761_optimized

|                   | `froptimized`            | `fqoptimized`                     |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `75.25 ns` (✅ **1.00x**) | `309.69 ns` (❌ *4.12x slower*)    |
| **`into_bigint`** | `47.02 ns` (✅ **1.00x**) | `158.72 ns` (❌ *3.38x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

