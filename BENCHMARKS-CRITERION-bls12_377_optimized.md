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
|        | `211.07 us` (✅ **1.00x**)                 | `2.05 ms` (❌ *9.69x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                   | `fq12optimized`                   | `fqoptimized`                     | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.25 us` (✅ **1.00x**)          | `4.61 us` (❌ *3.70x slower*)     | `23.05 ns` (🚀 **54.15x faster**) | `181.09 ns` (🚀 **6.89x faster**)  | `12.52 ns` (🚀 **99.70x faster**)  | `8.70 ns` (🚀 **143.42x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.29 us` (✅ **1.00x**)          | `4.63 us` (❌ *3.58x slower*)     | `23.24 ns` (🚀 **55.71x faster**) | `161.48 ns` (🚀 **8.02x faster**)  | `12.77 ns` (🚀 **101.41x faster**) | `8.84 ns` (🚀 **146.56x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `906.18 ns` (✅ **1.00x**)        | `3.31 us` (❌ *3.65x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `936.70 ns` (✅ **1.00x**)        | `3.35 us` (❌ *3.58x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `592.95 ns` (✅ **1.00x**)        | `2.26 us` (❌ *3.81x slower*)     | `12.31 ns` (🚀 **48.18x faster**) | `68.75 ns` (🚀 **8.63x faster**)   | `7.14 ns` (🚀 **83.04x faster**)   | `5.83 ns` (🚀 **101.62x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `340.99 us` (✅ **1.00x**)        | `1.17 ms` (❌ *3.44x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `23.38 ns` (❌ *3.79x slower*)    | `94.58 ns` (❌ *15.34x slower*)    | `18.87 ns` (❌ *3.06x slower*)     | `6.17 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `271.47 ns` (❌ *6.25x slower*)   | `7.10 us` (❌ *163.38x slower*)    | `74.95 ns` (❌ *1.72x slower*)     | `43.45 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `241.98 ns` (❌ *6.73x slower*)   | `5.02 us` (❌ *139.53x slower*)    | `66.80 ns` (❌ *1.86x slower*)     | `35.95 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `14.97 us` (❌ *2.12x slower*)    | `27.29 us` (❌ *3.87x slower*)     | `14.62 us` (❌ *2.07x slower*)     | `7.06 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `590.82 ns` (❌ *9.63x slower*)   | `14.54 us` (❌ *237.02x slower*)   | `118.02 ns` (❌ *1.92x slower*)    | `61.35 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `577.96 ns` (❌ *6.30x slower*)   | `14.46 us` (❌ *157.67x slower*)   | `162.47 ns` (❌ *1.77x slower*)    | `91.72 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.64 ns` (✅ **1.00x**)        | `8.67 ns` (❌ *1.13x slower*)    | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.70 ns` (✅ **1.00x**)        | `10.29 ns` (❌ *1.18x slower*)   | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.87 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`division_by_2`**                   | `4.67 ns` (✅ **1.00x**)        | `4.71 ns` (✅ **1.01x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `160.06 ns` (✅ **1.00x**)        | `222.74 ns` (❌ *1.39x slower*)   | `31.30 ns` (🚀 **5.11x faster**)    | `57.06 ns` (🚀 **2.81x faster**)    | `110.32 ns` (✅ **1.45x faster**)    | `699.88 ns` (❌ *4.37x slower*)    |
| **`serialize_uncompressed`**             | `212.41 ns` (✅ **1.00x**)        | `344.63 ns` (❌ *1.62x slower*)   | `30.56 ns` (🚀 **6.95x faster**)    | `56.22 ns` (🚀 **3.78x faster**)    | `111.10 ns` (🚀 **1.91x faster**)    | `699.47 ns` (❌ *3.29x slower*)    |
| **`deserialize_compressed`**             | `315.97 us` (✅ **1.00x**)        | `1.06 ms` (❌ *3.35x slower*)     | `52.39 ns` (🚀 **6031.45x faster**) | `92.98 ns` (🚀 **3398.21x faster**) | `210.77 ns` (🚀 **1499.13x faster**) | `1.31 us` (🚀 **241.40x faster**)  |
| **`deserialize_compressed_unchecked`**   | `68.00 us` (✅ **1.00x**)         | `182.70 us` (❌ *2.69x slower*)   | `52.55 ns` (🚀 **1294.16x faster**) | `92.80 ns` (🚀 **732.77x faster**)  | `210.47 ns` (🚀 **323.10x faster**)  | `1.30 us` (🚀 **52.17x faster**)   |
| **`deserialize_uncompressed`**           | `248.42 us` (✅ **1.00x**)        | `875.00 us` (❌ *3.52x slower*)   | `52.32 ns` (🚀 **4747.91x faster**) | `93.01 ns` (🚀 **2670.96x faster**) | `210.33 ns` (🚀 **1181.09x faster**) | `1.30 us` (🚀 **190.68x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `224.74 ns` (✅ **1.00x**)        | `466.17 ns` (❌ *2.07x slower*)   | `52.46 ns` (🚀 **4.28x faster**)    | `93.11 ns` (🚀 **2.41x faster**)    | `210.27 ns` (✅ **1.07x faster**)    | `1.30 us` (❌ *5.80x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.36 s` (✅ **1.00x**)           | `8.39 s` (❌ *3.56x slower*)       |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.31 us` (✅ **1.00x**) | `67.26 us` (❌ *2.15x slower*)   | `181.75 us` (❌ *5.81x slower*)    |
| **`legendre_for_qr`**    | `10.95 us` (✅ **1.00x**) | `31.65 us` (❌ *2.89x slower*)   | `32.88 us` (❌ *3.00x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.95 ns` (✅ **1.00x**)       | `90.45 ns` (❌ *1.85x slower*)    |
| **`from_big-endian_bits`**    | `48.99 ns` (✅ **1.00x**)       | `90.47 ns` (❌ *1.85x slower*)    |
| **`comparison`**              | `4.88 ns` (✅ **1.00x**)        | `5.14 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.43 ns` (✅ **1.00x**)        | `5.65 ns` (✅ **1.04x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.59 ns` (✅ **1.00x**) | `75.29 ns` (❌ *1.85x slower*)    |
| **`into_bigint`** | `22.83 ns` (✅ **1.00x**) | `46.87 ns` (❌ *2.05x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

