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
|        | `220.27 us` (✅ **1.00x**)                 | `1.81 ms` (❌ *8.23x slower*)               |

### arithmetic_for_bls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_381`          | `g2projectivebls12_381`          | `fq2optimized`                   | `fq12optimized`                   | `fqoptimized`                    | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.22 us` (✅ **1.00x**)          | `3.87 us` (❌ *3.17x slower*)     | `23.30 ns` (🚀 **52.43x faster**) | `181.82 ns` (🚀 **6.72x faster**)  | `12.68 ns` (🚀 **96.33x faster**) | `8.65 ns` (🚀 **141.17x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.27 us` (✅ **1.00x**)          | `3.93 us` (❌ *3.10x slower*)     | `23.32 ns` (🚀 **54.40x faster**) | `159.92 ns` (🚀 **7.93x faster**)  | `12.87 ns` (🚀 **98.57x faster**) | `8.77 ns` (🚀 **144.64x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `885.27 ns` (✅ **1.00x**)        | `2.78 us` (❌ *3.14x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `922.70 ns` (✅ **1.00x**)        | `2.82 us` (❌ *3.06x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `582.50 ns` (✅ **1.00x**)        | `1.78 us` (❌ *3.05x slower*)     | `12.53 ns` (🚀 **46.49x faster**) | `74.00 ns` (🚀 **7.87x faster**)   | `7.25 ns` (🚀 **80.29x faster**)  | `5.89 ns` (🚀 **98.86x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `338.30 us` (✅ **1.00x**)        | `977.43 us` (❌ *2.89x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `22.14 ns` (❌ *3.60x slower*)    | `102.02 ns` (❌ *16.57x slower*)   | `18.34 ns` (❌ *2.98x slower*)    | `6.16 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `242.46 ns` (❌ *5.28x slower*)   | `6.24 us` (❌ *135.86x slower*)    | `76.40 ns` (❌ *1.66x slower*)    | `45.95 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `176.07 ns` (❌ *4.69x slower*)   | `4.38 us` (❌ *116.54x slower*)    | `65.04 ns` (❌ *1.73x slower*)    | `37.58 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `15.36 us` (❌ *2.15x slower*)    | `25.72 us` (❌ *3.60x slower*)     | `15.06 us` (❌ *2.11x slower*)    | `7.14 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `525.93 ns` (❌ *6.20x slower*)   | `12.79 us` (❌ *150.73x slower*)   | `118.29 ns` (❌ *1.39x slower*)   | `84.86 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `515.21 ns` (❌ *5.89x slower*)   | `12.71 us` (❌ *145.33x slower*)   | `163.07 ns` (❌ *1.86x slower*)   | `87.46 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**)        | `8.65 ns` (❌ *1.14x slower*)    | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.65 ns` (✅ **1.00x**)        | `10.49 ns` (❌ *1.21x slower*)   | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.87 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.68 ns` (✅ **1.00x**)        | `4.74 ns` (✅ **1.01x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381_optimized

|                                          | `g1projectivebls12_381`          | `g2projectivebls12_381`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                     | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `150.62 ns` (✅ **1.00x**)        | `202.55 ns` (❌ *1.34x slower*)   | `32.29 ns` (🚀 **4.66x faster**)    | `55.52 ns` (🚀 **2.71x faster**)    | `108.82 ns` (✅ **1.38x faster**)   | `704.90 ns` (❌ *4.68x slower*)    |
| **`serialize_uncompressed`**             | `191.63 ns` (✅ **1.00x**)        | `284.50 ns` (❌ *1.48x slower*)   | `32.60 ns` (🚀 **5.88x faster**)    | `55.28 ns` (🚀 **3.47x faster**)    | `109.07 ns` (✅ **1.76x faster**)   | `698.70 ns` (❌ *3.65x slower*)    |
| **`deserialize_compressed`**             | `165.42 us` (✅ **1.00x**)        | `285.17 us` (❌ *1.72x slower*)   | `52.32 ns` (🚀 **3161.63x faster**) | `92.96 ns` (🚀 **1779.50x faster**) | `213.12 ns` (🚀 **776.21x faster**) | `1.32 us` (🚀 **124.96x faster**)  |
| **`deserialize_compressed_unchecked`**   | `39.63 us` (✅ **1.00x**)         | `135.24 us` (❌ *3.41x slower*)   | `52.29 ns` (🚀 **757.92x faster**)  | `93.16 ns` (🚀 **425.42x faster**)  | `213.02 ns` (🚀 **186.05x faster**) | `1.32 us` (🚀 **29.96x faster**)   |
| **`deserialize_uncompressed`**           | `125.56 us` (✅ **1.00x**)        | `150.93 us` (❌ *1.20x slower*)   | `52.28 ns` (🚀 **2401.60x faster**) | `93.23 ns` (🚀 **1346.78x faster**) | `212.80 ns` (🚀 **590.06x faster**) | `1.33 us` (🚀 **94.76x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `222.80 ns` (✅ **1.00x**)        | `484.60 ns` (❌ *2.18x slower*)   | `52.25 ns` (🚀 **4.26x faster**)    | `93.12 ns` (🚀 **2.39x faster**)    | `212.70 ns` (✅ **1.05x faster**)   | `1.32 us` (❌ *5.94x slower*)      |

### msm_for_bls12_381_optimized

|        | `g1projectivebls12_381`          | `g2projectivebls12_381`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.36 s` (✅ **1.00x**)           | `6.98 s` (❌ *2.96x slower*)       |

### squareroot_for_bls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `25.46 us` (✅ **1.00x**) | `39.35 us` (❌ *1.55x slower*)   | `134.34 us` (❌ *5.28x slower*)    |
| **`legendre_for_qr`**    | `14.32 us` (✅ **1.00x**) | `39.42 us` (❌ *2.75x slower*)   | `39.46 us` (❌ *2.76x slower*)     |

### bitwise_operations_for_bls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.86 ns` (✅ **1.00x**)        | `5.03 ns` (✅ **1.03x slower**)   |
| **`from_little-endian_bits`** | `48.95 ns` (✅ **1.00x**)       | `90.02 ns` (❌ *1.84x slower*)    |
| **`from_big-endian_bits`**    | `48.89 ns` (✅ **1.00x**)       | `89.53 ns` (❌ *1.83x slower*)    |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)        | `5.12 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.36 ns` (✅ **1.00x**)        | `5.67 ns` (✅ **1.06x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.05 ns` (✅ **1.00x**) | `76.53 ns` (❌ *1.86x slower*)    |
| **`into_bigint`** | `22.42 ns` (✅ **1.00x**) | `47.86 ns` (❌ *2.13x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

