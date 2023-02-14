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
|        | `211.22 us` (✅ **1.00x**)                 | `2.05 ms` (❌ *9.70x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                   | `fq12optimized`                   | `fqoptimized`                     | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.25 us` (✅ **1.00x**)          | `4.58 us` (❌ *3.67x slower*)     | `23.05 ns` (🚀 **54.20x faster**) | `176.12 ns` (🚀 **7.09x faster**)  | `12.51 ns` (🚀 **99.82x faster**)  | `8.71 ns` (🚀 **143.43x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.30 us` (✅ **1.00x**)          | `4.67 us` (❌ *3.60x slower*)     | `23.70 ns` (🚀 **54.76x faster**) | `160.12 ns` (🚀 **8.11x faster**)  | `12.76 ns` (🚀 **101.72x faster**) | `8.79 ns` (🚀 **147.59x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `907.09 ns` (✅ **1.00x**)        | `3.32 us` (❌ *3.66x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `937.50 ns` (✅ **1.00x**)        | `3.38 us` (❌ *3.60x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `595.39 ns` (✅ **1.00x**)        | `2.25 us` (❌ *3.79x slower*)     | `12.31 ns` (🚀 **48.38x faster**) | `72.40 ns` (🚀 **8.22x faster**)   | `7.40 ns` (🚀 **80.51x faster**)   | `6.10 ns` (🚀 **97.54x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `341.39 us` (✅ **1.00x**)        | `1.18 ms` (❌ *3.45x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `23.03 ns` (❌ *3.74x slower*)    | `93.31 ns` (❌ *15.14x slower*)    | `18.45 ns` (❌ *2.99x slower*)     | `6.16 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `275.63 ns` (❌ *6.33x slower*)   | `7.10 us` (❌ *163.10x slower*)    | `75.25 ns` (❌ *1.73x slower*)     | `43.52 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `242.05 ns` (❌ *6.68x slower*)   | `5.02 us` (❌ *138.54x slower*)    | `66.80 ns` (❌ *1.84x slower*)     | `36.22 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `14.95 us` (❌ *2.12x slower*)    | `27.37 us` (❌ *3.87x slower*)     | `14.66 us` (❌ *2.08x slower*)     | `7.06 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `590.21 ns` (❌ *9.46x slower*)   | `14.64 us` (❌ *234.62x slower*)   | `118.05 ns` (❌ *1.89x slower*)    | `62.39 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `577.96 ns` (❌ *6.30x slower*)   | `14.56 us` (❌ *158.72x slower*)   | `163.90 ns` (❌ *1.79x slower*)    | `91.72 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.65 ns` (✅ **1.00x**)        | `8.68 ns` (❌ *1.13x slower*)    | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.68 ns` (✅ **1.00x**)        | `10.32 ns` (❌ *1.19x slower*)   | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.88 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`division_by_2`**                   | `4.60 ns` (✅ **1.00x**)        | `4.63 ns` (✅ **1.01x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `160.18 ns` (✅ **1.00x**)        | `222.78 ns` (❌ *1.39x slower*)   | `30.65 ns` (🚀 **5.23x faster**)    | `56.38 ns` (🚀 **2.84x faster**)    | `110.77 ns` (✅ **1.45x faster**)    | `702.34 ns` (❌ *4.38x slower*)    |
| **`serialize_uncompressed`**             | `211.11 ns` (✅ **1.00x**)        | `344.38 ns` (❌ *1.63x slower*)   | `30.30 ns` (🚀 **6.97x faster**)    | `56.00 ns` (🚀 **3.77x faster**)    | `111.03 ns` (🚀 **1.90x faster**)    | `706.11 ns` (❌ *3.34x slower*)    |
| **`deserialize_compressed`**             | `316.61 us` (✅ **1.00x**)        | `1.06 ms` (❌ *3.35x slower*)     | `52.42 ns` (🚀 **6040.09x faster**) | `93.57 ns` (🚀 **3383.56x faster**) | `210.17 ns` (🚀 **1506.49x faster**) | `1.32 us` (🚀 **239.91x faster**)  |
| **`deserialize_compressed_unchecked`**   | `67.86 us` (✅ **1.00x**)         | `182.92 us` (❌ *2.70x slower*)   | `52.42 ns` (🚀 **1294.62x faster**) | `93.52 ns` (🚀 **725.63x faster**)  | `210.18 ns` (🚀 **322.87x faster**)  | `1.31 us` (🚀 **51.69x faster**)   |
| **`deserialize_uncompressed`**           | `248.66 us` (✅ **1.00x**)        | `875.69 us` (❌ *3.52x slower*)   | `52.34 ns` (🚀 **4750.95x faster**) | `93.66 ns` (🚀 **2655.04x faster**) | `209.49 ns` (🚀 **1186.98x faster**) | `1.31 us` (🚀 **190.07x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `223.89 ns` (✅ **1.00x**)        | `466.37 ns` (❌ *2.08x slower*)   | `52.33 ns` (🚀 **4.28x faster**)    | `94.29 ns` (🚀 **2.37x faster**)    | `209.50 ns` (✅ **1.07x faster**)    | `1.31 us` (❌ *5.86x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.36 s` (✅ **1.00x**)           | `8.47 s` (❌ *3.58x slower*)       |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.43 us` (✅ **1.00x**) | `67.35 us` (❌ *2.14x slower*)   | `181.75 us` (❌ *5.78x slower*)    |
| **`legendre_for_qr`**    | `11.00 us` (✅ **1.00x**) | `31.66 us` (❌ *2.88x slower*)   | `33.09 us` (❌ *3.01x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `49.02 ns` (✅ **1.00x**)       | `90.21 ns` (❌ *1.84x slower*)    |
| **`from_big-endian_bits`**    | `49.36 ns` (✅ **1.00x**)       | `89.01 ns` (❌ *1.80x slower*)    |
| **`comparison`**              | `4.87 ns` (✅ **1.00x**)        | `5.77 ns` (❌ *1.18x slower*)     |
| **`equality`**                | `5.46 ns` (✅ **1.00x**)        | `5.67 ns` (✅ **1.04x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.62 ns` (✅ **1.00x**) | `74.99 ns` (❌ *1.85x slower*)    |
| **`into_bigint`** | `22.94 ns` (✅ **1.00x**) | `46.87 ns` (❌ *2.04x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

