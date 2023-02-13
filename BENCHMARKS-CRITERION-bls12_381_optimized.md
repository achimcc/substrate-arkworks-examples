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
|        | `220.24 us` (✅ **1.00x**)                 | `1.81 ms` (❌ *8.23x slower*)               |

### arithmetic_for_bls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_381`          | `g2projectivebls12_381`          | `fq2optimized`                   | `fq12optimized`                   | `fqoptimized`                    | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.22 us` (✅ **1.00x**)          | `3.88 us` (❌ *3.18x slower*)     | `23.30 ns` (🚀 **52.35x faster**) | `180.84 ns` (🚀 **6.74x faster**)  | `12.66 ns` (🚀 **96.33x faster**) | `8.66 ns` (🚀 **140.79x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.27 us` (✅ **1.00x**)          | `3.93 us` (❌ *3.10x slower*)     | `23.32 ns` (🚀 **54.36x faster**) | `158.46 ns` (🚀 **8.00x faster**)  | `12.94 ns` (🚀 **97.99x faster**) | `8.77 ns` (🚀 **144.50x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `886.22 ns` (✅ **1.00x**)        | `2.78 us` (❌ *3.14x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `923.01 ns` (✅ **1.00x**)        | `2.83 us` (❌ *3.06x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `584.27 ns` (✅ **1.00x**)        | `1.78 us` (❌ *3.05x slower*)     | `12.51 ns` (🚀 **46.69x faster**) | `71.11 ns` (🚀 **8.22x faster**)   | `7.24 ns` (🚀 **80.65x faster**)  | `5.87 ns` (🚀 **99.48x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `338.24 us` (✅ **1.00x**)        | `976.80 us` (❌ *2.89x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `23.07 ns` (❌ *3.75x slower*)    | `101.08 ns` (❌ *16.42x slower*)   | `18.79 ns` (❌ *3.05x slower*)    | `6.16 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `241.98 ns` (❌ *5.26x slower*)   | `6.24 us` (❌ *135.65x slower*)    | `76.41 ns` (❌ *1.66x slower*)    | `45.97 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `176.17 ns` (❌ *4.69x slower*)   | `4.37 us` (❌ *116.29x slower*)    | `65.07 ns` (❌ *1.73x slower*)    | `37.58 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `15.41 us` (❌ *2.15x slower*)    | `25.73 us` (❌ *3.60x slower*)     | `15.10 us` (❌ *2.11x slower*)    | `7.15 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `526.45 ns` (❌ *6.19x slower*)   | `12.79 us` (❌ *150.27x slower*)   | `118.29 ns` (❌ *1.39x slower*)   | `85.11 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `514.96 ns` (❌ *5.89x slower*)   | `12.70 us` (❌ *145.22x slower*)   | `163.41 ns` (❌ *1.87x slower*)   | `87.45 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**)        | `8.66 ns` (❌ *1.14x slower*)    | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.66 ns` (✅ **1.00x**)        | `10.50 ns` (❌ *1.21x slower*)   | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.87 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.70 ns` (✅ **1.00x**)        | `4.72 ns` (✅ **1.00x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381_optimized

|                                          | `g1projectivebls12_381`          | `g2projectivebls12_381`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                     | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `150.62 ns` (✅ **1.00x**)        | `228.13 ns` (❌ *1.51x slower*)   | `32.21 ns` (🚀 **4.68x faster**)    | `55.37 ns` (🚀 **2.72x faster**)    | `108.99 ns` (✅ **1.38x faster**)   | `704.32 ns` (❌ *4.68x slower*)    |
| **`serialize_uncompressed`**             | `191.69 ns` (✅ **1.00x**)        | `309.85 ns` (❌ *1.62x slower*)   | `32.75 ns` (🚀 **5.85x faster**)    | `55.25 ns` (🚀 **3.47x faster**)    | `109.01 ns` (✅ **1.76x faster**)   | `698.46 ns` (❌ *3.64x slower*)    |
| **`deserialize_compressed`**             | `165.37 us` (✅ **1.00x**)        | `285.07 us` (❌ *1.72x slower*)   | `52.54 ns` (🚀 **3147.66x faster**) | `93.25 ns` (🚀 **1773.34x faster**) | `214.41 ns` (🚀 **771.27x faster**) | `1.32 us` (🚀 **124.98x faster**)  |
| **`deserialize_compressed_unchecked`**   | `39.62 us` (✅ **1.00x**)         | `135.24 us` (❌ *3.41x slower*)   | `52.62 ns` (🚀 **753.01x faster**)  | `93.38 ns` (🚀 **424.29x faster**)  | `213.94 ns` (🚀 **185.20x faster**) | `1.32 us` (🚀 **29.96x faster**)   |
| **`deserialize_uncompressed`**           | `125.51 us` (✅ **1.00x**)        | `149.26 us` (❌ *1.19x slower*)   | `52.50 ns` (🚀 **2390.46x faster**) | `93.29 ns` (🚀 **1345.39x faster**) | `213.43 ns` (🚀 **588.07x faster**) | `1.32 us` (🚀 **94.92x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `224.40 ns` (✅ **1.00x**)        | `492.97 ns` (❌ *2.20x slower*)   | `52.51 ns` (🚀 **4.27x faster**)    | `93.26 ns` (🚀 **2.41x faster**)    | `213.38 ns` (✅ **1.05x faster**)   | `1.32 us` (❌ *5.90x slower*)      |

### msm_for_bls12_381_optimized

|        | `g1projectivebls12_381`          | `g2projectivebls12_381`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.35 s` (✅ **1.00x**)           | `6.98 s` (❌ *2.97x slower*)       |

### squareroot_for_bls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `25.43 us` (✅ **1.00x**) | `39.34 us` (❌ *1.55x slower*)   | `134.35 us` (❌ *5.28x slower*)    |
| **`legendre_for_qr`**    | `14.33 us` (✅ **1.00x**) | `39.44 us` (❌ *2.75x slower*)   | `39.46 us` (❌ *2.75x slower*)     |

### bitwise_operations_for_bls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.16 ns` (✅ **1.00x**)       | `89.38 ns` (❌ *1.86x slower*)    |
| **`from_big-endian_bits`**    | `48.11 ns` (✅ **1.00x**)       | `89.22 ns` (❌ *1.85x slower*)    |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)        | `5.12 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.37 ns` (✅ **1.00x**)        | `5.67 ns` (✅ **1.06x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.05 ns` (✅ **1.00x**) | `75.88 ns` (❌ *1.85x slower*)    |
| **`into_bigint`** | `22.42 ns` (✅ **1.00x**) | `47.88 ns` (❌ *2.14x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

