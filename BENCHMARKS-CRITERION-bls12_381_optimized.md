# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [sample_bls12_381_optimized](#sample_bls12_381_optimized)
    - [arithmetic_for_bls12_381_optimized](#arithmetic_for_bls12_381_optimized)
    - [serialization_for_bls12_381_optimized](#serialization_for_bls12_381_optimized)
    - [msm_for_bls12_381_optimized](#msm_for_bls12_381_optimized)
    - [squareroot_for_bls12_381_optimized](#squareroot_for_bls12_381_optimized)
    - [bitwise_operations_for_bls12_381_optimized](#bitwise_operations_for_bls12_381_optimized)
    - [conversions_for_bls12_381_optimized](#conversions_for_bls12_381_optimized)

## Benchmark Results

### sample_bls12_381_optimized

|        | `g1projectivebls12_381_elements`          | `g2projectivebls12_381_elements`           |
|:-------|:------------------------------------------|:------------------------------------------ |
|        | `220.25 us` (✅ **1.00x**)                 | `1.82 ms` (❌ *8.24x slower*)               |

### arithmetic_for_bls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_381`          | `g2projectivebls12_381`          | `fq2optimized`                   | `fq12optimized`                   | `fqoptimized`                    | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.22 us` (✅ **1.00x**)          | `3.87 us` (❌ *3.17x slower*)     | `23.31 ns` (🚀 **52.38x faster**) | `180.76 ns` (🚀 **6.75x faster**)  | `12.67 ns` (🚀 **96.38x faster**) | `8.67 ns` (🚀 **140.89x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.27 us` (✅ **1.00x**)          | `3.93 us` (❌ *3.10x slower*)     | `23.32 ns` (🚀 **54.37x faster**) | `159.29 ns` (🚀 **7.96x faster**)  | `12.85 ns` (🚀 **98.64x faster**) | `8.78 ns` (🚀 **144.30x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `885.11 ns` (✅ **1.00x**)        | `2.78 us` (❌ *3.14x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `922.74 ns` (✅ **1.00x**)        | `2.82 us` (❌ *3.06x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `582.76 ns` (✅ **1.00x**)        | `1.78 us` (❌ *3.05x slower*)     | `12.45 ns` (🚀 **46.82x faster**) | `74.46 ns` (🚀 **7.83x faster**)   | `7.26 ns` (🚀 **80.25x faster**)  | `5.89 ns` (🚀 **99.01x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `338.20 us` (✅ **1.00x**)        | `976.79 us` (❌ *2.89x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `22.21 ns` (❌ *3.60x slower*)    | `102.81 ns` (❌ *16.69x slower*)   | `18.14 ns` (❌ *2.94x slower*)    | `6.16 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `242.87 ns` (❌ *5.28x slower*)   | `6.24 us` (❌ *135.72x slower*)    | `76.42 ns` (❌ *1.66x slower*)    | `45.97 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `176.43 ns` (❌ *4.70x slower*)   | `4.37 us` (❌ *116.35x slower*)    | `65.00 ns` (❌ *1.73x slower*)    | `37.57 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `15.41 us` (❌ *2.16x slower*)    | `25.74 us` (❌ *3.60x slower*)     | `15.10 us` (❌ *2.11x slower*)    | `7.15 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `525.73 ns` (❌ *6.19x slower*)   | `12.78 us` (❌ *150.57x slower*)   | `118.20 ns` (❌ *1.39x slower*)   | `84.90 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `515.95 ns` (❌ *5.90x slower*)   | `12.71 us` (❌ *145.26x slower*)   | `163.26 ns` (❌ *1.87x slower*)   | `87.48 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.62 ns` (✅ **1.00x**)        | `8.70 ns` (❌ *1.14x slower*)    | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.66 ns` (✅ **1.00x**)        | `10.54 ns` (❌ *1.22x slower*)   | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.87 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.73 ns` (✅ **1.00x**)        | `4.78 ns` (✅ **1.01x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381_optimized

|                                          | `g1projectivebls12_381`          | `g2projectivebls12_381`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                     | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `151.14 ns` (✅ **1.00x**)        | `202.68 ns` (❌ *1.34x slower*)   | `32.20 ns` (🚀 **4.69x faster**)    | `55.25 ns` (🚀 **2.74x faster**)    | `108.92 ns` (✅ **1.39x faster**)   | `704.11 ns` (❌ *4.66x slower*)    |
| **`serialize_uncompressed`**             | `191.43 ns` (✅ **1.00x**)        | `284.51 ns` (❌ *1.49x slower*)   | `32.59 ns` (🚀 **5.87x faster**)    | `55.19 ns` (🚀 **3.47x faster**)    | `108.96 ns` (✅ **1.76x faster**)   | `698.50 ns` (❌ *3.65x slower*)    |
| **`deserialize_compressed`**             | `165.29 us` (✅ **1.00x**)        | `285.73 us` (❌ *1.73x slower*)   | `52.52 ns` (🚀 **3147.41x faster**) | `93.24 ns` (🚀 **1772.74x faster**) | `213.85 ns` (🚀 **772.94x faster**) | `1.32 us` (🚀 **125.19x faster**)  |
| **`deserialize_compressed_unchecked`**   | `39.65 us` (✅ **1.00x**)         | `135.26 us` (❌ *3.41x slower*)   | `52.59 ns` (🚀 **753.98x faster**)  | `93.29 ns` (🚀 **425.03x faster**)  | `213.70 ns` (🚀 **185.55x faster**) | `1.32 us` (🚀 **30.01x faster**)   |
| **`deserialize_uncompressed`**           | `125.77 us` (✅ **1.00x**)        | `149.26 us` (❌ *1.19x slower*)   | `52.50 ns` (🚀 **2395.68x faster**) | `93.30 ns` (🚀 **1347.99x faster**) | `212.99 ns` (🚀 **590.50x faster**) | `1.32 us` (🚀 **95.18x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `221.96 ns` (✅ **1.00x**)        | `483.11 ns` (❌ *2.18x slower*)   | `52.44 ns` (🚀 **4.23x faster**)    | `93.30 ns` (🚀 **2.38x faster**)    | `214.21 ns` (✅ **1.04x faster**)   | `1.32 us` (❌ *5.96x slower*)      |

### msm_for_bls12_381_optimized

|        | `g1projectivebls12_381`          | `g2projectivebls12_381`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.36 s` (✅ **1.00x**)           | `6.97 s` (❌ *2.96x slower*)       |

### squareroot_for_bls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `25.42 us` (✅ **1.00x**) | `39.34 us` (❌ *1.55x slower*)   | `134.37 us` (❌ *5.29x slower*)    |
| **`legendre_for_qr`**    | `14.34 us` (✅ **1.00x**) | `39.44 us` (❌ *2.75x slower*)   | `39.45 us` (❌ *2.75x slower*)     |

### bitwise_operations_for_bls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.33 ns` (✅ **1.00x**)       | `89.82 ns` (❌ *1.86x slower*)    |
| **`from_big-endian_bits`**    | `48.40 ns` (✅ **1.00x**)       | `90.05 ns` (❌ *1.86x slower*)    |
| **`comparison`**              | `4.88 ns` (✅ **1.00x**)        | `5.12 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.37 ns` (✅ **1.00x**)        | `5.68 ns` (✅ **1.06x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.06 ns` (✅ **1.00x**) | `76.16 ns` (❌ *1.85x slower*)    |
| **`into_bigint`** | `22.42 ns` (✅ **1.00x**) | `48.36 ns` (❌ *2.16x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

