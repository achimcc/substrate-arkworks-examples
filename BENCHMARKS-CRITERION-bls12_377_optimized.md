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
|        | `193.45 us` (✅ **1.00x**)                 | `1.87 ms` (❌ *9.69x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                    | `fq12optimized`                   | `fqoptimized`                    | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.13 us` (✅ **1.00x**)          | `4.28 us` (❌ *3.78x slower*)     | `26.74 ns` (🚀 **42.39x faster**)  | `176.57 ns` (🚀 **6.42x faster**)  | `19.03 ns` (🚀 **59.56x faster**) | `8.28 ns` (🚀 **136.84x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.17 us` (✅ **1.00x**)          | `4.31 us` (❌ *3.70x slower*)     | `27.27 ns` (🚀 **42.76x faster**)  | `171.55 ns` (🚀 **6.80x faster**)  | `14.76 ns` (🚀 **79.03x faster**) | `8.59 ns` (🚀 **135.69x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `825.54 ns` (✅ **1.00x**)        | `3.09 us` (❌ *3.74x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `856.58 ns` (✅ **1.00x**)        | `3.12 us` (❌ *3.64x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `572.38 ns` (✅ **1.00x**)        | `2.05 us` (❌ *3.58x slower*)     | `12.81 ns` (🚀 **44.67x faster**)  | `100.53 ns` (🚀 **5.69x faster**)  | `11.09 ns` (🚀 **51.61x faster**) | `5.30 ns` (🚀 **108.06x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `300.33 us` (✅ **1.00x**)        | `1.08 ms` (❌ *3.59x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `22.83 ns` (❌ *3.84x slower*)     | `105.42 ns` (❌ *17.73x slower*)   | `17.04 ns` (❌ *2.87x slower*)    | `5.95 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `267.70 ns` (❌ *7.18x slower*)    | `6.66 us` (❌ *178.70x slower*)    | `69.37 ns` (❌ *1.86x slower*)    | `37.28 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `250.32 ns` (❌ *7.87x slower*)    | `4.68 us` (❌ *147.21x slower*)    | `59.13 ns` (❌ *1.86x slower*)    | `31.82 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `13.72 us` (❌ *2.17x slower*)     | `25.03 us` (❌ *3.96x slower*)     | `13.41 us` (❌ *2.12x slower*)    | `6.31 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `570.11 ns` (❌ *10.75x slower*)   | `13.59 us` (❌ *256.37x slower*)   | `111.81 ns` (❌ *2.11x slower*)   | `53.01 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `553.38 ns` (❌ *6.73x slower*)    | `13.52 us` (❌ *164.38x slower*)   | `156.56 ns` (❌ *1.90x slower*)   | `82.23 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.53 ns` (✅ **1.00x**)        | `7.83 ns` (❌ *1.20x slower*)    | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**)        | `10.63 ns` (❌ *1.36x slower*)   | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**)        | `4.04 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**)        | `3.74 ns` (✅ **1.00x slower**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `150.52 ns` (✅ **1.00x**)        | `211.79 ns` (❌ *1.41x slower*)   | `27.89 ns` (🚀 **5.40x faster**)    | `50.30 ns` (🚀 **2.99x faster**)    | `100.16 ns` (✅ **1.50x faster**)    | `626.44 ns` (❌ *4.16x slower*)    |
| **`serialize_uncompressed`**             | `198.55 ns` (✅ **1.00x**)        | `326.70 ns` (❌ *1.65x slower*)   | `27.81 ns` (🚀 **7.14x faster**)    | `50.16 ns` (🚀 **3.96x faster**)    | `100.15 ns` (🚀 **1.98x faster**)    | `626.41 ns` (❌ *3.15x slower*)    |
| **`deserialize_compressed`**             | `282.31 us` (✅ **1.00x**)        | `973.50 us` (❌ *3.45x slower*)   | `46.64 ns` (🚀 **6052.39x faster**) | `93.69 ns` (🚀 **3013.35x faster**) | `207.74 ns` (🚀 **1358.95x faster**) | `1.25 us` (🚀 **225.03x faster**)  |
| **`deserialize_compressed_unchecked`**   | `65.12 us` (✅ **1.00x**)         | `173.58 us` (❌ *2.67x slower*)   | `46.61 ns` (🚀 **1397.12x faster**) | `93.28 ns` (🚀 **698.06x faster**)  | `207.65 ns` (🚀 **313.59x faster**)  | `1.26 us` (🚀 **51.88x faster**)   |
| **`deserialize_uncompressed`**           | `217.46 us` (✅ **1.00x**)        | `796.24 us` (❌ *3.66x slower*)   | `46.50 ns` (🚀 **4676.23x faster**) | `93.22 ns` (🚀 **2332.73x faster**) | `206.60 ns` (🚀 **1052.54x faster**) | `1.25 us` (🚀 **173.31x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `253.05 ns` (✅ **1.00x**)        | `468.18 ns` (❌ *1.85x slower*)   | `46.52 ns` (🚀 **5.44x faster**)    | `93.23 ns` (🚀 **2.71x faster**)    | `206.70 ns` (✅ **1.22x faster**)    | `1.26 us` (❌ *4.97x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.25 s` (✅ **1.00x**)           | `7.99 s` (❌ *3.55x slower*)       |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `27.74 us` (✅ **1.00x**) | `64.92 us` (❌ *2.34x slower*)   | `172.85 us` (❌ *6.23x slower*)    |
| **`legendre_for_qr`**    | `9.58 us` (✅ **1.00x**)  | `29.14 us` (❌ *3.04x slower*)   | `29.58 us` (❌ *3.09x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)        | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `60.80 ns` (✅ **1.00x**)       | `108.75 ns` (❌ *1.79x slower*)    |
| **`from_big-endian_bits`**    | `60.98 ns` (✅ **1.00x**)       | `108.72 ns` (❌ *1.78x slower*)    |
| **`comparison`**              | `4.12 ns` (✅ **1.00x**)        | `4.32 ns` (✅ **1.05x slower**)    |
| **`equality`**                | `4.48 ns` (✅ **1.00x**)        | `4.66 ns` (✅ **1.04x slower**)    |
| **`is_zero`**                 | `3.91 ns` (✅ **1.00x**)        | `4.01 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `36.02 ns` (✅ **1.00x**) | `79.25 ns` (❌ *2.20x slower*)    |
| **`into_bigint`** | `21.65 ns` (✅ **1.00x**) | `41.53 ns` (❌ *1.92x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

