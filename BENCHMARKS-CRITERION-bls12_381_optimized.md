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
|        | `196.86 us` (✅ **1.00x**)                 | `1.64 ms` (❌ *8.35x slower*)               |

### arithmetic_for_bls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_381`          | `g2projectivebls12_381`          | `fq2optimized`                   | `fq12optimized`                   | `fqoptimized`                    | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.11 us` (✅ **1.00x**)          | `3.57 us` (❌ *3.21x slower*)     | `29.45 ns` (🚀 **37.73x faster**) | `180.72 ns` (🚀 **6.15x faster**)  | `19.13 ns` (🚀 **58.07x faster**) | `8.18 ns` (🚀 **135.85x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.15 us` (✅ **1.00x**)          | `3.61 us` (❌ *3.15x slower*)     | `27.21 ns` (🚀 **42.18x faster**) | `167.42 ns` (🚀 **6.86x faster**)  | `14.51 ns` (🚀 **79.12x faster**) | `8.60 ns` (🚀 **133.49x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `811.93 ns` (✅ **1.00x**)        | `2.58 us` (❌ *3.18x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `834.70 ns` (✅ **1.00x**)        | `2.61 us` (❌ *3.13x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `557.70 ns` (✅ **1.00x**)        | `1.63 us` (❌ *2.92x slower*)     | `13.08 ns` (🚀 **42.63x faster**) | `103.76 ns` (🚀 **5.38x faster**)  | `11.49 ns` (🚀 **48.54x faster**) | `5.39 ns` (🚀 **103.53x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `298.57 us` (✅ **1.00x**)        | `885.25 us` (❌ *2.97x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `22.89 ns` (❌ *3.85x slower*)    | `109.45 ns` (❌ *18.41x slower*)   | `17.71 ns` (❌ *2.98x slower*)    | `5.94 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `226.00 ns` (❌ *5.83x slower*)   | `5.74 us` (❌ *148.15x slower*)    | `70.33 ns` (❌ *1.81x slower*)    | `38.77 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `173.59 ns` (❌ *4.89x slower*)   | `4.04 us` (❌ *113.74x slower*)    | `58.55 ns` (❌ *1.65x slower*)    | `35.51 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `13.76 us` (❌ *2.16x slower*)    | `22.95 us` (❌ *3.61x slower*)     | `13.46 us` (❌ *2.12x slower*)    | `6.35 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `492.69 ns` (❌ *6.06x slower*)   | `11.78 us` (❌ *144.95x slower*)   | `106.54 ns` (❌ *1.31x slower*)   | `81.24 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `477.00 ns` (❌ *5.89x slower*)   | `11.68 us` (❌ *144.30x slower*)   | `157.91 ns` (❌ *1.95x slower*)   | `80.93 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**)        | `7.83 ns` (❌ *1.20x slower*)    | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**)        | `10.62 ns` (❌ *1.36x slower*)   | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**)        | `4.04 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**)        | `3.74 ns` (✅ **1.00x faster**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381_optimized

|                                          | `g1projectivebls12_381`          | `g2projectivebls12_381`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                     | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `141.56 ns` (✅ **1.00x**)        | `191.88 ns` (❌ *1.36x slower*)   | `29.91 ns` (🚀 **4.73x faster**)    | `49.52 ns` (🚀 **2.86x faster**)    | `97.96 ns` (✅ **1.45x faster**)    | `631.27 ns` (❌ *4.46x slower*)    |
| **`serialize_uncompressed`**             | `179.60 ns` (✅ **1.00x**)        | `267.67 ns` (❌ *1.49x slower*)   | `29.81 ns` (🚀 **6.03x faster**)    | `49.52 ns` (🚀 **3.63x faster**)    | `97.95 ns` (🚀 **1.83x faster**)    | `630.99 ns` (❌ *3.51x slower*)    |
| **`deserialize_compressed`**             | `148.75 us` (✅ **1.00x**)        | `258.28 us` (❌ *1.74x slower*)   | `45.03 ns` (🚀 **3303.45x faster**) | `93.68 ns` (🚀 **1587.90x faster**) | `211.56 ns` (🚀 **703.12x faster**) | `1.26 us` (🚀 **117.76x faster**)  |
| **`deserialize_compressed_unchecked`**   | `36.15 us` (✅ **1.00x**)         | `122.66 us` (❌ *3.39x slower*)   | `45.02 ns` (🚀 **802.96x faster**)  | `93.71 ns` (🚀 **385.71x faster**)  | `211.61 ns` (🚀 **170.81x faster**) | `1.26 us` (🚀 **28.61x faster**)   |
| **`deserialize_uncompressed`**           | `111.95 us` (✅ **1.00x**)        | `135.27 us` (❌ *1.21x slower*)   | `44.73 ns` (🚀 **2502.89x faster**) | `93.51 ns` (🚀 **1197.13x faster**) | `211.42 ns` (🚀 **529.48x faster**) | `1.27 us` (🚀 **88.08x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `232.27 ns` (✅ **1.00x**)        | `491.77 ns` (❌ *2.12x slower*)   | `44.73 ns` (🚀 **5.19x faster**)    | `93.51 ns` (🚀 **2.48x faster**)    | `212.14 ns` (✅ **1.09x faster**)   | `1.27 us` (❌ *5.45x slower*)      |

### msm_for_bls12_381_optimized

|        | `g1projectivebls12_381`          | `g2projectivebls12_381`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.21 s` (✅ **1.00x**)           | `6.60 s` (❌ *2.99x slower*)       |

### squareroot_for_bls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `22.19 us` (✅ **1.00x**) | `35.75 us` (❌ *1.61x slower*)   | `121.76 us` (❌ *5.49x slower*)    |
| **`legendre_for_qr`**    | `12.35 us` (✅ **1.00x**) | `35.60 us` (❌ *2.88x slower*)   | `35.58 us` (❌ *2.88x slower*)     |

### bitwise_operations_for_bls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)        | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `60.91 ns` (✅ **1.00x**)       | `108.39 ns` (❌ *1.78x slower*)    |
| **`from_big-endian_bits`**    | `60.69 ns` (✅ **1.00x**)       | `108.61 ns` (❌ *1.79x slower*)    |
| **`comparison`**              | `4.07 ns` (✅ **1.00x**)        | `4.32 ns` (✅ **1.06x slower**)    |
| **`equality`**                | `4.48 ns` (✅ **1.00x**)        | `4.65 ns` (✅ **1.04x slower**)    |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)        | `4.00 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `36.07 ns` (✅ **1.00x**) | `78.97 ns` (❌ *2.19x slower*)    |
| **`into_bigint`** | `21.55 ns` (✅ **1.00x**) | `41.42 ns` (❌ *1.92x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

