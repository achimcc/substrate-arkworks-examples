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
|        | `196.38 us` (✅ **1.00x**)                 | `1.64 ms` (❌ *8.33x slower*)               |

### arithmetic_for_bls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_381`          | `g2projectivebls12_381`          | `fq2optimized`                   | `fq12optimized`                   | `fqoptimized`                    | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.11 us` (✅ **1.00x**)          | `3.57 us` (❌ *3.20x slower*)     | `26.75 ns` (🚀 **41.64x faster**) | `179.91 ns` (🚀 **6.19x faster**)  | `19.13 ns` (🚀 **58.22x faster**) | `8.20 ns` (🚀 **135.76x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.15 us` (✅ **1.00x**)          | `3.61 us` (❌ *3.14x slower*)     | `27.46 ns` (🚀 **41.85x faster**) | `172.70 ns` (🚀 **6.65x faster**)  | `16.15 ns` (🚀 **71.14x faster**) | `8.65 ns` (🚀 **132.79x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `814.50 ns` (✅ **1.00x**)        | `2.58 us` (❌ *3.16x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `838.59 ns` (✅ **1.00x**)        | `2.61 us` (❌ *3.11x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `562.13 ns` (✅ **1.00x**)        | `1.63 us` (❌ *2.90x slower*)     | `12.97 ns` (🚀 **43.33x faster**) | `105.01 ns` (🚀 **5.35x faster**)  | `7.62 ns` (🚀 **73.82x faster**)  | `5.40 ns` (🚀 **104.16x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `298.30 us` (✅ **1.00x**)        | `884.71 us` (❌ *2.97x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `22.53 ns` (❌ *3.79x slower*)    | `107.15 ns` (❌ *18.01x slower*)   | `17.01 ns` (❌ *2.86x slower*)    | `5.95 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `223.85 ns` (❌ *5.76x slower*)   | `5.74 us` (❌ *147.67x slower*)    | `70.24 ns` (❌ *1.81x slower*)    | `38.87 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `175.20 ns` (❌ *4.93x slower*)   | `4.04 us` (❌ *113.75x slower*)    | `58.51 ns` (❌ *1.65x slower*)    | `35.55 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `13.86 us` (❌ *2.16x slower*)    | `23.10 us` (❌ *3.60x slower*)     | `13.52 us` (❌ *2.11x slower*)    | `6.42 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `493.00 ns` (❌ *6.08x slower*)   | `11.72 us` (❌ *144.46x slower*)   | `106.26 ns` (❌ *1.31x slower*)   | `81.14 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `474.22 ns` (❌ *5.85x slower*)   | `11.65 us` (❌ *143.67x slower*)   | `156.57 ns` (❌ *1.93x slower*)   | `81.11 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.52 ns` (✅ **1.00x**)        | `7.83 ns` (❌ *1.20x slower*)    | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**)        | `10.62 ns` (❌ *1.35x slower*)   | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**)        | `4.04 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**)        | `3.74 ns` (✅ **1.00x faster**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381_optimized

|                                          | `g1projectivebls12_381`          | `g2projectivebls12_381`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                     | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `141.26 ns` (✅ **1.00x**)        | `191.02 ns` (❌ *1.35x slower*)   | `29.89 ns` (🚀 **4.73x faster**)    | `49.56 ns` (🚀 **2.85x faster**)    | `97.83 ns` (✅ **1.44x faster**)    | `634.21 ns` (❌ *4.49x slower*)    |
| **`serialize_uncompressed`**             | `179.73 ns` (✅ **1.00x**)        | `268.08 ns` (❌ *1.49x slower*)   | `29.81 ns` (🚀 **6.03x faster**)    | `49.58 ns` (🚀 **3.63x faster**)    | `97.82 ns` (🚀 **1.84x faster**)    | `629.86 ns` (❌ *3.50x slower*)    |
| **`deserialize_compressed`**             | `148.48 us` (✅ **1.00x**)        | `257.77 us` (❌ *1.74x slower*)   | `46.54 ns` (🚀 **3190.09x faster**) | `95.55 ns` (🚀 **1553.86x faster**) | `205.78 ns` (🚀 **721.54x faster**) | `1.27 us` (🚀 **116.58x faster**)  |
| **`deserialize_compressed_unchecked`**   | `36.00 us` (✅ **1.00x**)         | `122.69 us` (❌ *3.41x slower*)   | `46.54 ns` (🚀 **773.63x faster**)  | `95.56 ns` (🚀 **376.76x faster**)  | `205.82 ns` (🚀 **174.93x faster**) | `1.27 us` (🚀 **28.27x faster**)   |
| **`deserialize_uncompressed`**           | `112.26 us` (✅ **1.00x**)        | `134.78 us` (❌ *1.20x slower*)   | `46.46 ns` (🚀 **2416.22x faster**) | `95.51 ns` (🚀 **1175.35x faster**) | `205.68 ns` (🚀 **545.81x faster**) | `1.27 us` (🚀 **88.13x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `232.19 ns` (✅ **1.00x**)        | `497.00 ns` (❌ *2.14x slower*)   | `46.46 ns` (🚀 **5.00x faster**)    | `95.51 ns` (🚀 **2.43x faster**)    | `206.77 ns` (✅ **1.12x faster**)   | `1.27 us` (❌ *5.49x slower*)      |

### msm_for_bls12_381_optimized

|        | `g1projectivebls12_381`          | `g2projectivebls12_381`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.22 s` (✅ **1.00x**)           | `6.58 s` (❌ *2.97x slower*)       |

### squareroot_for_bls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `22.14 us` (✅ **1.00x**) | `35.61 us` (❌ *1.61x slower*)   | `121.80 us` (❌ *5.50x slower*)    |
| **`legendre_for_qr`**    | `12.49 us` (✅ **1.00x**) | `35.90 us` (❌ *2.88x slower*)   | `35.88 us` (❌ *2.87x slower*)     |

### bitwise_operations_for_bls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)        | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `60.87 ns` (✅ **1.00x**)       | `108.73 ns` (❌ *1.79x slower*)    |
| **`from_big-endian_bits`**    | `60.83 ns` (✅ **1.00x**)       | `108.45 ns` (❌ *1.78x slower*)    |
| **`comparison`**              | `4.08 ns` (✅ **1.00x**)        | `4.31 ns` (✅ **1.06x slower**)    |
| **`equality`**                | `4.50 ns` (✅ **1.00x**)        | `4.64 ns` (✅ **1.03x slower**)    |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)        | `4.00 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `36.08 ns` (✅ **1.00x**) | `78.95 ns` (❌ *2.19x slower*)    |
| **`into_bigint`** | `21.57 ns` (✅ **1.00x**) | `41.41 ns` (❌ *1.92x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

