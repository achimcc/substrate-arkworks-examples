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
|        | `211.56 us` (✅ **1.00x**)                 | `2.05 ms` (❌ *9.69x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                   | `fq12optimized`                   | `fqoptimized`                     | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.25 us` (✅ **1.00x**)          | `4.51 us` (❌ *3.62x slower*)     | `22.96 ns` (🚀 **54.25x faster**) | `190.18 ns` (🚀 **6.55x faster**)  | `12.50 ns` (🚀 **99.65x faster**)  | `8.70 ns` (🚀 **143.19x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.29 us` (✅ **1.00x**)          | `4.57 us` (❌ *3.54x slower*)     | `23.34 ns` (🚀 **55.26x faster**) | `159.90 ns` (🚀 **8.07x faster**)  | `12.76 ns` (🚀 **101.11x faster**) | `8.80 ns` (🚀 **146.62x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `916.25 ns` (✅ **1.00x**)        | `3.27 us` (❌ *3.57x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `926.28 ns` (✅ **1.00x**)        | `3.30 us` (❌ *3.57x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `584.11 ns` (✅ **1.00x**)        | `2.22 us` (❌ *3.81x slower*)     | `12.38 ns` (🚀 **47.18x faster**) | `67.36 ns` (🚀 **8.67x faster**)   | `7.14 ns` (🚀 **81.81x faster**)   | `5.86 ns` (🚀 **99.62x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `339.37 us` (✅ **1.00x**)        | `1.18 ms` (❌ *3.47x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `23.56 ns` (❌ *3.83x slower*)    | `103.00 ns` (❌ *16.74x slower*)   | `18.52 ns` (❌ *3.01x slower*)     | `6.15 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `267.33 ns` (❌ *6.02x slower*)   | `7.09 us` (❌ *159.65x slower*)    | `76.41 ns` (❌ *1.72x slower*)     | `44.43 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `233.26 ns` (❌ *6.30x slower*)   | `5.00 us` (❌ *135.02x slower*)    | `66.62 ns` (❌ *1.80x slower*)     | `37.05 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `15.10 us` (❌ *2.13x slower*)    | `27.43 us` (❌ *3.88x slower*)     | `14.77 us` (❌ *2.09x slower*)     | `7.08 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `576.87 ns` (❌ *9.33x slower*)   | `14.52 us` (❌ *234.86x slower*)   | `117.96 ns` (❌ *1.91x slower*)    | `61.83 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `564.07 ns` (❌ *6.17x slower*)   | `14.43 us` (❌ *157.90x slower*)   | `162.75 ns` (❌ *1.78x slower*)    | `91.39 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.63 ns` (✅ **1.00x**)        | `8.65 ns` (❌ *1.13x slower*)    | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.65 ns` (✅ **1.00x**)        | `10.48 ns` (❌ *1.21x slower*)   | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.87 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`division_by_2`**                   | `4.75 ns` (✅ **1.00x**)        | `4.72 ns` (✅ **1.00x faster**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `159.76 ns` (✅ **1.00x**)        | `221.84 ns` (❌ *1.39x slower*)   | `31.07 ns` (🚀 **5.14x faster**)    | `56.81 ns` (🚀 **2.81x faster**)    | `110.08 ns` (✅ **1.45x faster**)    | `709.83 ns` (❌ *4.44x slower*)    |
| **`serialize_uncompressed`**             | `209.82 ns` (✅ **1.00x**)        | `347.04 ns` (❌ *1.65x slower*)   | `30.77 ns` (🚀 **6.82x faster**)    | `56.04 ns` (🚀 **3.74x faster**)    | `110.74 ns` (🚀 **1.89x faster**)    | `706.50 ns` (❌ *3.37x slower*)    |
| **`deserialize_compressed`**             | `315.95 us` (✅ **1.00x**)        | `1.06 ms` (❌ *3.36x slower*)     | `51.98 ns` (🚀 **6078.22x faster**) | `92.67 ns` (🚀 **3409.44x faster**) | `212.69 ns` (🚀 **1485.47x faster**) | `1.33 us` (🚀 **238.32x faster**)  |
| **`deserialize_compressed_unchecked`**   | `67.99 us` (✅ **1.00x**)         | `183.72 us` (❌ *2.70x slower*)   | `51.97 ns` (🚀 **1308.11x faster**) | `93.29 ns` (🚀 **728.82x faster**)  | `212.68 ns` (🚀 **319.68x faster**)  | `1.33 us` (🚀 **51.28x faster**)   |
| **`deserialize_uncompressed`**           | `248.03 us` (✅ **1.00x**)        | `875.70 us` (❌ *3.53x slower*)   | `51.91 ns` (🚀 **4777.85x faster**) | `93.40 ns` (🚀 **2655.50x faster**) | `212.46 ns` (🚀 **1167.42x faster**) | `1.33 us` (🚀 **187.12x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `230.47 ns` (✅ **1.00x**)        | `466.94 ns` (❌ *2.03x slower*)   | `51.68 ns` (🚀 **4.46x faster**)    | `93.42 ns` (🚀 **2.47x faster**)    | `212.52 ns` (✅ **1.08x faster**)    | `1.33 us` (❌ *5.78x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.36 s` (✅ **1.00x**)           | `8.23 s` (❌ *3.48x slower*)       |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.11 us` (✅ **1.00x**) | `67.55 us` (❌ *2.17x slower*)   | `182.75 us` (❌ *5.87x slower*)    |
| **`legendre_for_qr`**    | `10.94 us` (✅ **1.00x**) | `31.68 us` (❌ *2.90x slower*)   | `31.55 us` (❌ *2.88x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `50.13 ns` (✅ **1.00x**)       | `88.71 ns` (❌ *1.77x slower*)    |
| **`from_big-endian_bits`**    | `50.03 ns` (✅ **1.00x**)       | `88.71 ns` (❌ *1.77x slower*)    |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)        | `5.12 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.42 ns` (✅ **1.00x**)        | `5.74 ns` (✅ **1.06x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.76 ns` (✅ **1.00x**) | `75.39 ns` (❌ *1.85x slower*)    |
| **`into_bigint`** | `23.74 ns` (✅ **1.00x**) | `46.88 ns` (❌ *1.97x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

