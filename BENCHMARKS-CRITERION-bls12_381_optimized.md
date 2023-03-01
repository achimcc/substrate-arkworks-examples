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
|        | `222.04 us` (✅ **1.00x**)                 | `1.81 ms` (❌ *8.16x slower*)               |

### arithmetic_for_bls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_381`          | `g2projectivebls12_381`          | `fq2optimized`                   | `fq12optimized`                   | `fqoptimized`                    | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.21 us` (✅ **1.00x**)          | `3.87 us` (❌ *3.20x slower*)     | `23.04 ns` (🚀 **52.49x faster**) | `199.49 ns` (🚀 **6.06x faster**)  | `12.67 ns` (🚀 **95.48x faster**) | `8.68 ns` (🚀 **139.30x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.26 us` (✅ **1.00x**)          | `3.92 us` (❌ *3.12x slower*)     | `23.42 ns` (🚀 **53.67x faster**) | `160.86 ns` (🚀 **7.82x faster**)  | `12.81 ns` (🚀 **98.17x faster**) | `8.78 ns` (🚀 **143.18x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `875.24 ns` (✅ **1.00x**)        | `2.79 us` (❌ *3.18x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `902.87 ns` (✅ **1.00x**)        | `2.83 us` (❌ *3.13x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `582.46 ns` (✅ **1.00x**)        | `1.78 us` (❌ *3.05x slower*)     | `12.57 ns` (🚀 **46.34x faster**) | `72.01 ns` (🚀 **8.09x faster**)   | `7.26 ns` (🚀 **80.21x faster**)  | `5.87 ns` (🚀 **99.23x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `342.45 us` (✅ **1.00x**)        | `977.97 us` (❌ *2.86x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `22.18 ns` (❌ *3.59x slower*)    | `100.88 ns` (❌ *16.33x slower*)   | `18.26 ns` (❌ *2.96x slower*)    | `6.18 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `242.56 ns` (❌ *5.26x slower*)   | `6.23 us` (❌ *135.05x slower*)    | `76.48 ns` (❌ *1.66x slower*)    | `46.12 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `173.73 ns` (❌ *4.58x slower*)   | `4.37 us` (❌ *115.05x slower*)    | `65.24 ns` (❌ *1.72x slower*)    | `37.95 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `15.35 us` (❌ *2.16x slower*)    | `25.58 us` (❌ *3.60x slower*)     | `14.93 us` (❌ *2.10x slower*)    | `7.10 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `527.08 ns` (❌ *6.20x slower*)   | `12.77 us` (❌ *150.20x slower*)   | `118.89 ns` (❌ *1.40x slower*)   | `85.02 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `514.05 ns` (❌ *5.84x slower*)   | `12.74 us` (❌ *144.77x slower*)   | `164.71 ns` (❌ *1.87x slower*)   | `87.98 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.63 ns` (✅ **1.00x**)        | `8.65 ns` (❌ *1.13x slower*)    | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.65 ns` (✅ **1.00x**)        | `10.44 ns` (❌ *1.21x slower*)   | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.87 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.55 ns` (✅ **1.00x**)        | `4.54 ns` (✅ **1.00x faster**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381_optimized

|                                          | `g1projectivebls12_381`          | `g2projectivebls12_381`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                     | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `150.89 ns` (✅ **1.00x**)        | `204.28 ns` (❌ *1.35x slower*)   | `33.54 ns` (🚀 **4.50x faster**)    | `57.15 ns` (🚀 **2.64x faster**)    | `109.34 ns` (✅ **1.38x faster**)   | `703.74 ns` (❌ *4.66x slower*)    |
| **`serialize_uncompressed`**             | `192.66 ns` (✅ **1.00x**)        | `282.69 ns` (❌ *1.47x slower*)   | `32.06 ns` (🚀 **6.01x faster**)    | `55.23 ns` (🚀 **3.49x faster**)    | `109.27 ns` (✅ **1.76x faster**)   | `703.32 ns` (❌ *3.65x slower*)    |
| **`deserialize_compressed`**             | `166.97 us` (✅ **1.00x**)        | `284.10 us` (❌ *1.70x slower*)   | `53.25 ns` (🚀 **3135.57x faster**) | `94.25 ns` (🚀 **1771.52x faster**) | `217.62 ns` (🚀 **767.26x faster**) | `1.31 us` (🚀 **127.73x faster**)  |
| **`deserialize_compressed_unchecked`**   | `39.38 us` (✅ **1.00x**)         | `134.28 us` (❌ *3.41x slower*)   | `53.26 ns` (🚀 **739.45x faster**)  | `94.83 ns` (🚀 **415.25x faster**)  | `217.29 ns` (🚀 **181.24x faster**) | `1.30 us` (🚀 **30.20x faster**)   |
| **`deserialize_uncompressed`**           | `127.28 us` (✅ **1.00x**)        | `149.53 us` (❌ *1.17x slower*)   | `53.18 ns` (🚀 **2393.57x faster**) | `94.40 ns` (🚀 **1348.31x faster**) | `217.39 ns` (🚀 **585.49x faster**) | `1.30 us` (🚀 **97.61x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `220.79 ns` (✅ **1.00x**)        | `469.31 ns` (❌ *2.13x slower*)   | `53.15 ns` (🚀 **4.15x faster**)    | `94.69 ns` (🚀 **2.33x faster**)    | `217.26 ns` (✅ **1.02x faster**)   | `1.31 us` (❌ *5.91x slower*)      |

### msm_for_bls12_381_optimized

|        | `g1projectivebls12_381`          | `g2projectivebls12_381`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.30 s` (✅ **1.00x**)           | `7.04 s` (❌ *3.06x slower*)       |

### squareroot_for_bls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `25.56 us` (✅ **1.00x**) | `38.92 us` (❌ *1.52x slower*)   | `133.21 us` (❌ *5.21x slower*)    |
| **`legendre_for_qr`**    | `14.44 us` (✅ **1.00x**) | `38.65 us` (❌ *2.68x slower*)   | `38.51 us` (❌ *2.67x slower*)     |

### bitwise_operations_for_bls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.01 ns` (✅ **1.00x**)       | `88.82 ns` (❌ *1.85x slower*)    |
| **`from_big-endian_bits`**    | `47.97 ns` (✅ **1.00x**)       | `88.81 ns` (❌ *1.85x slower*)    |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)        | `5.13 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.43 ns` (✅ **1.00x**)        | `5.67 ns` (✅ **1.04x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.18 ns` (✅ **1.00x**) | `76.37 ns` (❌ *1.85x slower*)    |
| **`into_bigint`** | `22.21 ns` (✅ **1.00x**) | `48.03 ns` (❌ *2.16x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

