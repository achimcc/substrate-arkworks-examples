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
|        | `1.75 ms` (✅ **1.00x**)                 | `1.74 ms` (✅ **1.01x faster**)           |

### arithmetic_for_bw6_761_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebw6_761`          | `g2projectivebw6_761`          | `fq3optimized`                   | `fq6optimized`                    | `fqoptimized`                     | `froptimized`                      |
|:--------------------------------------|:-------------------------------|:--------------------------------|:-------------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `4.14 us` (✅ **1.00x**)        | `4.14 us` (✅ **1.00x faster**) | `91.56 ns` (🚀 **45.24x faster**) | `182.36 ns` (🚀 **22.72x faster**) | `30.34 ns` (🚀 **136.51x faster**) | `19.47 ns` (🚀 **212.76x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `4.19 us` (✅ **1.00x**)        | `4.19 us` (✅ **1.00x slower**) | `86.34 ns` (🚀 **48.56x faster**) | `168.31 ns` (🚀 **24.91x faster**) | `30.91 ns` (🚀 **135.61x faster**) | `15.44 ns` (🚀 **271.46x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `2.92 us` (✅ **1.00x**)        | `2.93 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `2.95 us` (✅ **1.00x**)        | `2.96 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                          | `N/A`                           | `1.93 us` (✅ **1.00x**)        | `1.93 us` (✅ **1.00x slower**) | `69.24 ns` (🚀 **27.88x faster**) | `145.41 ns` (🚀 **13.27x faster**) | `24.58 ns` (🚀 **78.51x faster**)  | `11.08 ns` (🚀 **174.23x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `1.55 ms` (✅ **1.00x**)        | `1.54 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `69.53 ns` (❌ *4.15x slower*)    | `123.42 ns` (❌ *7.36x slower*)    | `23.96 ns` (❌ *1.43x slower*)     | `16.77 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `2.17 us` (❌ *31.23x slower*)    | `6.89 us` (❌ *99.40x slower*)     | `270.17 ns` (❌ *3.89x slower*)    | `69.37 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `1.55 us` (❌ *26.28x slower*)    | `4.86 us` (❌ *82.11x slower*)     | `217.01 ns` (❌ *3.67x slower*)    | `59.17 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `50.92 us` (❌ *3.79x slower*)    | `58.70 us` (❌ *4.37x slower*)     | `47.37 us` (❌ *3.53x slower*)     | `13.42 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `4.46 us` (❌ *42.10x slower*)    | `14.06 us` (❌ *132.81x slower*)   | `400.63 ns` (❌ *3.78x slower*)    | `105.87 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `4.40 us` (❌ *27.95x slower*)    | `13.96 us` (❌ *88.64x slower*)    | `567.38 ns` (❌ *3.60x slower*)    | `157.54 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.83 ns` (✅ **1.00x**)        | `15.67 ns` (❌ *2.00x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.63 ns` (✅ **1.00x**)       | `21.10 ns` (❌ *1.99x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.01 ns` (✅ **1.00x**)        | `4.09 ns` (✅ **1.02x slower**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**)        | `3.74 ns` (✅ **1.00x faster**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761_optimized

|                                          | `g1projectivebw6_761`          | `g2projectivebw6_761`            | `froptimized`                       | `fqoptimized`                       | `fq3optimized`                      | `fq6optimized`                    |
|:-----------------------------------------|:-------------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `470.36 ns` (✅ **1.00x**)      | `468.57 ns` (✅ **1.00x faster**) | `50.32 ns` (🚀 **9.35x faster**)     | `156.52 ns` (🚀 **3.01x faster**)    | `464.83 ns` (✅ **1.01x faster**)    | `994.07 ns` (❌ *2.11x slower*)    |
| **`serialize_uncompressed`**             | `629.50 ns` (✅ **1.00x**)      | `628.56 ns` (✅ **1.00x faster**) | `50.00 ns` (🚀 **12.59x faster**)    | `156.30 ns` (🚀 **4.03x faster**)    | `464.76 ns` (✅ **1.35x faster**)    | `994.13 ns` (❌ *1.58x slower*)    |
| **`deserialize_compressed`**             | `1.37 ms` (✅ **1.00x**)        | `1.37 ms` (✅ **1.00x slower**)   | `93.26 ns` (🚀 **14648.26x faster**) | `305.76 ns` (🚀 **4467.81x faster**) | `940.26 ns` (🚀 **1452.87x faster**) | `1.89 us` (🚀 **722.39x faster**)  |
| **`deserialize_compressed_unchecked`**   | `252.04 us` (✅ **1.00x**)      | `252.10 us` (✅ **1.00x slower**) | `93.25 ns` (🚀 **2702.80x faster**)  | `305.77 ns` (🚀 **824.28x faster**)  | `944.72 ns` (🚀 **266.79x faster**)  | `1.89 us` (🚀 **133.36x faster**)  |
| **`deserialize_uncompressed`**           | `1.11 ms` (✅ **1.00x**)        | `1.11 ms` (✅ **1.00x faster**)   | `93.24 ns` (🚀 **11947.07x faster**) | `307.30 ns` (🚀 **3624.91x faster**) | `940.30 ns` (🚀 **1184.66x faster**) | `1.89 us` (🚀 **588.93x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `675.58 ns` (✅ **1.00x**)      | `673.24 ns` (✅ **1.00x faster**) | `93.29 ns` (🚀 **7.24x faster**)     | `307.41 ns` (🚀 **2.20x faster**)    | `940.30 ns` (❌ *1.39x slower*)      | `1.89 us` (❌ *2.80x slower*)      |

### msm_for_bw6_761_optimized

|        | `g1projectivebw6_761`          | `g2projectivebw6_761`           |
|:-------|:-------------------------------|:------------------------------- |
|        | `10.99 s` (✅ **1.00x**)        | `10.99 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761_optimized

|                          | `froptimized`            | `fqoptimized`                    | `fq3optimized`                    |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `64.90 us` (✅ **1.00x**) | `250.72 us` (❌ *3.86x slower*)   | `5.98 ms` (❌ *92.15x slower*)     |
| **`legendre_for_qr`**    | `29.12 us` (✅ **1.00x**) | `251.29 us` (❌ *8.63x slower*)   | `256.53 us` (❌ *8.81x slower*)    |

### bitwise_operations_for_bw6_761_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.19 ns` (✅ **1.00x**)        | `4.24 ns` (✅ **1.01x slower**)    |
| **`from_little-endian_bits`** | `110.61 ns` (✅ **1.00x**)      | `210.41 ns` (❌ *1.90x slower*)    |
| **`from_big-endian_bits`**    | `110.66 ns` (✅ **1.00x**)      | `210.28 ns` (❌ *1.90x slower*)    |
| **`comparison`**              | `4.20 ns` (✅ **1.00x**)        | `4.20 ns` (✅ **1.00x faster**)    |
| **`equality`**                | `4.66 ns` (✅ **1.00x**)        | `5.12 ns` (✅ **1.10x slower**)    |
| **`is_zero`**                 | `4.00 ns` (✅ **1.00x**)        | `4.11 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761_optimized

|                   | `froptimized`            | `fqoptimized`                     |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `78.91 ns` (✅ **1.00x**) | `278.73 ns` (❌ *3.53x slower*)    |
| **`into_bigint`** | `41.47 ns` (✅ **1.00x**) | `144.41 ns` (❌ *3.48x slower*)    |

### pairing_for_bw6_761optimized

|        | `g1_preparation_for_bw6_761optimized`          | `g2_preparation_for_bw6_761optimized`          | `miller_loop_for_bw6_761optimized`          | `final_exponentiation_for_bw6_761optimized`          | `full_pairing_for_bw6_761optimized`           |
|:-------|:-----------------------------------------------|:-----------------------------------------------|:--------------------------------------------|:-----------------------------------------------------|:--------------------------------------------- |
|        | `16.09 ns` (✅ **1.00x**)                       | `10.67 ns` (✅ **1.51x faster**)                | `3.89 ms` (❌ *241724.91x slower*)           | `3.68 ms` (❌ *228394.88x slower*)                    | `7.57 ms` (❌ *470469.86x slower*)             |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

