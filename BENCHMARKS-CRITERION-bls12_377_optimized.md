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
|        | `211.03 us` (✅ **1.00x**)                 | `2.04 ms` (❌ *9.69x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                   | `fq12optimized`                   | `fqoptimized`                     | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.25 us` (✅ **1.00x**)          | `4.58 us` (❌ *3.67x slower*)     | `23.10 ns` (🚀 **54.02x faster**) | `184.30 ns` (🚀 **6.77x faster**)  | `12.50 ns` (🚀 **99.79x faster**)  | `8.70 ns` (🚀 **143.32x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.29 us` (✅ **1.00x**)          | `4.63 us` (❌ *3.58x slower*)     | `23.29 ns` (🚀 **55.57x faster**) | `163.47 ns` (🚀 **7.92x faster**)  | `12.73 ns` (🚀 **101.64x faster**) | `8.79 ns` (🚀 **147.20x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `905.22 ns` (✅ **1.00x**)        | `3.31 us` (❌ *3.66x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `935.51 ns` (✅ **1.00x**)        | `3.35 us` (❌ *3.58x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `593.98 ns` (✅ **1.00x**)        | `2.25 us` (❌ *3.79x slower*)     | `12.39 ns` (🚀 **47.92x faster**) | `71.95 ns` (🚀 **8.26x faster**)   | `7.14 ns` (🚀 **83.20x faster**)   | `5.83 ns` (🚀 **101.83x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `340.76 us` (✅ **1.00x**)        | `1.17 ms` (❌ *3.44x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `22.58 ns` (❌ *3.68x slower*)    | `94.48 ns` (❌ *15.38x slower*)    | `18.22 ns` (❌ *2.97x slower*)     | `6.14 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `271.79 ns` (❌ *6.22x slower*)   | `7.10 us` (❌ *162.62x slower*)    | `75.28 ns` (❌ *1.72x slower*)     | `43.69 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `241.48 ns` (❌ *6.72x slower*)   | `5.02 us` (❌ *139.77x slower*)    | `66.77 ns` (❌ *1.86x slower*)     | `35.95 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `14.97 us` (❌ *2.12x slower*)    | `27.28 us` (❌ *3.87x slower*)     | `14.61 us` (❌ *2.07x slower*)     | `7.06 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `590.88 ns` (❌ *9.63x slower*)   | `14.53 us` (❌ *236.85x slower*)   | `117.85 ns` (❌ *1.92x slower*)    | `61.33 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `578.01 ns` (❌ *6.31x slower*)   | `14.46 us` (❌ *157.77x slower*)   | `162.34 ns` (❌ *1.77x slower*)    | `91.66 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**)        | `8.67 ns` (❌ *1.14x slower*)    | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.64 ns` (✅ **1.00x**)        | `10.31 ns` (❌ *1.19x slower*)   | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.87 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`division_by_2`**                   | `4.76 ns` (✅ **1.00x**)        | `4.77 ns` (✅ **1.00x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `159.13 ns` (✅ **1.00x**)        | `226.22 ns` (❌ *1.42x slower*)   | `31.22 ns` (🚀 **5.10x faster**)    | `56.13 ns` (🚀 **2.83x faster**)    | `110.47 ns` (✅ **1.44x faster**)    | `698.69 ns` (❌ *4.39x slower*)    |
| **`serialize_uncompressed`**             | `211.80 ns` (✅ **1.00x**)        | `347.03 ns` (❌ *1.64x slower*)   | `30.62 ns` (🚀 **6.92x faster**)    | `55.92 ns` (🚀 **3.79x faster**)    | `111.04 ns` (🚀 **1.91x faster**)    | `698.27 ns` (❌ *3.30x slower*)    |
| **`deserialize_compressed`**             | `315.91 us` (✅ **1.00x**)        | `1.06 ms` (❌ *3.35x slower*)     | `52.40 ns` (🚀 **6028.65x faster**) | `94.12 ns` (🚀 **3356.30x faster**) | `216.23 ns` (🚀 **1460.98x faster**) | `1.31 us` (🚀 **240.84x faster**)  |
| **`deserialize_compressed_unchecked`**   | `67.80 us` (✅ **1.00x**)         | `182.77 us` (❌ *2.70x slower*)   | `52.44 ns` (🚀 **1292.94x faster**) | `94.07 ns` (🚀 **720.69x faster**)  | `216.21 ns` (🚀 **313.58x faster**)  | `1.31 us` (🚀 **51.66x faster**)   |
| **`deserialize_uncompressed`**           | `248.34 us` (✅ **1.00x**)        | `873.87 us` (❌ *3.52x slower*)   | `52.39 ns` (🚀 **4740.31x faster**) | `94.13 ns` (🚀 **2638.36x faster**) | `215.87 ns` (🚀 **1150.44x faster**) | `1.31 us` (🚀 **189.38x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `223.94 ns` (✅ **1.00x**)        | `469.20 ns` (❌ *2.10x slower*)   | `52.38 ns` (🚀 **4.28x faster**)    | `94.08 ns` (🚀 **2.38x faster**)    | `215.91 ns` (✅ **1.04x faster**)    | `1.31 us` (❌ *5.86x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.35 s` (✅ **1.00x**)           | `8.38 s` (❌ *3.56x slower*)       |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.31 us` (✅ **1.00x**) | `67.23 us` (❌ *2.15x slower*)   | `181.72 us` (❌ *5.80x slower*)    |
| **`legendre_for_qr`**    | `10.95 us` (✅ **1.00x**) | `31.67 us` (❌ *2.89x slower*)   | `32.87 us` (❌ *3.00x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.84 ns` (✅ **1.00x**)        | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `49.18 ns` (✅ **1.00x**)       | `89.46 ns` (❌ *1.82x slower*)    |
| **`from_big-endian_bits`**    | `49.21 ns` (✅ **1.00x**)       | `89.25 ns` (❌ *1.81x slower*)    |
| **`comparison`**              | `4.87 ns` (✅ **1.00x**)        | `5.13 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.43 ns` (✅ **1.00x**)        | `5.67 ns` (✅ **1.04x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.54 ns` (✅ **1.00x**) | `75.19 ns` (❌ *1.85x slower*)    |
| **`into_bigint`** | `22.90 ns` (✅ **1.00x**) | `46.86 ns` (❌ *2.05x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

