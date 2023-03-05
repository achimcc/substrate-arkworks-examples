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
    - [pairing_for_bw6_761optimized](#pairing_for_bw6_761optimized)

## Benchmark Results

### sample_bw6_761_optimized

|        | `g1projectivebw6_761_elements`          | `g2projectivebw6_761_elements`           |
|:-------|:----------------------------------------|:---------------------------------------- |
|        | `2.03 ms` (✅ **1.00x**)                 | `2.02 ms` (✅ **1.00x faster**)           |

### arithmetic_for_bw6_761_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebw6_761`          | `g2projectivebw6_761`          | `fq3optimized`                   | `fq6optimized`                    | `fqoptimized`                     | `froptimized`                      |
|:--------------------------------------|:-------------------------------|:--------------------------------|:-------------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `4.86 us` (✅ **1.00x**)        | `4.86 us` (✅ **1.00x faster**) | `77.96 ns` (🚀 **62.32x faster**) | `161.71 ns` (🚀 **30.04x faster**) | `27.53 ns` (🚀 **176.51x faster**) | `12.63 ns` (🚀 **384.54x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `4.90 us` (✅ **1.00x**)        | `4.91 us` (✅ **1.00x slower**) | `79.27 ns` (🚀 **61.87x faster**) | `152.56 ns` (🚀 **32.15x faster**) | `25.98 ns` (🚀 **188.81x faster**) | `13.36 ns` (🚀 **367.15x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `3.37 us` (✅ **1.00x**)        | `3.37 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `3.42 us` (✅ **1.00x**)        | `3.42 us` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                          | `N/A`                           | `2.22 us` (✅ **1.00x**)        | `2.23 us` (✅ **1.00x slower**) | `54.19 ns` (🚀 **41.03x faster**) | `116.44 ns` (🚀 **19.09x faster**) | `19.28 ns` (🚀 **115.31x faster**) | `7.14 ns` (🚀 **311.23x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `1.81 ms` (✅ **1.00x**)        | `1.80 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `71.49 ns` (❌ *3.91x slower*)    | `123.83 ns` (❌ *6.77x slower*)    | `22.21 ns` (❌ *1.21x slower*)     | `18.30 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `2.49 us` (❌ *33.05x slower*)    | `7.96 us` (❌ *105.51x slower*)    | `306.79 ns` (❌ *4.07x slower*)    | `75.42 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `1.79 us` (❌ *26.88x slower*)    | `5.55 us` (❌ *83.26x slower*)     | `243.76 ns` (❌ *3.66x slower*)    | `66.67 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `51.79 us` (❌ *3.62x slower*)    | `60.81 us` (❌ *4.25x slower*)     | `47.52 us` (❌ *3.32x slower*)     | `14.30 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `5.13 us` (❌ *43.69x slower*)    | `16.23 us` (❌ *138.23x slower*)   | `418.86 ns` (❌ *3.57x slower*)    | `117.38 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `5.07 us` (❌ *30.91x slower*)    | `16.12 us` (❌ *98.21x slower*)    | `654.24 ns` (❌ *3.99x slower*)    | `164.16 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `8.64 ns` (✅ **1.00x**)        | `17.20 ns` (❌ *1.99x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.34 ns` (✅ **1.00x**)       | `21.89 ns` (❌ *2.12x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.88 ns` (✅ **1.00x**)        | `4.95 ns` (✅ **1.02x slower**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `4.76 ns` (✅ **1.00x**)        | `4.74 ns` (✅ **1.00x faster**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761_optimized

|                                          | `g1projectivebw6_761`          | `g2projectivebw6_761`            | `froptimized`                       | `fqoptimized`                       | `fq3optimized`                    | `fq6optimized`                    |
|:-----------------------------------------|:-------------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `516.88 ns` (✅ **1.00x**)      | `515.41 ns` (✅ **1.00x faster**) | `55.99 ns` (🚀 **9.23x faster**)     | `172.55 ns` (🚀 **3.00x faster**)    | `512.94 ns` (✅ **1.01x faster**)  | `1.05 us` (❌ *2.04x slower*)      |
| **`serialize_uncompressed`**             | `696.58 ns` (✅ **1.00x**)      | `695.92 ns` (✅ **1.00x faster**) | `56.17 ns` (🚀 **12.40x faster**)    | `171.00 ns` (🚀 **4.07x faster**)    | `512.83 ns` (✅ **1.36x faster**)  | `1.06 us` (❌ *1.52x slower*)      |
| **`deserialize_compressed`**             | `1.59 ms` (✅ **1.00x**)        | `1.59 ms` (✅ **1.00x slower**)   | `94.15 ns` (🚀 **16925.34x faster**) | `342.52 ns` (🚀 **4652.53x faster**) | `1.05 us` (🚀 **1514.52x faster**) | `2.09 us` (🚀 **761.59x faster**)  |
| **`deserialize_compressed_unchecked`**   | `291.68 us` (✅ **1.00x**)      | `291.63 us` (✅ **1.00x faster**) | `94.07 ns` (🚀 **3100.73x faster**)  | `342.52 ns` (🚀 **851.58x faster**)  | `1.05 us` (🚀 **277.10x faster**)  | `2.09 us` (🚀 **139.54x faster**)  |
| **`deserialize_uncompressed`**           | `1.30 ms` (✅ **1.00x**)        | `1.30 ms` (✅ **1.00x faster**)   | `94.07 ns` (🚀 **13853.40x faster**) | `342.47 ns` (🚀 **3805.34x faster**) | `1.05 us` (🚀 **1238.19x faster**) | `2.09 us` (🚀 **623.26x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `771.46 ns` (✅ **1.00x**)      | `774.32 ns` (✅ **1.00x slower**) | `94.05 ns` (🚀 **8.20x faster**)     | `342.52 ns` (🚀 **2.25x faster**)    | `1.05 us` (❌ *1.36x slower*)      | `2.09 us` (❌ *2.71x slower*)      |

### msm_for_bw6_761_optimized

|        | `g1projectivebw6_761`          | `g2projectivebw6_761`           |
|:-------|:-------------------------------|:------------------------------- |
|        | `12.38 s` (✅ **1.00x**)        | `12.40 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761_optimized

|                          | `froptimized`            | `fqoptimized`                    | `fq3optimized`                    |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `67.69 us` (✅ **1.00x**) | `290.36 us` (❌ *4.29x slower*)   | `7.00 ms` (❌ *103.46x slower*)    |
| **`legendre_for_qr`**    | `31.89 us` (✅ **1.00x**) | `291.90 us` (❌ *9.15x slower*)   | `297.53 us` (❌ *9.33x slower*)    |

### bitwise_operations_for_bw6_761_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `5.02 ns` (✅ **1.00x**)        | `5.12 ns` (✅ **1.02x slower**)    |
| **`from_little-endian_bits`** | `85.90 ns` (✅ **1.00x**)       | `184.53 ns` (❌ *2.15x slower*)    |
| **`from_big-endian_bits`**    | `85.84 ns` (✅ **1.00x**)       | `172.08 ns` (❌ *2.00x slower*)    |
| **`comparison`**              | `5.08 ns` (✅ **1.00x**)        | `5.09 ns` (✅ **1.00x slower**)    |
| **`equality`**                | `5.67 ns` (✅ **1.00x**)        | `5.74 ns` (✅ **1.01x slower**)    |
| **`is_zero`**                 | `5.21 ns` (✅ **1.00x**)        | `5.35 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761_optimized

|                   | `froptimized`            | `fqoptimized`                     |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `75.26 ns` (✅ **1.00x**) | `313.10 ns` (❌ *4.16x slower*)    |
| **`into_bigint`** | `47.04 ns` (✅ **1.00x**) | `155.69 ns` (❌ *3.31x slower*)    |

### pairing_for_bw6_761optimized

|        | `g1_preparation_for_bw6_761optimized`          | `g2_preparation_for_bw6_761optimized`          | `miller_loop_for_bw6_761optimized`          | `final_exponentiation_for_bw6_761optimized`          | `full_pairing_for_bw6_761optimized`           |
|:-------|:-----------------------------------------------|:-----------------------------------------------|:--------------------------------------------|:-----------------------------------------------------|:--------------------------------------------- |
|        | `19.67 ns` (✅ **1.00x**)                       | `11.81 ns` (✅ **1.67x faster**)                | `4.52 ms` (❌ *229674.86x slower*)           | `4.21 ms` (❌ *213985.00x slower*)                    | `8.74 ms` (❌ *444229.42x slower*)             |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

