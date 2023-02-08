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
|        | `229.65 us` (✅ **1.00x**)                 | `2.25 ms` (❌ *9.80x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                    | `fq12optimized`                   | `fqoptimized`                    | `froptimized`                      |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.35 us` (✅ **1.00x**)          | `5.12 us` (❌ *3.80x slower*)     | `34.14 ns` (🚀 **39.50x faster**)  | `212.29 ns` (🚀 **6.35x faster**)  | `22.76 ns` (🚀 **59.25x faster**) | `9.93 ns` (🚀 **135.77x faster**)   |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.43 us` (✅ **1.00x**)          | `5.17 us` (❌ *3.61x slower*)     | `32.56 ns` (🚀 **44.01x faster**)  | `202.82 ns` (🚀 **7.07x faster**)  | `17.89 ns` (🚀 **80.12x faster**) | `10.29 ns` (🚀 **139.25x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `981.08 ns` (✅ **1.00x**)        | `3.68 us` (❌ *3.75x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `1.02 us` (✅ **1.00x**)          | `3.72 us` (❌ *3.66x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`double`**                          | `N/A`                          | `N/A`                           | `680.49 ns` (✅ **1.00x**)        | `2.45 us` (❌ *3.60x slower*)     | `15.23 ns` (🚀 **44.68x faster**)  | `124.37 ns` (🚀 **5.47x faster**)  | `13.13 ns` (🚀 **51.81x faster**) | `11.37 ns` (🚀 **59.87x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `354.58 us` (✅ **1.00x**)        | `1.30 ms` (❌ *3.65x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `26.72 ns` (❌ *3.75x slower*)     | `125.52 ns` (❌ *17.62x slower*)   | `19.95 ns` (❌ *2.80x slower*)    | `7.13 ns` (✅ **1.00x**)            |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `317.04 ns` (❌ *7.13x slower*)    | `8.00 us` (❌ *179.79x slower*)    | `81.85 ns` (❌ *1.84x slower*)    | `44.49 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `302.38 ns` (❌ *7.73x slower*)    | `5.58 us` (❌ *142.65x slower*)    | `72.94 ns` (❌ *1.87x slower*)    | `39.10 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `16.30 us` (❌ *2.16x slower*)     | `30.25 us` (❌ *4.00x slower*)     | `15.79 us` (❌ *2.09x slower*)    | `7.56 us` (✅ **1.00x**)            |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `676.32 ns` (❌ *10.69x slower*)   | `16.21 us` (❌ *256.09x slower*)   | `132.40 ns` (❌ *2.09x slower*)   | `63.29 ns` (✅ **1.00x**)           |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `683.95 ns` (❌ *6.97x slower*)    | `15.99 us` (❌ *162.97x slower*)   | `185.86 ns` (❌ *1.89x slower*)   | `98.12 ns` (✅ **1.00x**)           |
| **`addition_with_carry`**             | `7.79 ns` (✅ **1.00x**)        | `9.45 ns` (❌ *1.21x slower*)    | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`subtraction_with_borrow`**         | `9.39 ns` (✅ **1.00x**)        | `12.73 ns` (❌ *1.36x slower*)   | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`multiplication_by_2`**             | `4.67 ns` (✅ **1.00x**)        | `4.82 ns` (✅ **1.03x slower**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`division_by_2`**                   | `4.48 ns` (✅ **1.00x**)        | `4.46 ns` (✅ **1.00x faster**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                       | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `179.25 ns` (✅ **1.00x**)        | `253.33 ns` (❌ *1.41x slower*)   | `33.55 ns` (🚀 **5.34x faster**)    | `64.86 ns` (🚀 **2.76x faster**)     | `118.35 ns` (✅ **1.51x faster**)    | `774.17 ns` (❌ *4.32x slower*)    |
| **`serialize_uncompressed`**             | `239.06 ns` (✅ **1.00x**)        | `383.46 ns` (❌ *1.60x slower*)   | `33.40 ns` (🚀 **7.16x faster**)    | `59.47 ns` (🚀 **4.02x faster**)     | `118.21 ns` (🚀 **2.02x faster**)    | `744.07 ns` (❌ *3.11x slower*)    |
| **`deserialize_compressed`**             | `334.29 us` (✅ **1.00x**)        | `1.17 ms` (❌ *3.50x slower*)     | `56.27 ns` (🚀 **5941.29x faster**) | `113.65 ns` (🚀 **2941.31x faster**) | `243.47 ns` (🚀 **1373.07x faster**) | `1.50 us` (🚀 **222.36x faster**)  |
| **`deserialize_compressed_unchecked`**   | `78.03 us` (✅ **1.00x**)         | `210.86 us` (❌ *2.70x slower*)   | `56.07 ns` (🚀 **1391.62x faster**) | `115.61 ns` (🚀 **674.97x faster**)  | `244.43 ns` (🚀 **319.25x faster**)  | `1.52 us` (🚀 **51.27x faster**)   |
| **`deserialize_uncompressed`**           | `257.24 us` (✅ **1.00x**)        | `956.10 us` (❌ *3.72x slower*)   | `56.04 ns` (🚀 **4590.25x faster**) | `121.35 ns` (🚀 **2119.89x faster**) | `247.80 ns` (🚀 **1038.10x faster**) | `1.51 us` (🚀 **170.65x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `266.43 ns` (✅ **1.00x**)        | `562.87 ns` (❌ *2.11x slower*)   | `56.39 ns` (🚀 **4.72x faster**)    | `114.29 ns` (🚀 **2.33x faster**)    | `245.23 ns` (✅ **1.09x faster**)    | `1.50 us` (❌ *5.64x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.70 s` (✅ **1.00x**)           | `9.57 s` (❌ *3.54x slower*)       |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `33.09 us` (✅ **1.00x**) | `77.71 us` (❌ *2.35x slower*)   | `206.76 us` (❌ *6.25x slower*)    |
| **`legendre_for_qr`**    | `11.42 us` (✅ **1.00x**) | `37.04 us` (❌ *3.24x slower*)   | `37.76 us` (❌ *3.31x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.77 ns` (✅ **1.00x**)        | `5.02 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `72.41 ns` (✅ **1.00x**)       | `127.61 ns` (❌ *1.76x slower*)    |
| **`from_big-endian_bits`**    | `72.17 ns` (✅ **1.00x**)       | `128.18 ns` (❌ *1.78x slower*)    |
| **`comparison`**              | `4.88 ns` (✅ **1.00x**)        | `5.18 ns` (✅ **1.06x slower**)    |
| **`equality`**                | `5.37 ns` (✅ **1.00x**)        | `5.58 ns` (✅ **1.04x slower**)    |
| **`is_zero`**                 | `4.66 ns` (✅ **1.00x**)        | `5.04 ns` (✅ **1.08x slower**)    |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `42.71 ns` (✅ **1.00x**) | `94.24 ns` (❌ *2.21x slower*)    |
| **`into_bigint`** | `25.77 ns` (✅ **1.00x**) | `49.43 ns` (❌ *1.92x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

