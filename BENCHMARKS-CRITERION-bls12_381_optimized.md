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
|        | `220.27 us` (✅ **1.00x**)                 | `1.81 ms` (❌ *8.22x slower*)               |

### arithmetic_for_bls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_381`          | `g2projectivebls12_381`          | `fq2optimized`                   | `fq12optimized`                   | `fqoptimized`                    | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.22 us` (✅ **1.00x**)          | `3.89 us` (❌ *3.18x slower*)     | `23.54 ns` (🚀 **51.97x faster**) | `182.38 ns` (🚀 **6.71x faster**)  | `12.68 ns` (🚀 **96.50x faster**) | `8.93 ns` (🚀 **136.93x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.27 us` (✅ **1.00x**)          | `3.92 us` (❌ *3.10x slower*)     | `23.34 ns` (🚀 **54.28x faster**) | `158.08 ns` (🚀 **8.02x faster**)  | `12.86 ns` (🚀 **98.50x faster**) | `8.78 ns` (🚀 **144.26x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `887.07 ns` (✅ **1.00x**)        | `2.79 us` (❌ *3.14x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `924.78 ns` (✅ **1.00x**)        | `2.82 us` (❌ *3.05x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `583.62 ns` (✅ **1.00x**)        | `1.78 us` (❌ *3.05x slower*)     | `14.20 ns` (🚀 **41.10x faster**) | `70.05 ns` (🚀 **8.33x faster**)   | `7.78 ns` (🚀 **75.02x faster**)  | `6.09 ns` (🚀 **95.88x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `339.63 us` (✅ **1.00x**)        | `980.66 us` (❌ *2.89x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `22.21 ns` (❌ *3.60x slower*)    | `101.90 ns` (❌ *16.53x slower*)   | `18.19 ns` (❌ *2.95x slower*)    | `6.17 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `242.16 ns` (❌ *5.24x slower*)   | `6.26 us` (❌ *135.41x slower*)    | `76.53 ns` (❌ *1.66x slower*)    | `46.20 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `176.88 ns` (❌ *4.68x slower*)   | `4.38 us` (❌ *115.87x slower*)    | `64.98 ns` (❌ *1.72x slower*)    | `37.77 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `15.36 us` (❌ *2.14x slower*)    | `25.74 us` (❌ *3.58x slower*)     | `15.06 us` (❌ *2.09x slower*)    | `7.19 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `527.31 ns` (❌ *6.21x slower*)   | `12.82 us` (❌ *150.89x slower*)   | `118.59 ns` (❌ *1.40x slower*)   | `84.96 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `515.50 ns` (❌ *5.89x slower*)   | `12.75 us` (❌ *145.64x slower*)   | `163.61 ns` (❌ *1.87x slower*)   | `87.52 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**)        | `8.65 ns` (❌ *1.14x slower*)    | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.65 ns` (✅ **1.00x**)        | `10.49 ns` (❌ *1.21x slower*)   | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.88 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.70 ns` (✅ **1.00x**)        | `4.62 ns` (✅ **1.02x faster**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381_optimized

|                                          | `g1projectivebls12_381`          | `g2projectivebls12_381`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                     | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `151.00 ns` (✅ **1.00x**)        | `203.31 ns` (❌ *1.35x slower*)   | `31.89 ns` (🚀 **4.74x faster**)    | `55.46 ns` (🚀 **2.72x faster**)    | `108.96 ns` (✅ **1.39x faster**)   | `703.37 ns` (❌ *4.66x slower*)    |
| **`serialize_uncompressed`**             | `192.13 ns` (✅ **1.00x**)        | `284.23 ns` (❌ *1.48x slower*)   | `32.02 ns` (🚀 **6.00x faster**)    | `55.34 ns` (🚀 **3.47x faster**)    | `108.98 ns` (✅ **1.76x faster**)   | `697.84 ns` (❌ *3.63x slower*)    |
| **`deserialize_compressed`**             | `165.46 us` (✅ **1.00x**)        | `285.14 us` (❌ *1.72x slower*)   | `52.60 ns` (🚀 **3145.39x faster**) | `94.85 ns` (🚀 **1744.47x faster**) | `220.85 ns` (🚀 **749.18x faster**) | `1.33 us` (🚀 **124.66x faster**)  |
| **`deserialize_compressed_unchecked`**   | `39.62 us` (✅ **1.00x**)         | `135.29 us` (❌ *3.41x slower*)   | `52.65 ns` (🚀 **752.55x faster**)  | `95.01 ns` (🚀 **417.03x faster**)  | `221.06 ns` (🚀 **179.23x faster**) | `1.33 us` (🚀 **29.87x faster**)   |
| **`deserialize_uncompressed`**           | `125.58 us` (✅ **1.00x**)        | `149.46 us` (❌ *1.19x slower*)   | `52.42 ns` (🚀 **2395.74x faster**) | `94.77 ns` (🚀 **1325.05x faster**) | `220.54 ns` (🚀 **569.40x faster**) | `1.33 us` (🚀 **94.69x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `224.81 ns` (✅ **1.00x**)        | `484.00 ns` (❌ *2.15x slower*)   | `52.47 ns` (🚀 **4.28x faster**)    | `94.75 ns` (🚀 **2.37x faster**)    | `220.75 ns` (✅ **1.02x faster**)   | `1.33 us` (❌ *5.90x slower*)      |

### msm_for_bls12_381_optimized

|        | `g1projectivebls12_381`          | `g2projectivebls12_381`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.36 s` (✅ **1.00x**)           | `7.05 s` (❌ *2.99x slower*)       |

### squareroot_for_bls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `25.43 us` (✅ **1.00x**) | `39.36 us` (❌ *1.55x slower*)   | `134.50 us` (❌ *5.29x slower*)    |
| **`legendre_for_qr`**    | `14.34 us` (✅ **1.00x**) | `39.50 us` (❌ *2.75x slower*)   | `39.51 us` (❌ *2.76x slower*)     |

### bitwise_operations_for_bls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `5.03 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.14 ns` (✅ **1.00x**)       | `95.90 ns` (❌ *1.99x slower*)    |
| **`from_big-endian_bits`**    | `48.13 ns` (✅ **1.00x**)       | `89.53 ns` (❌ *1.86x slower*)    |
| **`comparison`**              | `5.59 ns` (✅ **1.00x**)        | `5.12 ns` (✅ **1.09x faster**)   |
| **`equality`**                | `5.37 ns` (✅ **1.00x**)        | `5.69 ns` (✅ **1.06x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.10 ns` (✅ **1.00x**) | `76.12 ns` (❌ *1.85x slower*)    |
| **`into_bigint`** | `22.43 ns` (✅ **1.00x**) | `47.89 ns` (❌ *2.13x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

