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
| **`addition`**                        | `N/A`                          | `N/A`                           | `4.82 us` (✅ **1.00x**)        | `4.82 us` (✅ **1.00x slower**) | `77.86 ns` (🚀 **61.85x faster**) | `161.23 ns` (🚀 **29.87x faster**) | `27.74 ns` (🚀 **173.62x faster**) | `12.70 ns` (🚀 **379.35x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `4.88 us` (✅ **1.00x**)        | `4.89 us` (✅ **1.00x slower**) | `78.85 ns` (🚀 **61.95x faster**) | `152.04 ns` (🚀 **32.13x faster**) | `25.90 ns` (🚀 **188.61x faster**) | `13.28 ns` (🚀 **367.76x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `3.36 us` (✅ **1.00x**)        | `3.36 us` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `3.40 us` (✅ **1.00x**)        | `3.40 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                          | `N/A`                           | `2.22 us` (✅ **1.00x**)        | `2.22 us` (✅ **1.00x slower**) | `54.40 ns` (🚀 **40.77x faster**) | `118.19 ns` (🚀 **18.77x faster**) | `19.42 ns` (🚀 **114.24x faster**) | `7.17 ns` (🚀 **309.36x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `1.80 ms` (✅ **1.00x**)        | `1.80 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `71.82 ns` (❌ *3.81x slower*)    | `119.01 ns` (❌ *6.31x slower*)    | `23.11 ns` (❌ *1.22x slower*)     | `18.87 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `2.48 us` (❌ *32.65x slower*)    | `7.89 us` (❌ *104.03x slower*)    | `303.95 ns` (❌ *4.01x slower*)    | `75.82 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `1.78 us` (❌ *26.69x slower*)    | `5.54 us` (❌ *83.13x slower*)     | `244.99 ns` (❌ *3.68x slower*)    | `66.66 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `51.60 us` (❌ *3.57x slower*)    | `60.70 us` (❌ *4.20x slower*)     | `47.44 us` (❌ *3.28x slower*)     | `14.45 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `5.10 us` (❌ *43.50x slower*)    | `16.07 us` (❌ *137.23x slower*)   | `417.78 ns` (❌ *3.57x slower*)    | `117.14 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `5.03 us` (❌ *30.77x slower*)    | `16.01 us` (❌ *97.98x slower*)    | `653.16 ns` (❌ *4.00x slower*)    | `163.43 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `8.63 ns` (✅ **1.00x**)        | `17.12 ns` (❌ *1.98x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.39 ns` (✅ **1.00x**)       | `21.63 ns` (❌ *2.08x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.87 ns` (✅ **1.00x**)        | `4.95 ns` (✅ **1.02x slower**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `4.55 ns` (✅ **1.00x**)        | `4.55 ns` (✅ **1.00x faster**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761_optimized

|                                          | `g1projectivebw6_761`          | `g2projectivebw6_761`            | `froptimized`                       | `fqoptimized`                       | `fq3optimized`                    | `fq6optimized`                    |
|:-----------------------------------------|:-------------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `515.79 ns` (✅ **1.00x**)      | `514.22 ns` (✅ **1.00x faster**) | `55.99 ns` (🚀 **9.21x faster**)     | `169.98 ns` (🚀 **3.03x faster**)    | `528.60 ns` (✅ **1.02x slower**)  | `1.06 us` (❌ *2.06x slower*)      |
| **`serialize_uncompressed`**             | `696.79 ns` (✅ **1.00x**)      | `696.86 ns` (✅ **1.00x slower**) | `55.99 ns` (🚀 **12.45x faster**)    | `172.58 ns` (🚀 **4.04x faster**)    | `528.32 ns` (✅ **1.32x faster**)  | `1.06 us` (❌ *1.53x slower*)      |
| **`deserialize_compressed`**             | `1.59 ms` (✅ **1.00x**)        | `1.59 ms` (✅ **1.00x slower**)   | `93.09 ns` (🚀 **17071.35x faster**) | `347.13 ns` (🚀 **4578.22x faster**) | `1.06 us` (🚀 **1500.66x faster**) | `2.14 us` (🚀 **743.80x faster**)  |
| **`deserialize_compressed_unchecked`**   | `292.30 us` (✅ **1.00x**)      | `292.22 us` (✅ **1.00x faster**) | `93.08 ns` (🚀 **3140.20x faster**)  | `347.14 ns` (🚀 **842.01x faster**)  | `1.06 us` (🚀 **276.01x faster**)  | `2.14 us` (🚀 **136.78x faster**)  |
| **`deserialize_uncompressed`**           | `1.30 ms` (✅ **1.00x**)        | `1.30 ms` (✅ **1.00x slower**)   | `92.98 ns` (🚀 **13954.76x faster**) | `347.10 ns` (🚀 **3738.08x faster**) | `1.06 us` (🚀 **1225.20x faster**) | `2.14 us` (🚀 **606.82x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `773.68 ns` (✅ **1.00x**)      | `766.61 ns` (✅ **1.01x faster**) | `92.93 ns` (🚀 **8.33x faster**)     | `347.16 ns` (🚀 **2.23x faster**)    | `1.06 us` (❌ *1.37x slower*)      | `2.14 us` (❌ *2.76x slower*)      |

### msm_for_bw6_761_optimized

|        | `g1projectivebw6_761`          | `g2projectivebw6_761`           |
|:-------|:-------------------------------|:------------------------------- |
|        | `12.29 s` (✅ **1.00x**)        | `12.30 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761_optimized

|                          | `froptimized`            | `fqoptimized`                    | `fq3optimized`                    |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `67.44 us` (✅ **1.00x**) | `290.78 us` (❌ *4.31x slower*)   | `6.95 ms` (❌ *103.01x slower*)    |
| **`legendre_for_qr`**    | `32.04 us` (✅ **1.00x**) | `291.69 us` (❌ *9.10x slower*)   | `297.20 us` (❌ *9.28x slower*)    |

### bitwise_operations_for_bw6_761_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `5.02 ns` (✅ **1.00x**)        | `5.12 ns` (✅ **1.02x slower**)    |
| **`from_little-endian_bits`** | `83.39 ns` (✅ **1.00x**)       | `168.92 ns` (❌ *2.03x slower*)    |
| **`from_big-endian_bits`**    | `83.47 ns` (✅ **1.00x**)       | `167.44 ns` (❌ *2.01x slower*)    |
| **`comparison`**              | `5.08 ns` (✅ **1.00x**)        | `5.07 ns` (✅ **1.00x faster**)    |
| **`equality`**                | `5.77 ns` (✅ **1.00x**)        | `5.65 ns` (✅ **1.02x faster**)    |
| **`is_zero`**                 | `5.21 ns` (✅ **1.00x**)        | `5.34 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761_optimized

|                   | `froptimized`            | `fqoptimized`                     |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `75.02 ns` (✅ **1.00x**) | `315.10 ns` (❌ *4.20x slower*)    |
| **`into_bigint`** | `46.93 ns` (✅ **1.00x**) | `157.58 ns` (❌ *3.36x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

