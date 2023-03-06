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
|        | `1.69 ms` (✅ **1.00x**)                 | `1.70 ms` (✅ **1.01x slower**)           |

### arithmetic_for_bw6_761_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebw6_761`          | `g2projectivebw6_761`          | `fq3optimized`                   | `fq6optimized`                    | `fqoptimized`                     | `froptimized`                      |
|:--------------------------------------|:-------------------------------|:--------------------------------|:-------------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `3.95 us` (✅ **1.00x**)        | `4.10 us` (✅ **1.04x slower**) | `89.06 ns` (🚀 **44.41x faster**) | `196.84 ns` (🚀 **20.09x faster**) | `29.28 ns` (🚀 **135.06x faster**) | `19.30 ns` (🚀 **204.86x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `4.12 us` (✅ **1.00x**)        | `4.15 us` (✅ **1.01x slower**) | `85.19 ns` (🚀 **48.41x faster**) | `195.04 ns` (🚀 **21.14x faster**) | `29.63 ns` (🚀 **139.16x faster**) | `15.13 ns` (🚀 **272.63x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `3.02 us` (✅ **1.00x**)        | `2.87 us` (✅ **1.05x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `2.87 us` (✅ **1.00x**)        | `2.97 us` (✅ **1.03x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                          | `N/A`                           | `1.88 us` (✅ **1.00x**)        | `1.95 us` (✅ **1.04x slower**) | `66.54 ns` (🚀 **28.19x faster**) | `166.64 ns` (🚀 **11.26x faster**) | `20.65 ns` (🚀 **90.82x faster**)  | `10.72 ns` (🚀 **175.03x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `1.50 ms` (✅ **1.00x**)        | `1.59 ms` (✅ **1.06x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `67.56 ns` (❌ *3.92x slower*)    | `146.04 ns` (❌ *8.48x slower*)    | `23.53 ns` (❌ *1.37x slower*)     | `17.22 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `2.24 us` (❌ *33.01x slower*)    | `8.15 us` (❌ *120.06x slower*)    | `275.48 ns` (❌ *4.06x slower*)    | `67.91 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `1.51 us` (❌ *25.25x slower*)    | `5.74 us` (❌ *96.09x slower*)     | `209.51 ns` (❌ *3.51x slower*)    | `59.77 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `49.66 us` (❌ *3.73x slower*)    | `70.11 us` (❌ *5.26x slower*)     | `46.52 us` (❌ *3.49x slower*)     | `13.32 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `4.34 us` (❌ *42.54x slower*)    | `16.36 us` (❌ *160.43x slower*)   | `404.44 ns` (❌ *3.97x slower*)    | `101.95 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `4.72 us` (❌ *30.88x slower*)    | `16.32 us` (❌ *106.86x slower*)   | `556.78 ns` (❌ *3.64x slower*)    | `152.77 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.74 ns` (✅ **1.00x**)        | `15.56 ns` (❌ *2.01x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.42 ns` (✅ **1.00x**)       | `22.19 ns` (❌ *2.13x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `3.92 ns` (✅ **1.00x**)        | `4.01 ns` (✅ **1.02x slower**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `3.72 ns` (✅ **1.00x**)        | `3.70 ns` (✅ **1.00x faster**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761_optimized

|                                          | `g1projectivebw6_761`          | `g2projectivebw6_761`            | `froptimized`                       | `fqoptimized`                       | `fq3optimized`                      | `fq6optimized`                    |
|:-----------------------------------------|:-------------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `455.83 ns` (✅ **1.00x**)      | `463.61 ns` (✅ **1.02x slower**) | `50.16 ns` (🚀 **9.09x faster**)     | `155.20 ns` (🚀 **2.94x faster**)    | `462.95 ns` (✅ **1.02x slower**)    | `1.16 us` (❌ *2.55x slower*)      |
| **`serialize_uncompressed`**             | `620.01 ns` (✅ **1.00x**)      | `608.64 ns` (✅ **1.02x faster**) | `48.38 ns` (🚀 **12.82x faster**)    | `155.16 ns` (🚀 **4.00x faster**)    | `459.41 ns` (✅ **1.35x faster**)    | `1.18 us` (❌ *1.90x slower*)      |
| **`deserialize_compressed`**             | `1.36 ms` (✅ **1.00x**)        | `1.39 ms` (✅ **1.02x slower**)   | `93.06 ns` (🚀 **14630.52x faster**) | `316.96 ns` (🚀 **4295.65x faster**) | `944.58 ns` (🚀 **1441.45x faster**) | `2.34 us` (🚀 **582.24x faster**)  |
| **`deserialize_compressed_unchecked`**   | `247.29 us` (✅ **1.00x**)      | `248.44 us` (✅ **1.00x slower**) | `93.96 ns` (🚀 **2631.92x faster**)  | `314.56 ns` (🚀 **786.16x faster**)  | `929.83 ns` (🚀 **265.96x faster**)  | `2.33 us` (🚀 **106.16x faster**)  |
| **`deserialize_uncompressed`**           | `1.09 ms` (✅ **1.00x**)        | `1.10 ms` (✅ **1.01x slower**)   | `98.01 ns` (🚀 **11128.13x faster**) | `309.65 ns` (🚀 **3522.23x faster**) | `927.66 ns` (🚀 **1175.73x faster**) | `2.33 us` (🚀 **468.06x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `644.20 ns` (✅ **1.00x**)      | `650.19 ns` (✅ **1.01x slower**) | `94.05 ns` (🚀 **6.85x faster**)     | `313.13 ns` (🚀 **2.06x faster**)    | `961.82 ns` (❌ *1.49x slower*)      | `2.32 us` (❌ *3.61x slower*)      |

### msm_for_bw6_761_optimized

|        | `g1projectivebw6_761`          | `g2projectivebw6_761`           |
|:-------|:-------------------------------|:------------------------------- |
|        | `10.96 s` (✅ **1.00x**)        | `10.93 s` (✅ **1.00x faster**)  |

### squareroot_for_bw6_761_optimized

|                          | `froptimized`            | `fqoptimized`                    | `fq3optimized`                    |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `62.50 us` (✅ **1.00x**) | `247.95 us` (❌ *3.97x slower*)   | `6.48 ms` (❌ *103.69x slower*)    |
| **`legendre_for_qr`**    | `28.67 us` (✅ **1.00x**) | `247.48 us` (❌ *8.63x slower*)   | `261.02 us` (❌ *9.10x slower*)    |

### bitwise_operations_for_bw6_761_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.28 ns` (✅ **1.00x**)        | `4.10 ns` (✅ **1.04x faster**)    |
| **`from_little-endian_bits`** | `108.07 ns` (✅ **1.00x**)      | `219.73 ns` (❌ *2.03x slower*)    |
| **`from_big-endian_bits`**    | `108.80 ns` (✅ **1.00x**)      | `209.45 ns` (❌ *1.92x slower*)    |
| **`comparison`**              | `4.05 ns` (✅ **1.00x**)        | `4.21 ns` (✅ **1.04x slower**)    |
| **`equality`**                | `4.93 ns` (✅ **1.00x**)        | `5.01 ns` (✅ **1.02x slower**)    |
| **`is_zero`**                 | `3.93 ns` (✅ **1.00x**)        | `4.34 ns` (✅ **1.11x slower**)    |

### conversions_for_bw6_761_optimized

|                   | `froptimized`            | `fqoptimized`                     |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `78.80 ns` (✅ **1.00x**) | `285.83 ns` (❌ *3.63x slower*)    |
| **`into_bigint`** | `41.16 ns` (✅ **1.00x**) | `139.56 ns` (❌ *3.39x slower*)    |

### pairing_for_bw6_761optimized

|        | `g1_preparation_for_bw6_761optimized`          | `g2_preparation_for_bw6_761optimized`          | `miller_loop_for_bw6_761optimized`          | `final_exponentiation_for_bw6_761optimized`          | `full_pairing_for_bw6_761optimized`           |
|:-------|:-----------------------------------------------|:-----------------------------------------------|:--------------------------------------------|:-----------------------------------------------------|:--------------------------------------------- |
|        | `19.79 ns` (✅ **1.00x**)                       | `12.78 ns` (✅ **1.55x faster**)                | `4.64 ms` (❌ *234590.26x slower*)           | `4.37 ms` (❌ *220764.48x slower*)                    | `9.03 ms` (❌ *456242.60x slower*)             |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

