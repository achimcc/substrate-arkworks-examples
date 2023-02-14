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
|        | `220.49 us` (✅ **1.00x**)                 | `1.82 ms` (❌ *8.23x slower*)               |

### arithmetic_for_bls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_381`          | `g2projectivebls12_381`          | `fq2optimized`                   | `fq12optimized`                   | `fqoptimized`                    | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.22 us` (✅ **1.00x**)          | `3.88 us` (❌ *3.17x slower*)     | `23.29 ns` (🚀 **52.51x faster**) | `186.44 ns` (🚀 **6.56x faster**)  | `12.71 ns` (🚀 **96.20x faster**) | `8.67 ns` (🚀 **141.06x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.27 us` (✅ **1.00x**)          | `3.95 us` (❌ *3.12x slower*)     | `23.33 ns` (🚀 **54.28x faster**) | `160.56 ns` (🚀 **7.89x faster**)  | `12.96 ns` (🚀 **97.76x faster**) | `8.80 ns` (🚀 **143.90x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `887.48 ns` (✅ **1.00x**)        | `2.79 us` (❌ *3.14x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `924.32 ns` (✅ **1.00x**)        | `2.82 us` (❌ *3.05x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `584.22 ns` (✅ **1.00x**)        | `1.78 us` (❌ *3.05x slower*)     | `12.55 ns` (🚀 **46.55x faster**) | `72.03 ns` (🚀 **8.11x faster**)   | `7.24 ns` (🚀 **80.66x faster**)  | `5.88 ns` (🚀 **99.31x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `338.47 us` (✅ **1.00x**)        | `976.95 us` (❌ *2.89x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `22.21 ns` (❌ *3.61x slower*)    | `102.20 ns` (❌ *16.59x slower*)   | `18.12 ns` (❌ *2.94x slower*)    | `6.16 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `242.17 ns` (❌ *5.27x slower*)   | `6.26 us` (❌ *136.19x slower*)    | `77.04 ns` (❌ *1.68x slower*)    | `45.98 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `176.91 ns` (❌ *4.71x slower*)   | `4.37 us` (❌ *116.30x slower*)    | `65.00 ns` (❌ *1.73x slower*)    | `37.57 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `15.37 us` (❌ *2.13x slower*)    | `25.71 us` (❌ *3.57x slower*)     | `15.06 us` (❌ *2.09x slower*)    | `7.20 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `525.12 ns` (❌ *6.19x slower*)   | `12.79 us` (❌ *150.71x slower*)   | `118.25 ns` (❌ *1.39x slower*)   | `84.89 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `515.68 ns` (❌ *5.89x slower*)   | `12.72 us` (❌ *145.35x slower*)   | `163.24 ns` (❌ *1.87x slower*)   | `87.51 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**)        | `8.65 ns` (❌ *1.14x slower*)    | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.65 ns` (✅ **1.00x**)        | `10.49 ns` (❌ *1.21x slower*)   | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.88 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.66 ns` (✅ **1.00x**)        | `4.67 ns` (✅ **1.00x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381_optimized

|                                          | `g1projectivebls12_381`          | `g2projectivebls12_381`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                     | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `150.60 ns` (✅ **1.00x**)        | `203.00 ns` (❌ *1.35x slower*)   | `32.26 ns` (🚀 **4.67x faster**)    | `55.33 ns` (🚀 **2.72x faster**)    | `108.89 ns` (✅ **1.38x faster**)   | `705.95 ns` (❌ *4.69x slower*)    |
| **`serialize_uncompressed`**             | `192.12 ns` (✅ **1.00x**)        | `284.25 ns` (❌ *1.48x slower*)   | `32.61 ns` (🚀 **5.89x faster**)    | `55.27 ns` (🚀 **3.48x faster**)    | `108.93 ns` (✅ **1.76x faster**)   | `699.64 ns` (❌ *3.64x slower*)    |
| **`deserialize_compressed`**             | `165.54 us` (✅ **1.00x**)        | `284.96 us` (❌ *1.72x slower*)   | `52.38 ns` (🚀 **3160.14x faster**) | `94.79 ns` (🚀 **1746.26x faster**) | `220.93 ns` (🚀 **749.28x faster**) | `1.32 us` (🚀 **125.07x faster**)  |
| **`deserialize_compressed_unchecked`**   | `39.62 us` (✅ **1.00x**)         | `135.25 us` (❌ *3.41x slower*)   | `52.50 ns` (🚀 **754.65x faster**)  | `95.03 ns` (🚀 **416.90x faster**)  | `220.54 ns` (🚀 **179.64x faster**) | `1.32 us` (🚀 **29.94x faster**)   |
| **`deserialize_uncompressed`**           | `125.80 us` (✅ **1.00x**)        | `149.33 us` (❌ *1.19x slower*)   | `52.47 ns` (🚀 **2397.55x faster**) | `94.90 ns` (🚀 **1325.58x faster**) | `220.40 ns` (🚀 **570.78x faster**) | `1.32 us` (🚀 **94.94x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `221.33 ns` (✅ **1.00x**)        | `487.51 ns` (❌ *2.20x slower*)   | `52.41 ns` (🚀 **4.22x faster**)    | `94.97 ns` (🚀 **2.33x faster**)    | `220.48 ns` (✅ **1.00x faster**)   | `1.32 us` (❌ *5.98x slower*)      |

### msm_for_bls12_381_optimized

|        | `g1projectivebls12_381`          | `g2projectivebls12_381`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.36 s` (✅ **1.00x**)           | `7.03 s` (❌ *2.98x slower*)       |

### squareroot_for_bls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `25.41 us` (✅ **1.00x**) | `39.34 us` (❌ *1.55x slower*)   | `134.36 us` (❌ *5.29x slower*)    |
| **`legendre_for_qr`**    | `14.32 us` (✅ **1.00x**) | `39.57 us` (❌ *2.76x slower*)   | `39.48 us` (❌ *2.76x slower*)     |

### bitwise_operations_for_bls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `5.03 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `49.67 ns` (✅ **1.00x**)       | `89.23 ns` (❌ *1.80x slower*)    |
| **`from_big-endian_bits`**    | `49.65 ns` (✅ **1.00x**)       | `88.92 ns` (❌ *1.79x slower*)    |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)        | `5.12 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.37 ns` (✅ **1.00x**)        | `5.67 ns` (✅ **1.06x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.06 ns` (✅ **1.00x**) | `76.12 ns` (❌ *1.85x slower*)    |
| **`into_bigint`** | `22.42 ns` (✅ **1.00x**) | `47.90 ns` (❌ *2.14x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

