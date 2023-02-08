# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [sample_bls12_377_optimized](#sample_bls12_377_optimized)
    - [arithmetic_for_bls12_377_optimized](#arithmetic_for_bls12_377_optimized)
    - [serialization_for_bls12_377_optimized](#serialization_for_bls12_377_optimized)
    - [msm_for_bls12_377_optimized](#msm_for_bls12_377_optimized)
    - [squareroot_for_bls12_377_optimized](#squareroot_for_bls12_377_optimized)
    - [bitwise_operations_for_bls12_377_optimized](#bitwise_operations_for_bls12_377_optimized)
    - [conversions_for_bls12_377_optimized](#conversions_for_bls12_377_optimized)

## Benchmark Results

### sample_bls12_377_optimized

|        | `g1projectivebls12_377_elements`          | `g2projectivebls12_377_elements`           |
|:-------|:------------------------------------------|:------------------------------------------ |
|        | `210.06 us` (✅ **1.00x**)                 | `2.05 ms` (❌ *9.74x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                   | `fq12optimized`                   | `fqoptimized`                     | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.24 us` (✅ **1.00x**)          | `4.59 us` (❌ *3.71x slower*)     | `23.06 ns` (🚀 **53.76x faster**) | `193.26 ns` (🚀 **6.41x faster**)  | `12.48 ns` (🚀 **99.32x faster**)  | `8.73 ns` (🚀 **142.06x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.29 us` (✅ **1.00x**)          | `4.62 us` (❌ *3.59x slower*)     | `23.25 ns` (🚀 **55.31x faster**) | `159.51 ns` (🚀 **8.06x faster**)  | `12.74 ns` (🚀 **100.94x faster**) | `8.82 ns` (🚀 **145.86x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `902.42 ns` (✅ **1.00x**)        | `3.31 us` (❌ *3.67x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `934.87 ns` (✅ **1.00x**)        | `3.35 us` (❌ *3.58x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `608.36 ns` (✅ **1.00x**)        | `2.26 us` (❌ *3.71x slower*)     | `12.33 ns` (🚀 **49.34x faster**) | `67.31 ns` (🚀 **9.04x faster**)   | `7.15 ns` (🚀 **85.14x faster**)   | `5.92 ns` (🚀 **102.79x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `340.84 us` (✅ **1.00x**)        | `1.18 ms` (❌ *3.45x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `22.52 ns` (❌ *3.65x slower*)    | `94.79 ns` (❌ *15.38x slower*)    | `18.28 ns` (❌ *2.97x slower*)     | `6.16 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `267.46 ns` (❌ *6.19x slower*)   | `7.18 us` (❌ *166.25x slower*)    | `76.12 ns` (❌ *1.76x slower*)     | `43.18 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `239.49 ns` (❌ *6.63x slower*)   | `5.05 us` (❌ *139.63x slower*)    | `66.48 ns` (❌ *1.84x slower*)     | `36.14 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `15.10 us` (❌ *2.14x slower*)    | `27.54 us` (❌ *3.90x slower*)     | `14.75 us` (❌ *2.09x slower*)     | `7.05 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `576.96 ns` (❌ *9.40x slower*)   | `14.63 us` (❌ *238.28x slower*)   | `117.66 ns` (❌ *1.92x slower*)    | `61.40 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `565.70 ns` (❌ *6.18x slower*)   | `14.57 us` (❌ *159.13x slower*)   | `162.55 ns` (❌ *1.78x slower*)    | `91.54 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.62 ns` (✅ **1.00x**)        | `8.65 ns` (❌ *1.14x slower*)    | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.65 ns` (✅ **1.00x**)        | `10.51 ns` (❌ *1.22x slower*)   | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.87 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`division_by_2`**                   | `4.52 ns` (✅ **1.00x**)        | `4.53 ns` (✅ **1.00x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `158.81 ns` (✅ **1.00x**)        | `220.41 ns` (❌ *1.39x slower*)   | `30.50 ns` (🚀 **5.21x faster**)    | `55.90 ns` (🚀 **2.84x faster**)    | `110.10 ns` (✅ **1.44x faster**)    | `709.31 ns` (❌ *4.47x slower*)    |
| **`serialize_uncompressed`**             | `208.17 ns` (✅ **1.00x**)        | `338.04 ns` (❌ *1.62x slower*)   | `32.63 ns` (🚀 **6.38x faster**)    | `55.70 ns` (🚀 **3.74x faster**)    | `109.70 ns` (🚀 **1.90x faster**)    | `705.87 ns` (❌ *3.39x slower*)    |
| **`deserialize_compressed`**             | `314.34 us` (✅ **1.00x**)        | `1.06 ms` (❌ *3.37x slower*)     | `51.92 ns` (🚀 **6054.23x faster**) | `94.08 ns` (🚀 **3341.06x faster**) | `216.08 ns` (🚀 **1454.75x faster**) | `1.27 us` (🚀 **247.06x faster**)  |
| **`deserialize_compressed_unchecked`**   | `67.41 us` (✅ **1.00x**)         | `182.30 us` (❌ *2.70x slower*)   | `51.94 ns` (🚀 **1297.79x faster**) | `94.13 ns` (🚀 **716.14x faster**)  | `215.88 ns` (🚀 **312.26x faster**)  | `1.27 us` (🚀 **53.02x faster**)   |
| **`deserialize_uncompressed`**           | `247.06 us` (✅ **1.00x**)        | `875.38 us` (❌ *3.54x slower*)   | `51.88 ns` (🚀 **4762.05x faster**) | `94.21 ns` (🚀 **2622.48x faster**) | `215.87 ns` (🚀 **1144.46x faster**) | `1.27 us` (🚀 **194.29x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `222.93 ns` (✅ **1.00x**)        | `465.27 ns` (❌ *2.09x slower*)   | `52.20 ns` (🚀 **4.27x faster**)    | `94.26 ns` (🚀 **2.37x faster**)    | `215.82 ns` (✅ **1.03x faster**)    | `1.27 us` (❌ *5.71x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.36 s` (✅ **1.00x**)           | `8.32 s` (❌ *3.52x slower*)       |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.15 us` (✅ **1.00x**) | `66.92 us` (❌ *2.15x slower*)   | `181.37 us` (❌ *5.82x slower*)    |
| **`legendre_for_qr`**    | `10.95 us` (✅ **1.00x**) | `31.63 us` (❌ *2.89x slower*)   | `31.53 us` (❌ *2.88x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `49.22 ns` (✅ **1.00x**)       | `86.11 ns` (❌ *1.75x slower*)    |
| **`from_big-endian_bits`**    | `49.16 ns` (✅ **1.00x**)       | `85.94 ns` (❌ *1.75x slower*)    |
| **`comparison`**              | `4.88 ns` (✅ **1.00x**)        | `5.12 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.37 ns` (✅ **1.00x**)        | `5.67 ns` (✅ **1.06x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.60 ns` (✅ **1.00x**) | `74.97 ns` (❌ *1.85x slower*)    |
| **`into_bigint`** | `23.77 ns` (✅ **1.00x**) | `46.91 ns` (❌ *1.97x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

