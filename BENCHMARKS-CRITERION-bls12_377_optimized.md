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
|        | `211.09 us` (✅ **1.00x**)                 | `2.04 ms` (❌ *9.68x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                   | `fq12optimized`                   | `fqoptimized`                     | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.25 us` (✅ **1.00x**)          | `4.57 us` (❌ *3.67x slower*)     | `23.11 ns` (🚀 **53.94x faster**) | `183.48 ns` (🚀 **6.79x faster**)  | `12.52 ns` (🚀 **99.54x faster**)  | `8.71 ns` (🚀 **143.14x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.30 us` (✅ **1.00x**)          | `4.63 us` (❌ *3.58x slower*)     | `23.28 ns` (🚀 **55.65x faster**) | `159.23 ns` (🚀 **8.14x faster**)  | `12.74 ns` (🚀 **101.73x faster**) | `8.80 ns` (🚀 **147.24x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `906.19 ns` (✅ **1.00x**)        | `3.31 us` (❌ *3.65x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `936.27 ns` (✅ **1.00x**)        | `3.36 us` (❌ *3.59x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `595.38 ns` (✅ **1.00x**)        | `2.25 us` (❌ *3.78x slower*)     | `12.34 ns` (🚀 **48.26x faster**) | `71.91 ns` (🚀 **8.28x faster**)   | `7.14 ns` (🚀 **83.41x faster**)   | `5.83 ns` (🚀 **102.13x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `341.13 us` (✅ **1.00x**)        | `1.17 ms` (❌ *3.44x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `23.08 ns` (❌ *3.74x slower*)    | `94.46 ns` (❌ *15.30x slower*)    | `18.24 ns` (❌ *2.95x slower*)     | `6.17 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `271.68 ns` (❌ *6.29x slower*)   | `7.10 us` (❌ *164.24x slower*)    | `75.32 ns` (❌ *1.74x slower*)     | `43.21 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `241.54 ns` (❌ *6.71x slower*)   | `5.02 us` (❌ *139.55x slower*)    | `66.78 ns` (❌ *1.86x slower*)     | `35.99 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `14.95 us` (❌ *2.12x slower*)    | `27.35 us` (❌ *3.87x slower*)     | `14.65 us` (❌ *2.07x slower*)     | `7.06 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `590.47 ns` (❌ *9.61x slower*)   | `14.54 us` (❌ *236.70x slower*)   | `117.88 ns` (❌ *1.92x slower*)    | `61.45 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `578.42 ns` (❌ *6.32x slower*)   | `14.47 us` (❌ *158.01x slower*)   | `162.45 ns` (❌ *1.77x slower*)    | `91.57 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**)        | `8.67 ns` (❌ *1.14x slower*)    | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.69 ns` (✅ **1.00x**)        | `10.32 ns` (❌ *1.19x slower*)   | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.88 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`division_by_2`**                   | `4.66 ns` (✅ **1.00x**)        | `4.66 ns` (✅ **1.00x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `159.68 ns` (✅ **1.00x**)        | `224.77 ns` (❌ *1.41x slower*)   | `30.98 ns` (🚀 **5.15x faster**)    | `56.40 ns` (🚀 **2.83x faster**)    | `111.36 ns` (✅ **1.43x faster**)    | `700.33 ns` (❌ *4.39x slower*)    |
| **`serialize_uncompressed`**             | `210.62 ns` (✅ **1.00x**)        | `344.15 ns` (❌ *1.63x slower*)   | `30.48 ns` (🚀 **6.91x faster**)    | `55.93 ns` (🚀 **3.77x faster**)    | `110.40 ns` (🚀 **1.91x faster**)    | `699.11 ns` (❌ *3.32x slower*)    |
| **`deserialize_compressed`**             | `315.92 us` (✅ **1.00x**)        | `1.06 ms` (❌ *3.35x slower*)     | `52.40 ns` (🚀 **6029.19x faster**) | `93.48 ns` (🚀 **3379.37x faster**) | `210.05 ns` (🚀 **1504.00x faster**) | `1.32 us` (🚀 **239.77x faster**)  |
| **`deserialize_compressed_unchecked`**   | `67.80 us` (✅ **1.00x**)         | `182.74 us` (❌ *2.70x slower*)   | `52.40 ns` (🚀 **1293.81x faster**) | `93.46 ns` (🚀 **725.44x faster**)  | `209.98 ns` (🚀 **322.89x faster**)  | `1.32 us` (🚀 **51.52x faster**)   |
| **`deserialize_uncompressed`**           | `248.31 us` (✅ **1.00x**)        | `874.46 us` (❌ *3.52x slower*)   | `52.34 ns` (🚀 **4743.98x faster**) | `93.56 ns` (🚀 **2654.08x faster**) | `209.38 ns` (🚀 **1185.91x faster**) | `1.32 us` (🚀 **188.59x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `223.70 ns` (✅ **1.00x**)        | `465.26 ns` (❌ *2.08x slower*)   | `52.37 ns` (🚀 **4.27x faster**)    | `93.59 ns` (🚀 **2.39x faster**)    | `209.50 ns` (✅ **1.07x faster**)    | `1.32 us` (❌ *5.89x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.36 s` (✅ **1.00x**)           | `8.38 s` (❌ *3.55x slower*)       |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.31 us` (✅ **1.00x**) | `67.26 us` (❌ *2.15x slower*)   | `181.73 us` (❌ *5.80x slower*)    |
| **`legendre_for_qr`**    | `10.95 us` (✅ **1.00x**) | `31.67 us` (❌ *2.89x slower*)   | `32.88 us` (❌ *3.00x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.84 ns` (✅ **1.00x**)        | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.94 ns` (✅ **1.00x**)       | `89.61 ns` (❌ *1.83x slower*)    |
| **`from_big-endian_bits`**    | `48.88 ns` (✅ **1.00x**)       | `88.83 ns` (❌ *1.82x slower*)    |
| **`comparison`**              | `4.87 ns` (✅ **1.00x**)        | `5.19 ns` (✅ **1.06x slower**)   |
| **`equality`**                | `5.42 ns` (✅ **1.00x**)        | `5.67 ns` (✅ **1.05x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.55 ns` (✅ **1.00x**) | `75.20 ns` (❌ *1.85x slower*)    |
| **`into_bigint`** | `22.94 ns` (✅ **1.00x**) | `46.85 ns` (❌ *2.04x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

