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
|        | `322.20 us` (✅ **1.00x**)                 | `2.22 ms` (❌ *6.90x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                    | `fq12optimized`                   | `fqoptimized`                    | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.16 us` (✅ **1.00x**)          | `4.28 us` (❌ *3.70x slower*)     | `27.94 ns` (🚀 **41.37x faster**)  | `179.48 ns` (🚀 **6.44x faster**)  | `19.04 ns` (🚀 **60.72x faster**) | `8.29 ns` (🚀 **139.40x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.19 us` (✅ **1.00x**)          | `4.31 us` (❌ *3.62x slower*)     | `27.66 ns` (🚀 **43.14x faster**)  | `172.50 ns` (🚀 **6.92x faster**)  | `17.12 ns` (🚀 **69.72x faster**) | `8.63 ns` (🚀 **138.30x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `835.30 ns` (✅ **1.00x**)        | `3.08 us` (❌ *3.68x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `861.06 ns` (✅ **1.00x**)        | `3.11 us` (❌ *3.61x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `569.73 ns` (✅ **1.00x**)        | `2.05 us` (❌ *3.59x slower*)     | `15.92 ns` (🚀 **35.79x faster**)  | `104.01 ns` (🚀 **5.48x faster**)  | `7.53 ns` (🚀 **75.66x faster**)  | `5.28 ns` (🚀 **107.86x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `429.20 us` (✅ **1.00x**)        | `1.43 ms` (❌ *3.34x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `22.88 ns` (❌ *3.85x slower*)     | `104.93 ns` (❌ *17.64x slower*)   | `16.77 ns` (❌ *2.82x slower*)    | `5.95 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `267.39 ns` (❌ *7.17x slower*)    | `5.90 us` (❌ *158.15x slower*)    | `70.36 ns` (❌ *1.89x slower*)    | `37.28 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `247.14 ns` (❌ *7.83x slower*)    | `4.71 us` (❌ *149.09x slower*)    | `58.93 ns` (❌ *1.87x slower*)    | `31.56 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `13.90 us` (❌ *2.18x slower*)     | `25.31 us` (❌ *3.98x slower*)     | `13.57 us` (❌ *2.13x slower*)    | `6.37 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `568.35 ns` (❌ *10.71x slower*)   | `13.61 us` (❌ *256.48x slower*)   | `111.62 ns` (❌ *2.10x slower*)   | `53.08 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `485.96 ns` (❌ *5.92x slower*)    | `13.58 us` (❌ *165.45x slower*)   | `156.51 ns` (❌ *1.91x slower*)   | `82.09 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.54 ns` (✅ **1.00x**)        | `7.84 ns` (❌ *1.20x slower*)    | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**)        | `10.63 ns` (❌ *1.36x slower*)   | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**)        | `4.04 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.75 ns` (✅ **1.00x**)        | `3.74 ns` (✅ **1.00x faster**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `151.07 ns` (✅ **1.00x**)        | `213.15 ns` (❌ *1.41x slower*)   | `27.83 ns` (🚀 **5.43x faster**)    | `50.29 ns` (🚀 **3.00x faster**)    | `99.36 ns` (✅ **1.52x faster**)     | `627.16 ns` (❌ *4.15x slower*)    |
| **`serialize_uncompressed`**             | `198.98 ns` (✅ **1.00x**)        | `320.57 ns` (❌ *1.61x slower*)   | `27.77 ns` (🚀 **7.16x faster**)    | `50.10 ns` (🚀 **3.97x faster**)    | `99.33 ns` (🚀 **2.00x faster**)     | `628.56 ns` (❌ *3.16x slower*)    |
| **`deserialize_compressed`**             | `346.23 us` (✅ **1.00x**)        | `1.15 ms` (❌ *3.31x slower*)     | `46.89 ns` (🚀 **7383.57x faster**) | `93.16 ns` (🚀 **3716.49x faster**) | `180.02 ns` (🚀 **1923.27x faster**) | `1.25 us` (🚀 **277.19x faster**)  |
| **`deserialize_compressed_unchecked`**   | `64.86 us` (✅ **1.00x**)         | `173.50 us` (❌ *2.67x slower*)   | `46.88 ns` (🚀 **1383.59x faster**) | `93.16 ns` (🚀 **696.24x faster**)  | `204.13 ns` (🚀 **317.75x faster**)  | `1.25 us` (🚀 **51.76x faster**)   |
| **`deserialize_uncompressed`**           | `281.88 us` (✅ **1.00x**)        | `972.02 us` (❌ *3.45x slower*)   | `46.86 ns` (🚀 **6015.88x faster**) | `94.39 ns` (🚀 **2986.30x faster**) | `203.99 ns` (🚀 **1381.80x faster**) | `1.25 us` (🚀 **225.65x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `239.10 ns` (✅ **1.00x**)        | `477.72 ns` (❌ *2.00x slower*)   | `46.88 ns` (🚀 **5.10x faster**)    | `94.51 ns` (🚀 **2.53x faster**)    | `203.98 ns` (✅ **1.17x faster**)    | `1.25 us` (❌ *5.23x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.31 s` (✅ **1.00x**)           | `7.98 s` (❌ *3.46x slower*)       |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `27.69 us` (✅ **1.00x**) | `64.71 us` (❌ *2.34x slower*)   | `172.94 us` (❌ *6.24x slower*)    |
| **`legendre_for_qr`**    | `9.58 us` (✅ **1.00x**)  | `29.17 us` (❌ *3.04x slower*)   | `29.30 us` (❌ *3.06x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)        | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `59.87 ns` (✅ **1.00x**)       | `108.81 ns` (❌ *1.82x slower*)    |
| **`from_big-endian_bits`**    | `59.98 ns` (✅ **1.00x**)       | `109.10 ns` (❌ *1.82x slower*)    |
| **`comparison`**              | `3.94 ns` (✅ **1.00x**)        | `4.21 ns` (✅ **1.07x slower**)    |
| **`equality`**                | `4.48 ns` (✅ **1.00x**)        | `4.71 ns` (✅ **1.05x slower**)    |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)        | `4.01 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.87 ns` (✅ **1.00x**) | `78.72 ns` (❌ *2.19x slower*)    |
| **`into_bigint`** | `22.03 ns` (✅ **1.00x**) | `41.51 ns` (❌ *1.88x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

