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
| **`addition`**                        | `N/A`                          | `N/A`                           | `4.82 us` (✅ **1.00x**)        | `4.82 us` (✅ **1.00x slower**) | `79.97 ns` (🚀 **60.31x faster**) | `161.37 ns` (🚀 **29.88x faster**) | `27.76 ns` (🚀 **173.72x faster**) | `12.62 ns` (🚀 **382.25x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `4.87 us` (✅ **1.00x**)        | `4.87 us` (✅ **1.00x slower**) | `79.17 ns` (🚀 **61.47x faster**) | `152.40 ns` (🚀 **31.93x faster**) | `25.83 ns` (🚀 **188.39x faster**) | `13.27 ns` (🚀 **366.73x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `3.35 us` (✅ **1.00x**)        | `3.35 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `3.41 us` (✅ **1.00x**)        | `3.41 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                          | `N/A`                           | `2.22 us` (✅ **1.00x**)        | `2.22 us` (✅ **1.00x slower**) | `56.29 ns` (🚀 **39.43x faster**) | `118.50 ns` (🚀 **18.73x faster**) | `19.03 ns` (🚀 **116.62x faster**) | `7.16 ns` (🚀 **309.90x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `1.81 ms` (✅ **1.00x**)        | `1.81 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `70.88 ns` (❌ *3.85x slower*)    | `120.87 ns` (❌ *6.56x slower*)    | `22.65 ns` (❌ *1.23x slower*)     | `18.43 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `2.49 us` (❌ *32.99x slower*)    | `7.93 us` (❌ *105.16x slower*)    | `310.62 ns` (❌ *4.12x slower*)    | `75.45 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `1.78 us` (❌ *26.77x slower*)    | `5.53 us` (❌ *83.05x slower*)     | `243.42 ns` (❌ *3.66x slower*)    | `66.58 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `51.25 us` (❌ *3.56x slower*)    | `60.34 us` (❌ *4.19x slower*)     | `47.03 us` (❌ *3.26x slower*)     | `14.41 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `5.12 us` (❌ *43.68x slower*)    | `16.13 us` (❌ *137.72x slower*)   | `418.28 ns` (❌ *3.57x slower*)    | `117.13 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `5.08 us` (❌ *31.16x slower*)    | `16.05 us` (❌ *98.55x slower*)    | `652.42 ns` (❌ *4.01x slower*)    | `162.88 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `8.64 ns` (✅ **1.00x**)        | `17.14 ns` (❌ *1.98x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.43 ns` (✅ **1.00x**)       | `21.84 ns` (❌ *2.09x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.87 ns` (✅ **1.00x**)        | `4.95 ns` (✅ **1.02x slower**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `4.55 ns` (✅ **1.00x**)        | `4.55 ns` (✅ **1.00x faster**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761_optimized

|                                          | `g1projectivebw6_761`          | `g2projectivebw6_761`            | `froptimized`                       | `fqoptimized`                       | `fq3optimized`                    | `fq6optimized`                    |
|:-----------------------------------------|:-------------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `518.89 ns` (✅ **1.00x**)      | `518.76 ns` (✅ **1.00x faster**) | `56.44 ns` (🚀 **9.19x faster**)     | `170.84 ns` (🚀 **3.04x faster**)    | `525.07 ns` (✅ **1.01x slower**)  | `1.11 us` (❌ *2.14x slower*)      |
| **`serialize_uncompressed`**             | `705.88 ns` (✅ **1.00x**)      | `705.90 ns` (✅ **1.00x slower**) | `56.33 ns` (🚀 **12.53x faster**)    | `170.14 ns` (🚀 **4.15x faster**)    | `525.15 ns` (✅ **1.34x faster**)  | `1.10 us` (❌ *1.56x slower*)      |
| **`deserialize_compressed`**             | `1.59 ms` (✅ **1.00x**)        | `1.59 ms` (✅ **1.00x faster**)   | `94.34 ns` (🚀 **16864.82x faster**) | `342.53 ns` (🚀 **4645.02x faster**) | `1.07 us` (🚀 **1484.29x faster**) | `2.14 us` (🚀 **742.32x faster**)  |
| **`deserialize_compressed_unchecked`**   | `291.28 us` (✅ **1.00x**)      | `291.25 us` (✅ **1.00x faster**) | `94.18 ns` (🚀 **3092.95x faster**)  | `342.45 ns` (🚀 **850.60x faster**)  | `1.07 us` (🚀 **271.55x faster**)  | `2.14 us` (🚀 **135.90x faster**)  |
| **`deserialize_uncompressed`**           | `1.30 ms` (✅ **1.00x**)        | `1.30 ms` (✅ **1.00x faster**)   | `94.16 ns` (🚀 **13816.01x faster**) | `342.51 ns` (🚀 **3798.37x faster**) | `1.07 us` (🚀 **1212.57x faster**) | `2.14 us` (🚀 **606.89x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `769.13 ns` (✅ **1.00x**)      | `778.92 ns` (✅ **1.01x slower**) | `94.31 ns` (🚀 **8.15x faster**)     | `342.60 ns` (🚀 **2.24x faster**)    | `1.07 us` (❌ *1.39x slower*)      | `2.14 us` (❌ *2.79x slower*)      |

### msm_for_bw6_761_optimized

|        | `g1projectivebw6_761`          | `g2projectivebw6_761`           |
|:-------|:-------------------------------|:------------------------------- |
|        | `12.53 s` (✅ **1.00x**)        | `12.55 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761_optimized

|                          | `froptimized`            | `fqoptimized`                    | `fq3optimized`                    |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `68.04 us` (✅ **1.00x**) | `289.59 us` (❌ *4.26x slower*)   | `6.95 ms` (❌ *102.11x slower*)    |
| **`legendre_for_qr`**    | `31.91 us` (✅ **1.00x**) | `292.25 us` (❌ *9.16x slower*)   | `297.82 us` (❌ *9.33x slower*)    |

### bitwise_operations_for_bw6_761_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `5.02 ns` (✅ **1.00x**)        | `5.12 ns` (✅ **1.02x slower**)    |
| **`from_little-endian_bits`** | `89.61 ns` (✅ **1.00x**)       | `166.11 ns` (❌ *1.85x slower*)    |
| **`from_big-endian_bits`**    | `88.65 ns` (✅ **1.00x**)       | `165.25 ns` (❌ *1.86x slower*)    |
| **`comparison`**              | `5.14 ns` (✅ **1.00x**)        | `5.09 ns` (✅ **1.01x faster**)    |
| **`equality`**                | `5.67 ns` (✅ **1.00x**)        | `5.74 ns` (✅ **1.01x slower**)    |
| **`is_zero`**                 | `5.21 ns` (✅ **1.00x**)        | `5.35 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761_optimized

|                   | `froptimized`            | `fqoptimized`                     |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `75.56 ns` (✅ **1.00x**) | `313.71 ns` (❌ *4.15x slower*)    |
| **`into_bigint`** | `47.22 ns` (✅ **1.00x**) | `157.56 ns` (❌ *3.34x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

