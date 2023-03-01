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
|        | `221.96 us` (✅ **1.00x**)                 | `1.81 ms` (❌ *8.17x slower*)               |

### arithmetic_for_bls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_381`          | `g2projectivebls12_381`          | `fq2optimized`                   | `fq12optimized`                   | `fqoptimized`                    | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.21 us` (✅ **1.00x**)          | `3.86 us` (❌ *3.19x slower*)     | `23.12 ns` (🚀 **52.38x faster**) | `200.47 ns` (🚀 **6.04x faster**)  | `12.69 ns` (🚀 **95.44x faster**) | `8.67 ns` (🚀 **139.69x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.26 us` (✅ **1.00x**)          | `3.93 us` (❌ *3.12x slower*)     | `23.40 ns` (🚀 **53.72x faster**) | `159.76 ns` (🚀 **7.87x faster**)  | `12.82 ns` (🚀 **98.09x faster**) | `8.79 ns` (🚀 **142.99x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `875.53 ns` (✅ **1.00x**)        | `2.79 us` (❌ *3.19x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `903.52 ns` (✅ **1.00x**)        | `2.85 us` (❌ *3.15x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `581.64 ns` (✅ **1.00x**)        | `1.78 us` (❌ *3.05x slower*)     | `12.56 ns` (🚀 **46.31x faster**) | `69.12 ns` (🚀 **8.41x faster**)   | `7.27 ns` (🚀 **79.98x faster**)  | `5.90 ns` (🚀 **98.54x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `340.96 us` (✅ **1.00x**)        | `976.95 us` (❌ *2.87x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `23.05 ns` (❌ *3.73x slower*)    | `101.69 ns` (❌ *16.46x slower*)   | `18.27 ns` (❌ *2.96x slower*)    | `6.18 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `242.29 ns` (❌ *5.26x slower*)   | `6.22 us` (❌ *134.87x slower*)    | `76.62 ns` (❌ *1.66x slower*)    | `46.08 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `173.05 ns` (❌ *4.56x slower*)   | `4.36 us` (❌ *114.90x slower*)    | `65.27 ns` (❌ *1.72x slower*)    | `37.96 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `15.29 us` (❌ *2.15x slower*)    | `25.60 us` (❌ *3.61x slower*)     | `14.97 us` (❌ *2.11x slower*)    | `7.10 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `525.36 ns` (❌ *6.18x slower*)   | `12.77 us` (❌ *150.13x slower*)   | `118.99 ns` (❌ *1.40x slower*)   | `85.06 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `515.11 ns` (❌ *5.86x slower*)   | `12.72 us` (❌ *144.56x slower*)   | `164.61 ns` (❌ *1.87x slower*)   | `87.97 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.63 ns` (✅ **1.00x**)        | `8.67 ns` (❌ *1.14x slower*)    | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.69 ns` (✅ **1.00x**)        | `10.43 ns` (❌ *1.20x slower*)   | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.88 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.55 ns` (✅ **1.00x**)        | `4.54 ns` (✅ **1.00x faster**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381_optimized

|                                          | `g1projectivebls12_381`          | `g2projectivebls12_381`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                     | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `150.79 ns` (✅ **1.00x**)        | `204.38 ns` (❌ *1.36x slower*)   | `33.35 ns` (🚀 **4.52x faster**)    | `57.16 ns` (🚀 **2.64x faster**)    | `109.45 ns` (✅ **1.38x faster**)   | `702.88 ns` (❌ *4.66x slower*)    |
| **`serialize_uncompressed`**             | `191.81 ns` (✅ **1.00x**)        | `283.84 ns` (❌ *1.48x slower*)   | `31.56 ns` (🚀 **6.08x faster**)    | `55.53 ns` (🚀 **3.45x faster**)    | `109.37 ns` (✅ **1.75x faster**)   | `702.94 ns` (❌ *3.66x slower*)    |
| **`deserialize_compressed`**             | `166.86 us` (✅ **1.00x**)        | `284.17 us` (❌ *1.70x slower*)   | `53.28 ns` (🚀 **3131.91x faster**) | `95.35 ns` (🚀 **1749.90x faster**) | `216.63 ns` (🚀 **770.23x faster**) | `1.33 us` (🚀 **125.59x faster**)  |
| **`deserialize_compressed_unchecked`**   | `39.39 us` (✅ **1.00x**)         | `134.39 us` (❌ *3.41x slower*)   | `53.25 ns` (🚀 **739.69x faster**)  | `95.28 ns` (🚀 **413.40x faster**)  | `216.63 ns` (🚀 **181.83x faster**) | `1.33 us` (🚀 **29.59x faster**)   |
| **`deserialize_uncompressed`**           | `127.21 us` (✅ **1.00x**)        | `149.47 us` (❌ *1.17x slower*)   | `53.13 ns` (🚀 **2394.23x faster**) | `94.91 ns` (🚀 **1340.35x faster**) | `216.89 ns` (🚀 **586.52x faster**) | `1.33 us` (🚀 **95.74x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `220.11 ns` (✅ **1.00x**)        | `488.27 ns` (❌ *2.22x slower*)   | `53.15 ns` (🚀 **4.14x faster**)    | `94.87 ns` (🚀 **2.32x faster**)    | `217.18 ns` (✅ **1.01x faster**)   | `1.33 us` (❌ *6.04x slower*)      |

### msm_for_bls12_381_optimized

|        | `g1projectivebls12_381`          | `g2projectivebls12_381`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.31 s` (✅ **1.00x**)           | `7.06 s` (❌ *3.06x slower*)       |

### squareroot_for_bls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `25.53 us` (✅ **1.00x**) | `38.92 us` (❌ *1.52x slower*)   | `133.25 us` (❌ *5.22x slower*)    |
| **`legendre_for_qr`**    | `14.49 us` (✅ **1.00x**) | `38.64 us` (❌ *2.67x slower*)   | `38.48 us` (❌ *2.66x slower*)     |

### bitwise_operations_for_bls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `49.32 ns` (✅ **1.00x**)       | `88.75 ns` (❌ *1.80x slower*)    |
| **`from_big-endian_bits`**    | `49.29 ns` (✅ **1.00x**)       | `88.77 ns` (❌ *1.80x slower*)    |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)        | `5.13 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.43 ns` (✅ **1.00x**)        | `5.67 ns` (✅ **1.05x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `5.24 ns` (✅ **1.07x slower**)   |

### conversions_for_bls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.17 ns` (✅ **1.00x**) | `76.11 ns` (❌ *1.85x slower*)    |
| **`into_bigint`** | `22.34 ns` (✅ **1.00x**) | `48.04 ns` (❌ *2.15x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

