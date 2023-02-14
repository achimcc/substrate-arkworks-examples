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
|        | `220.44 us` (✅ **1.00x**)                 | `1.81 ms` (❌ *8.22x slower*)               |

### arithmetic_for_bls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_381`          | `g2projectivebls12_381`          | `fq2optimized`                   | `fq12optimized`                   | `fqoptimized`                    | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.22 us` (✅ **1.00x**)          | `3.87 us` (❌ *3.16x slower*)     | `23.27 ns` (🚀 **52.53x faster**) | `181.83 ns` (🚀 **6.72x faster**)  | `12.67 ns` (🚀 **96.51x faster**) | `8.66 ns` (🚀 **141.19x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.27 us` (✅ **1.00x**)          | `3.95 us` (❌ *3.12x slower*)     | `23.35 ns` (🚀 **54.25x faster**) | `158.63 ns` (🚀 **7.99x faster**)  | `12.90 ns` (🚀 **98.19x faster**) | `8.78 ns` (🚀 **144.29x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `884.88 ns` (✅ **1.00x**)        | `2.79 us` (❌ *3.15x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `922.81 ns` (✅ **1.00x**)        | `2.83 us` (❌ *3.07x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `583.54 ns` (✅ **1.00x**)        | `1.78 us` (❌ *3.05x slower*)     | `12.50 ns` (🚀 **46.68x faster**) | `71.88 ns` (🚀 **8.12x faster**)   | `7.24 ns` (🚀 **80.58x faster**)  | `5.86 ns` (🚀 **99.60x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `338.34 us` (✅ **1.00x**)        | `976.62 us` (❌ *2.89x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `22.74 ns` (❌ *3.69x slower*)    | `100.98 ns` (❌ *16.38x slower*)   | `18.34 ns` (❌ *2.98x slower*)    | `6.16 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `242.04 ns` (❌ *5.27x slower*)   | `6.23 us` (❌ *135.47x slower*)    | `76.59 ns` (❌ *1.67x slower*)    | `45.97 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `175.67 ns` (❌ *4.68x slower*)   | `4.38 us` (❌ *116.54x slower*)    | `64.99 ns` (❌ *1.73x slower*)    | `37.55 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `15.36 us` (❌ *2.14x slower*)    | `25.75 us` (❌ *3.60x slower*)     | `15.07 us` (❌ *2.10x slower*)    | `7.16 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `526.32 ns` (❌ *6.20x slower*)   | `12.78 us` (❌ *150.61x slower*)   | `118.22 ns` (❌ *1.39x slower*)   | `84.87 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `515.66 ns` (❌ *5.89x slower*)   | `12.70 us` (❌ *145.18x slower*)   | `163.18 ns` (❌ *1.86x slower*)   | `87.50 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**)        | `8.65 ns` (❌ *1.14x slower*)    | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.66 ns` (✅ **1.00x**)        | `10.47 ns` (❌ *1.21x slower*)   | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.87 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.70 ns` (✅ **1.00x**)        | `4.77 ns` (✅ **1.01x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381_optimized

|                                          | `g1projectivebls12_381`          | `g2projectivebls12_381`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                     | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `150.81 ns` (✅ **1.00x**)        | `202.22 ns` (❌ *1.34x slower*)   | `32.31 ns` (🚀 **4.67x faster**)    | `55.32 ns` (🚀 **2.73x faster**)    | `108.89 ns` (✅ **1.39x faster**)   | `704.92 ns` (❌ *4.67x slower*)    |
| **`serialize_uncompressed`**             | `191.52 ns` (✅ **1.00x**)        | `286.93 ns` (❌ *1.50x slower*)   | `32.23 ns` (🚀 **5.94x faster**)    | `55.26 ns` (🚀 **3.47x faster**)    | `108.88 ns` (✅ **1.76x faster**)   | `699.63 ns` (❌ *3.65x slower*)    |
| **`deserialize_compressed`**             | `165.38 us` (✅ **1.00x**)        | `284.89 us` (❌ *1.72x slower*)   | `52.49 ns` (🚀 **3150.91x faster**) | `93.83 ns` (🚀 **1762.49x faster**) | `213.08 ns` (🚀 **776.15x faster**) | `1.34 us` (🚀 **123.53x faster**)  |
| **`deserialize_compressed_unchecked`**   | `39.60 us` (✅ **1.00x**)         | `135.27 us` (❌ *3.42x slower*)   | `52.46 ns` (🚀 **754.89x faster**)  | `93.60 ns` (🚀 **423.06x faster**)  | `213.11 ns` (🚀 **185.82x faster**) | `1.34 us` (🚀 **29.58x faster**)   |
| **`deserialize_uncompressed`**           | `125.55 us` (✅ **1.00x**)        | `149.36 us` (❌ *1.19x slower*)   | `52.24 ns` (🚀 **2403.26x faster**) | `93.80 ns` (🚀 **1338.46x faster**) | `213.56 ns` (🚀 **587.89x faster**) | `1.34 us` (🚀 **93.75x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `221.52 ns` (✅ **1.00x**)        | `485.29 ns` (❌ *2.19x slower*)   | `52.31 ns` (🚀 **4.23x faster**)    | `93.80 ns` (🚀 **2.36x faster**)    | `214.02 ns` (✅ **1.04x faster**)   | `1.34 us` (❌ *6.04x slower*)      |

### msm_for_bls12_381_optimized

|        | `g1projectivebls12_381`          | `g2projectivebls12_381`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.36 s` (✅ **1.00x**)           | `6.98 s` (❌ *2.96x slower*)       |

### squareroot_for_bls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `25.42 us` (✅ **1.00x**) | `39.30 us` (❌ *1.55x slower*)   | `134.33 us` (❌ *5.29x slower*)    |
| **`legendre_for_qr`**    | `14.32 us` (✅ **1.00x**) | `39.43 us` (❌ *2.75x slower*)   | `39.44 us` (❌ *2.75x slower*)     |

### bitwise_operations_for_bls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `5.03 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.16 ns` (✅ **1.00x**)       | `83.33 ns` (❌ *1.73x slower*)    |
| **`from_big-endian_bits`**    | `48.18 ns` (✅ **1.00x**)       | `83.35 ns` (❌ *1.73x slower*)    |
| **`comparison`**              | `4.90 ns` (✅ **1.00x**)        | `5.12 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.37 ns` (✅ **1.00x**)        | `5.67 ns` (✅ **1.06x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.06 ns` (✅ **1.00x**) | `75.98 ns` (❌ *1.85x slower*)    |
| **`into_bigint`** | `22.42 ns` (✅ **1.00x**) | `47.91 ns` (❌ *2.14x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

