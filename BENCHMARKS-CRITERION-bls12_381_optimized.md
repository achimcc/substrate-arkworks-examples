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
|        | `305.06 us` (✅ **1.00x**)                 | `2.34 ms` (❌ *7.67x slower*)               |

### arithmetic_for_bls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_381`          | `g2projectivebls12_381`          | `fq2optimized`                   | `fq12optimized`                   | `fqoptimized`                    | `froptimized`                      |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.72 us` (✅ **1.00x**)          | `4.98 us` (❌ *2.90x slower*)     | `36.19 ns` (🚀 **47.41x faster**) | `225.83 ns` (🚀 **7.60x faster**)  | `24.73 ns` (🚀 **69.38x faster**) | `10.44 ns` (🚀 **164.24x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.80 us` (✅ **1.00x**)          | `4.67 us` (❌ *2.60x slower*)     | `35.39 ns` (🚀 **50.78x faster**) | `215.89 ns` (🚀 **8.33x faster**)  | `19.51 ns` (🚀 **92.13x faster**) | `11.26 ns` (🚀 **159.56x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `1.24 us` (✅ **1.00x**)          | `3.32 us` (❌ *2.67x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `1.28 us` (✅ **1.00x**)          | `3.37 us` (❌ *2.62x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`double`**                          | `N/A`                          | `N/A`                           | `825.72 ns` (✅ **1.00x**)        | `2.32 us` (❌ *2.81x slower*)     | `16.27 ns` (🚀 **50.75x faster**) | `130.91 ns` (🚀 **6.31x faster**)  | `14.22 ns` (🚀 **58.07x faster**) | `6.57 ns` (🚀 **125.64x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `466.62 us` (✅ **1.00x**)        | `1.15 ms` (❌ *2.47x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `26.46 ns` (❌ *3.29x slower*)    | `149.51 ns` (❌ *18.57x slower*)   | `21.74 ns` (❌ *2.70x slower*)    | `8.05 ns` (✅ **1.00x**)            |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `276.76 ns` (❌ *5.55x slower*)   | `7.26 us` (❌ *145.69x slower*)    | `88.23 ns` (❌ *1.77x slower*)    | `49.83 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `244.15 ns` (❌ *5.65x slower*)   | `5.32 us` (❌ *122.97x slower*)    | `75.83 ns` (❌ *1.75x slower*)    | `43.23 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `16.37 us` (❌ *2.33x slower*)    | `29.43 us` (❌ *4.19x slower*)     | `16.07 us` (❌ *2.29x slower*)    | `7.02 us` (✅ **1.00x**)            |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `608.19 ns` (❌ *5.88x slower*)   | `14.93 us` (❌ *144.25x slower*)   | `128.35 ns` (❌ *1.24x slower*)   | `103.47 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `599.44 ns` (❌ *5.80x slower*)   | `14.95 us` (❌ *144.67x slower*)   | `213.17 ns` (❌ *2.06x slower*)   | `103.34 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.99 ns` (✅ **1.00x**)        | `22.66 ns` (❌ *2.83x slower*)   | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.49 ns` (✅ **1.00x**)       | `14.80 ns` (❌ *1.41x slower*)   | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`multiplication_by_2`**             | `4.51 ns` (✅ **1.00x**)        | `4.71 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`division_by_2`**                   | `4.33 ns` (✅ **1.00x**)        | `4.12 ns` (✅ **1.05x faster**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |

### serialization_for_bls12_381_optimized

|                                          | `g1projectivebls12_381`          | `g2projectivebls12_381`          | `froptimized`                      | `fqoptimized`                       | `fq2optimized`                     | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:------------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `182.25 ns` (✅ **1.00x**)        | `251.15 ns` (❌ *1.38x slower*)   | `37.92 ns` (🚀 **4.81x faster**)    | `60.93 ns` (🚀 **2.99x faster**)     | `120.93 ns` (✅ **1.51x faster**)   | `766.85 ns` (❌ *4.21x slower*)    |
| **`serialize_uncompressed`**             | `239.51 ns` (✅ **1.00x**)        | `344.70 ns` (❌ *1.44x slower*)   | `38.27 ns` (🚀 **6.26x faster**)    | `60.57 ns` (🚀 **3.95x faster**)     | `118.31 ns` (🚀 **2.02x faster**)   | `770.44 ns` (❌ *3.22x slower*)    |
| **`deserialize_compressed`**             | `206.45 us` (✅ **1.00x**)        | `377.32 us` (❌ *1.83x slower*)   | `60.53 ns` (🚀 **3410.87x faster**) | `122.83 ns` (🚀 **1680.76x faster**) | `291.60 ns` (🚀 **707.99x faster**) | `1.76 us` (🚀 **117.55x faster**)  |
| **`deserialize_compressed_unchecked`**   | `57.24 us` (✅ **1.00x**)         | `185.54 us` (❌ *3.24x slower*)   | `59.66 ns` (🚀 **959.42x faster**)  | `125.76 ns` (🚀 **455.15x faster**)  | `294.18 ns` (🚀 **194.56x faster**) | `1.76 us` (🚀 **32.47x faster**)   |
| **`deserialize_uncompressed`**           | `163.85 us` (✅ **1.00x**)        | `181.78 us` (✅ **1.11x slower**) | `61.40 ns` (🚀 **2668.62x faster**) | `125.79 ns` (🚀 **1302.52x faster**) | `295.28 ns` (🚀 **554.89x faster**) | `1.77 us` (🚀 **92.67x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `329.22 ns` (✅ **1.00x**)        | `701.23 ns` (❌ *2.13x slower*)   | `62.75 ns` (🚀 **5.25x faster**)    | `121.87 ns` (🚀 **2.70x faster**)    | `298.73 ns` (✅ **1.10x faster**)   | `1.77 us` (❌ *5.39x slower*)      |

### msm_for_bls12_381_optimized

|        | `g1projectivebls12_381`          | `g2projectivebls12_381`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `3.38 s` (✅ **1.00x**)           | `9.03 s` (❌ *2.67x slower*)       |

### squareroot_for_bls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `28.27 us` (✅ **1.00x**) | `55.50 us` (❌ *1.96x slower*)   | `191.44 us` (❌ *6.77x slower*)    |
| **`legendre_for_qr`**    | `15.92 us` (✅ **1.00x**) | `58.17 us` (❌ *3.65x slower*)   | `56.55 us` (❌ *3.55x slower*)     |

### bitwise_operations_for_bls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.48 ns` (✅ **1.00x**)        | `4.65 ns` (✅ **1.04x slower**)    |
| **`from_little-endian_bits`** | `72.83 ns` (✅ **1.00x**)       | `125.38 ns` (❌ *1.72x slower*)    |
| **`from_big-endian_bits`**    | `74.64 ns` (✅ **1.00x**)       | `140.20 ns` (❌ *1.88x slower*)    |
| **`comparison`**              | `4.59 ns` (✅ **1.00x**)        | `5.37 ns` (❌ *1.17x slower*)      |
| **`equality`**                | `4.92 ns` (✅ **1.00x**)        | `6.19 ns` (❌ *1.26x slower*)      |
| **`is_zero`**                 | `4.32 ns` (✅ **1.00x**)        | `4.82 ns` (❌ *1.12x slower*)      |

### conversions_for_bls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `44.74 ns` (✅ **1.00x**) | `97.31 ns` (❌ *2.17x slower*)    |
| **`into_bigint`** | `27.50 ns` (✅ **1.00x**) | `49.59 ns` (❌ *1.80x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

