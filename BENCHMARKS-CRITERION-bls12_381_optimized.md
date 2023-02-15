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
|        | `220.25 us` (✅ **1.00x**)                 | `1.81 ms` (❌ *8.23x slower*)               |

### arithmetic_for_bls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_381`          | `g2projectivebls12_381`          | `fq2optimized`                   | `fq12optimized`                   | `fqoptimized`                    | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.22 us` (✅ **1.00x**)          | `3.87 us` (❌ *3.16x slower*)     | `23.29 ns` (🚀 **52.49x faster**) | `179.76 ns` (🚀 **6.80x faster**)  | `12.67 ns` (🚀 **96.47x faster**) | `8.66 ns` (🚀 **141.14x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.27 us` (✅ **1.00x**)          | `3.94 us` (❌ *3.11x slower*)     | `23.33 ns` (🚀 **54.30x faster**) | `158.25 ns` (🚀 **8.00x faster**)  | `12.89 ns` (🚀 **98.29x faster**) | `8.77 ns` (🚀 **144.37x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `885.26 ns` (✅ **1.00x**)        | `2.78 us` (❌ *3.15x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `923.45 ns` (✅ **1.00x**)        | `2.85 us` (❌ *3.08x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `583.95 ns` (✅ **1.00x**)        | `1.78 us` (❌ *3.05x slower*)     | `12.47 ns` (🚀 **46.84x faster**) | `67.37 ns` (🚀 **8.67x faster**)   | `7.24 ns` (🚀 **80.64x faster**)  | `5.87 ns` (🚀 **99.42x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `338.33 us` (✅ **1.00x**)        | `976.57 us` (❌ *2.89x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `22.70 ns` (❌ *3.68x slower*)    | `102.91 ns` (❌ *16.70x slower*)   | `18.33 ns` (❌ *2.97x slower*)    | `6.16 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `242.09 ns` (❌ *5.27x slower*)   | `6.23 us` (❌ *135.57x slower*)    | `76.60 ns` (❌ *1.67x slower*)    | `45.98 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `176.08 ns` (❌ *4.68x slower*)   | `4.38 us` (❌ *116.40x slower*)    | `65.02 ns` (❌ *1.73x slower*)    | `37.59 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `15.37 us` (❌ *2.15x slower*)    | `25.73 us` (❌ *3.60x slower*)     | `15.09 us` (❌ *2.11x slower*)    | `7.16 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `526.71 ns` (❌ *6.21x slower*)   | `12.79 us` (❌ *150.71x slower*)   | `118.25 ns` (❌ *1.39x slower*)   | `84.86 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `515.89 ns` (❌ *5.90x slower*)   | `12.72 us` (❌ *145.37x slower*)   | `163.09 ns` (❌ *1.86x slower*)   | `87.47 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**)        | `8.65 ns` (❌ *1.14x slower*)    | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.65 ns` (✅ **1.00x**)        | `10.48 ns` (❌ *1.21x slower*)   | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.87 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.70 ns` (✅ **1.00x**)        | `4.68 ns` (✅ **1.00x faster**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381_optimized

|                                          | `g1projectivebls12_381`          | `g2projectivebls12_381`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                     | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `150.75 ns` (✅ **1.00x**)        | `202.55 ns` (❌ *1.34x slower*)   | `32.10 ns` (🚀 **4.70x faster**)    | `55.36 ns` (🚀 **2.72x faster**)    | `108.89 ns` (✅ **1.38x faster**)   | `704.63 ns` (❌ *4.67x slower*)    |
| **`serialize_uncompressed`**             | `191.54 ns` (✅ **1.00x**)        | `284.39 ns` (❌ *1.48x slower*)   | `32.69 ns` (🚀 **5.86x faster**)    | `55.26 ns` (🚀 **3.47x faster**)    | `108.88 ns` (✅ **1.76x faster**)   | `698.58 ns` (❌ *3.65x slower*)    |
| **`deserialize_compressed`**             | `165.40 us` (✅ **1.00x**)        | `284.98 us` (❌ *1.72x slower*)   | `52.55 ns` (🚀 **3147.27x faster**) | `93.97 ns` (🚀 **1760.06x faster**) | `218.70 ns` (🚀 **756.30x faster**) | `1.34 us` (🚀 **123.47x faster**)  |
| **`deserialize_compressed_unchecked`**   | `39.64 us` (✅ **1.00x**)         | `135.27 us` (❌ *3.41x slower*)   | `52.55 ns` (🚀 **754.37x faster**)  | `93.99 ns` (🚀 **421.77x faster**)  | `218.66 ns` (🚀 **181.29x faster**) | `1.34 us` (🚀 **29.61x faster**)   |
| **`deserialize_uncompressed`**           | `125.58 us` (✅ **1.00x**)        | `149.35 us` (❌ *1.19x slower*)   | `52.50 ns` (🚀 **2391.85x faster**) | `93.89 ns` (🚀 **1337.55x faster**) | `218.29 ns` (🚀 **575.31x faster**) | `1.34 us` (🚀 **93.76x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `220.90 ns` (✅ **1.00x**)        | `481.17 ns` (❌ *2.18x slower*)   | `52.38 ns` (🚀 **4.22x faster**)    | `93.87 ns` (🚀 **2.35x faster**)    | `218.26 ns` (✅ **1.01x faster**)   | `1.34 us` (❌ *6.07x slower*)      |

### msm_for_bls12_381_optimized

|        | `g1projectivebls12_381`          | `g2projectivebls12_381`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.36 s` (✅ **1.00x**)           | `7.00 s` (❌ *2.96x slower*)       |

### squareroot_for_bls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `25.41 us` (✅ **1.00x**) | `39.34 us` (❌ *1.55x slower*)   | `134.44 us` (❌ *5.29x slower*)    |
| **`legendre_for_qr`**    | `14.33 us` (✅ **1.00x**) | `39.41 us` (❌ *2.75x slower*)   | `39.48 us` (❌ *2.76x slower*)     |

### bitwise_operations_for_bls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.84 ns` (✅ **1.00x**)        | `5.03 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.22 ns` (✅ **1.00x**)       | `89.16 ns` (❌ *1.85x slower*)    |
| **`from_big-endian_bits`**    | `48.16 ns` (✅ **1.00x**)       | `89.18 ns` (❌ *1.85x slower*)    |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)        | `5.12 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.36 ns` (✅ **1.00x**)        | `5.65 ns` (✅ **1.05x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.04 ns` (✅ **1.00x**) | `76.07 ns` (❌ *1.85x slower*)    |
| **`into_bigint`** | `22.42 ns` (✅ **1.00x**) | `47.89 ns` (❌ *2.14x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

