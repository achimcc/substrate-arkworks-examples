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
|        | `226.48 us` (✅ **1.00x**)                 | `2.04 ms` (❌ *9.03x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                   | `fq12optimized`                   | `fqoptimized`                     | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.27 us` (✅ **1.00x**)          | `4.58 us` (❌ *3.60x slower*)     | `23.16 ns` (🚀 **54.87x faster**) | `179.89 ns` (🚀 **7.06x faster**)  | `12.51 ns` (🚀 **101.58x faster**) | `8.71 ns` (🚀 **145.85x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.31 us` (✅ **1.00x**)          | `4.63 us` (❌ *3.53x slower*)     | `23.26 ns` (🚀 **56.37x faster**) | `158.01 ns` (🚀 **8.30x faster**)  | `12.75 ns` (🚀 **102.86x faster**) | `8.79 ns` (🚀 **149.14x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `907.46 ns` (✅ **1.00x**)        | `3.32 us` (❌ *3.66x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `937.74 ns` (✅ **1.00x**)        | `3.35 us` (❌ *3.57x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `593.36 ns` (✅ **1.00x**)        | `2.25 us` (❌ *3.79x slower*)     | `12.30 ns` (🚀 **48.24x faster**) | `70.94 ns` (🚀 **8.36x faster**)   | `7.14 ns` (🚀 **83.15x faster**)   | `5.84 ns` (🚀 **101.59x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `340.98 us` (✅ **1.00x**)        | `1.17 ms` (❌ *3.44x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `23.35 ns` (❌ *3.79x slower*)    | `92.91 ns` (❌ *15.09x slower*)    | `18.91 ns` (❌ *3.07x slower*)     | `6.16 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `271.66 ns` (❌ *6.20x slower*)   | `7.09 us` (❌ *161.76x slower*)    | `75.28 ns` (❌ *1.72x slower*)     | `43.85 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `242.05 ns` (❌ *6.70x slower*)   | `5.02 us` (❌ *138.87x slower*)    | `66.80 ns` (❌ *1.85x slower*)     | `36.12 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `15.00 us` (❌ *2.13x slower*)    | `27.32 us` (❌ *3.87x slower*)     | `14.64 us` (❌ *2.07x slower*)     | `7.06 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `590.48 ns` (❌ *9.62x slower*)   | `14.53 us` (❌ *236.91x slower*)   | `117.87 ns` (❌ *1.92x slower*)    | `61.35 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `577.94 ns` (❌ *6.31x slower*)   | `14.47 us` (❌ *158.04x slower*)   | `162.62 ns` (❌ *1.78x slower*)    | `91.59 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.62 ns` (✅ **1.00x**)        | `8.67 ns` (❌ *1.14x slower*)    | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.65 ns` (✅ **1.00x**)        | `10.32 ns` (❌ *1.19x slower*)   | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.88 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`division_by_2`**                   | `4.65 ns` (✅ **1.00x**)        | `4.62 ns` (✅ **1.01x faster**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `159.57 ns` (✅ **1.00x**)        | `222.62 ns` (❌ *1.40x slower*)   | `31.27 ns` (🚀 **5.10x faster**)    | `56.77 ns` (🚀 **2.81x faster**)    | `110.37 ns` (✅ **1.45x faster**)    | `699.02 ns` (❌ *4.38x slower*)    |
| **`serialize_uncompressed`**             | `209.38 ns` (✅ **1.00x**)        | `344.72 ns` (❌ *1.65x slower*)   | `30.58 ns` (🚀 **6.85x faster**)    | `55.94 ns` (🚀 **3.74x faster**)    | `110.59 ns` (🚀 **1.89x faster**)    | `699.46 ns` (❌ *3.34x slower*)    |
| **`deserialize_compressed`**             | `316.30 us` (✅ **1.00x**)        | `1.06 ms` (❌ *3.35x slower*)     | `52.43 ns` (🚀 **6032.89x faster**) | `92.96 ns` (🚀 **3402.41x faster**) | `210.47 ns` (🚀 **1502.85x faster**) | `1.32 us` (🚀 **239.72x faster**)  |
| **`deserialize_compressed_unchecked`**   | `67.84 us` (✅ **1.00x**)         | `182.76 us` (❌ *2.69x slower*)   | `52.42 ns` (🚀 **1294.04x faster**) | `93.02 ns` (🚀 **729.27x faster**)  | `210.48 ns` (🚀 **322.29x faster**)  | `1.32 us` (🚀 **51.36x faster**)   |
| **`deserialize_uncompressed`**           | `248.47 us` (✅ **1.00x**)        | `874.35 us` (❌ *3.52x slower*)   | `52.31 ns` (🚀 **4749.53x faster**) | `93.17 ns` (🚀 **2666.92x faster**) | `210.65 ns` (🚀 **1179.54x faster**) | `1.32 us` (🚀 **188.28x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `224.75 ns` (✅ **1.00x**)        | `466.55 ns` (❌ *2.08x slower*)   | `52.35 ns` (🚀 **4.29x faster**)    | `93.13 ns` (🚀 **2.41x faster**)    | `209.60 ns` (✅ **1.07x faster**)    | `1.32 us` (❌ *5.87x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.36 s` (✅ **1.00x**)           | `8.41 s` (❌ *3.56x slower*)       |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.32 us` (✅ **1.00x**) | `67.26 us` (❌ *2.15x slower*)   | `181.77 us` (❌ *5.80x slower*)    |
| **`legendre_for_qr`**    | `10.95 us` (✅ **1.00x**) | `31.68 us` (❌ *2.89x slower*)   | `32.89 us` (❌ *3.00x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.89 ns` (✅ **1.00x**)       | `89.22 ns` (❌ *1.82x slower*)    |
| **`from_big-endian_bits`**    | `48.89 ns` (✅ **1.00x**)       | `88.92 ns` (❌ *1.82x slower*)    |
| **`comparison`**              | `4.87 ns` (✅ **1.00x**)        | `5.13 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.43 ns` (✅ **1.00x**)        | `5.67 ns` (✅ **1.05x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.59 ns` (✅ **1.00x**) | `75.15 ns` (❌ *1.85x slower*)    |
| **`into_bigint`** | `22.93 ns` (✅ **1.00x**) | `46.88 ns` (❌ *2.04x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

