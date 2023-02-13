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
|        | `220.22 us` (✅ **1.00x**)                 | `1.81 ms` (❌ *8.23x slower*)               |

### arithmetic_for_bls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_381`          | `g2projectivebls12_381`          | `fq2optimized`                   | `fq12optimized`                   | `fqoptimized`                    | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.22 us` (✅ **1.00x**)          | `3.86 us` (❌ *3.16x slower*)     | `23.30 ns` (🚀 **52.43x faster**) | `181.18 ns` (🚀 **6.74x faster**)  | `12.68 ns` (🚀 **96.37x faster**) | `8.66 ns` (🚀 **141.12x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.27 us` (✅ **1.00x**)          | `3.93 us` (❌ *3.11x slower*)     | `23.32 ns` (🚀 **54.31x faster**) | `157.23 ns` (🚀 **8.06x faster**)  | `12.85 ns` (🚀 **98.59x faster**) | `8.78 ns` (🚀 **144.34x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `884.62 ns` (✅ **1.00x**)        | `2.78 us` (❌ *3.14x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `923.17 ns` (✅ **1.00x**)        | `2.82 us` (❌ *3.06x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `583.54 ns` (✅ **1.00x**)        | `1.78 us` (❌ *3.05x slower*)     | `12.51 ns` (🚀 **46.65x faster**) | `70.96 ns` (🚀 **8.22x faster**)   | `7.24 ns` (🚀 **80.62x faster**)  | `5.86 ns` (🚀 **99.51x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `338.15 us` (✅ **1.00x**)        | `976.10 us` (❌ *2.89x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `22.64 ns` (❌ *3.67x slower*)    | `102.48 ns` (❌ *16.63x slower*)   | `18.37 ns` (❌ *2.98x slower*)    | `6.16 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `241.95 ns` (❌ *5.26x slower*)   | `6.23 us` (❌ *135.45x slower*)    | `76.49 ns` (❌ *1.66x slower*)    | `45.97 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `176.70 ns` (❌ *4.71x slower*)   | `4.37 us` (❌ *116.46x slower*)    | `64.94 ns` (❌ *1.73x slower*)    | `37.55 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `15.37 us` (❌ *2.15x slower*)    | `25.71 us` (❌ *3.59x slower*)     | `15.08 us` (❌ *2.11x slower*)    | `7.15 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `526.28 ns` (❌ *6.20x slower*)   | `12.78 us` (❌ *150.56x slower*)   | `118.29 ns` (❌ *1.39x slower*)   | `84.85 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `516.01 ns` (❌ *5.90x slower*)   | `12.70 us` (❌ *145.22x slower*)   | `162.99 ns` (❌ *1.86x slower*)   | `87.46 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**)        | `8.65 ns` (❌ *1.14x slower*)    | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.64 ns` (✅ **1.00x**)        | `10.49 ns` (❌ *1.21x slower*)   | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.87 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.80 ns` (✅ **1.00x**)        | `4.80 ns` (✅ **1.00x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381_optimized

|                                          | `g1projectivebls12_381`          | `g2projectivebls12_381`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                     | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `150.41 ns` (✅ **1.00x**)        | `202.33 ns` (❌ *1.35x slower*)   | `32.25 ns` (🚀 **4.66x faster**)    | `55.35 ns` (🚀 **2.72x faster**)    | `108.87 ns` (✅ **1.38x faster**)   | `704.53 ns` (❌ *4.68x slower*)    |
| **`serialize_uncompressed`**             | `191.43 ns` (✅ **1.00x**)        | `284.23 ns` (❌ *1.48x slower*)   | `32.05 ns` (🚀 **5.97x faster**)    | `55.22 ns` (🚀 **3.47x faster**)    | `108.85 ns` (✅ **1.76x faster**)   | `698.58 ns` (❌ *3.65x slower*)    |
| **`deserialize_compressed`**             | `165.27 us` (✅ **1.00x**)        | `284.85 us` (❌ *1.72x slower*)   | `52.59 ns` (🚀 **3142.60x faster**) | `94.18 ns` (🚀 **1754.87x faster**) | `213.65 ns` (🚀 **773.55x faster**) | `1.34 us` (🚀 **123.45x faster**)  |
| **`deserialize_compressed_unchecked`**   | `39.60 us` (✅ **1.00x**)         | `135.25 us` (❌ *3.42x slower*)   | `52.62 ns` (🚀 **752.67x faster**)  | `93.99 ns` (🚀 **421.36x faster**)  | `213.78 ns` (🚀 **185.24x faster**) | `1.34 us` (🚀 **29.60x faster**)   |
| **`deserialize_uncompressed`**           | `125.56 us` (✅ **1.00x**)        | `149.24 us` (❌ *1.19x slower*)   | `52.42 ns` (🚀 **2395.39x faster**) | `94.09 ns` (🚀 **1334.55x faster**) | `214.11 ns` (🚀 **586.45x faster**) | `1.34 us` (🚀 **93.87x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `220.10 ns` (✅ **1.00x**)        | `484.49 ns` (❌ *2.20x slower*)   | `52.53 ns` (🚀 **4.19x faster**)    | `94.12 ns` (🚀 **2.34x faster**)    | `213.88 ns` (✅ **1.03x faster**)   | `1.34 us` (❌ *6.08x slower*)      |

### msm_for_bls12_381_optimized

|        | `g1projectivebls12_381`          | `g2projectivebls12_381`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.35 s` (✅ **1.00x**)           | `6.97 s` (❌ *2.97x slower*)       |

### squareroot_for_bls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `25.41 us` (✅ **1.00x**) | `39.32 us` (❌ *1.55x slower*)   | `134.33 us` (❌ *5.29x slower*)    |
| **`legendre_for_qr`**    | `14.31 us` (✅ **1.00x**) | `39.46 us` (❌ *2.76x slower*)   | `39.45 us` (❌ *2.76x slower*)     |

### bitwise_operations_for_bls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.84 ns` (✅ **1.00x**)        | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `49.33 ns` (✅ **1.00x**)       | `88.91 ns` (❌ *1.80x slower*)    |
| **`from_big-endian_bits`**    | `49.30 ns` (✅ **1.00x**)       | `88.81 ns` (❌ *1.80x slower*)    |
| **`comparison`**              | `4.88 ns` (✅ **1.00x**)        | `5.12 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.36 ns` (✅ **1.00x**)        | `5.67 ns` (✅ **1.06x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.05 ns` (✅ **1.00x**) | `75.89 ns` (❌ *1.85x slower*)    |
| **`into_bigint`** | `22.41 ns` (✅ **1.00x**) | `47.88 ns` (❌ *2.14x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

