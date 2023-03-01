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
|        | `294.32 us` (✅ **1.00x**)                 | `2.48 ms` (❌ *8.44x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                    | `fq12optimized`                   | `fqoptimized`                    | `froptimized`                      |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.79 us` (✅ **1.00x**)          | `5.45 us` (❌ *3.04x slower*)     | `34.78 ns` (🚀 **51.46x faster**)  | `239.32 ns` (🚀 **7.48x faster**)  | `24.57 ns` (🚀 **72.84x faster**) | `11.34 ns` (🚀 **157.87x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.85 us` (✅ **1.00x**)          | `5.70 us` (❌ *3.09x slower*)     | `36.06 ns` (🚀 **51.19x faster**)  | `220.05 ns` (🚀 **8.39x faster**)  | `20.03 ns` (🚀 **92.17x faster**) | `11.82 ns` (🚀 **156.19x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `1.30 us` (✅ **1.00x**)          | `4.02 us` (❌ *3.10x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `1.34 us` (✅ **1.00x**)          | `3.94 us` (❌ *2.95x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`double`**                          | `N/A`                          | `N/A`                           | `887.89 ns` (✅ **1.00x**)        | `2.64 us` (❌ *2.97x slower*)     | `17.51 ns` (🚀 **50.71x faster**)  | `135.87 ns` (🚀 **6.53x faster**)  | `13.41 ns` (🚀 **66.19x faster**) | `11.12 ns` (🚀 **79.86x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `467.72 us` (✅ **1.00x**)        | `1.41 ms` (❌ *3.01x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `29.31 ns` (❌ *3.47x slower*)     | `176.71 ns` (❌ *20.90x slower*)   | `21.17 ns` (❌ *2.50x slower*)    | `8.46 ns` (✅ **1.00x**)            |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `341.57 ns` (❌ *7.11x slower*)    | `8.89 us` (❌ *185.02x slower*)    | `102.66 ns` (❌ *2.14x slower*)   | `48.02 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `382.70 ns` (❌ *9.72x slower*)    | `6.54 us` (❌ *166.18x slower*)    | `78.68 ns` (❌ *2.00x slower*)    | `39.37 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `17.16 us` (❌ *2.42x slower*)     | `33.36 us` (❌ *4.71x slower*)     | `16.14 us` (❌ *2.28x slower*)    | `7.09 us` (✅ **1.00x**)            |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `759.76 ns` (❌ *11.91x slower*)   | `18.12 us` (❌ *284.01x slower*)   | `142.01 ns` (❌ *2.23x slower*)   | `63.81 ns` (✅ **1.00x**)           |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `726.44 ns` (❌ *7.30x slower*)    | `18.06 us` (❌ *181.42x slower*)   | `233.08 ns` (❌ *2.34x slower*)   | `99.57 ns` (✅ **1.00x**)           |
| **`addition_with_carry`**             | `8.12 ns` (✅ **1.00x**)        | `23.07 ns` (❌ *2.84x slower*)   | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`subtraction_with_borrow`**         | `11.11 ns` (✅ **1.00x**)       | `14.39 ns` (❌ *1.30x slower*)   | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`multiplication_by_2`**             | `4.30 ns` (✅ **1.00x**)        | `4.82 ns` (❌ *1.12x slower*)    | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`division_by_2`**                   | `4.24 ns` (✅ **1.00x**)        | `4.39 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                       | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `190.66 ns` (✅ **1.00x**)        | `268.71 ns` (❌ *1.41x slower*)   | `37.15 ns` (🚀 **5.13x faster**)    | `62.24 ns` (🚀 **3.06x faster**)     | `126.58 ns` (✅ **1.51x faster**)    | `798.88 ns` (❌ *4.19x slower*)    |
| **`serialize_uncompressed`**             | `257.48 ns` (✅ **1.00x**)        | `397.98 ns` (❌ *1.55x slower*)   | `39.44 ns` (🚀 **6.53x faster**)    | `62.94 ns` (🚀 **4.09x faster**)     | `127.20 ns` (🚀 **2.02x faster**)    | `799.14 ns` (❌ *3.10x slower*)    |
| **`deserialize_compressed`**             | `431.95 us` (✅ **1.00x**)        | `1.28 ms` (❌ *2.97x slower*)     | `63.09 ns` (🚀 **6846.60x faster**) | `175.84 ns` (🚀 **2456.53x faster**) | `299.43 ns` (🚀 **1442.58x faster**) | `1.83 us` (🚀 **236.28x faster**)  |
| **`deserialize_compressed_unchecked`**   | `92.08 us` (✅ **1.00x**)         | `239.35 us` (❌ *2.60x slower*)   | `62.40 ns` (🚀 **1475.68x faster**) | `166.15 ns` (🚀 **554.22x faster**)  | `296.35 ns` (🚀 **310.72x faster**)  | `1.82 us` (🚀 **50.47x faster**)   |
| **`deserialize_uncompressed`**           | `342.81 us` (✅ **1.00x**)        | `1.05 ms` (❌ *3.05x slower*)     | `59.04 ns` (🚀 **5806.17x faster**) | `168.46 ns` (🚀 **2035.02x faster**) | `303.44 ns` (🚀 **1129.74x faster**) | `1.83 us` (🚀 **187.03x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `305.49 ns` (✅ **1.00x**)        | `648.53 ns` (❌ *2.12x slower*)   | `59.71 ns` (🚀 **5.12x faster**)    | `167.47 ns` (🚀 **1.82x faster**)    | `304.87 ns` (✅ **1.00x faster**)    | `1.80 us` (❌ *5.89x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `3.49 s` (✅ **1.00x**)           | `10.79 s` (❌ *3.09x slower*)      |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `35.32 us` (✅ **1.00x**) | `96.87 us` (❌ *2.74x slower*)   | `257.66 us` (❌ *7.29x slower*)    |
| **`legendre_for_qr`**    | `12.66 us` (✅ **1.00x**) | `45.22 us` (❌ *3.57x slower*)   | `46.65 us` (❌ *3.68x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.50 ns` (✅ **1.00x**)        | `4.81 ns` (✅ **1.07x slower**)    |
| **`from_little-endian_bits`** | `75.41 ns` (✅ **1.00x**)       | `131.58 ns` (❌ *1.75x slower*)    |
| **`from_big-endian_bits`**    | `74.55 ns` (✅ **1.00x**)       | `129.04 ns` (❌ *1.73x slower*)    |
| **`comparison`**              | `4.64 ns` (✅ **1.00x**)        | `4.90 ns` (✅ **1.05x slower**)    |
| **`equality`**                | `4.97 ns` (✅ **1.00x**)        | `5.88 ns` (❌ *1.18x slower*)      |
| **`is_zero`**                 | `4.38 ns` (✅ **1.00x**)        | `4.65 ns` (✅ **1.06x slower**)    |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `45.12 ns` (✅ **1.00x**) | `95.54 ns` (❌ *2.12x slower*)    |
| **`into_bigint`** | `27.20 ns` (✅ **1.00x**) | `51.75 ns` (❌ *1.90x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

