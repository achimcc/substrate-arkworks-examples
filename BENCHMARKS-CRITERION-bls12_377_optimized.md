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
    - [pairing_for_bls12_377optimized](#pairing_for_bls12_377optimized)

## Benchmark Results

### sample_bls12_377_optimized

|        | `g1projectivebls12_377_elements`          | `g2projectivebls12_377_elements`           |
|:-------|:------------------------------------------|:------------------------------------------ |
|        | `213.99 us` (✅ **1.00x**)                 | `2.06 ms` (❌ *9.63x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                   | `fq12optimized`                   | `fqoptimized`                     | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.25 us` (✅ **1.00x**)          | `4.52 us` (❌ *3.61x slower*)     | `23.12 ns` (🚀 **54.14x faster**) | `201.02 ns` (🚀 **6.23x faster**)  | `12.55 ns` (🚀 **99.73x faster**)  | `8.71 ns` (🚀 **143.73x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.31 us` (✅ **1.00x**)          | `4.58 us` (❌ *3.49x slower*)     | `23.33 ns` (🚀 **56.19x faster**) | `159.15 ns` (🚀 **8.24x faster**)  | `12.75 ns` (🚀 **102.78x faster**) | `8.85 ns` (🚀 **148.07x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `903.28 ns` (✅ **1.00x**)        | `3.28 us` (❌ *3.63x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `936.19 ns` (✅ **1.00x**)        | `3.31 us` (❌ *3.54x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `609.98 ns` (✅ **1.00x**)        | `2.24 us` (❌ *3.67x slower*)     | `12.28 ns` (🚀 **49.65x faster**) | `68.07 ns` (🚀 **8.96x faster**)   | `7.14 ns` (🚀 **85.41x faster**)   | `5.85 ns` (🚀 **104.26x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `345.56 us` (✅ **1.00x**)        | `1.18 ms` (❌ *3.43x slower*)     | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `23.49 ns` (❌ *3.83x slower*)    | `104.27 ns` (❌ *17.01x slower*)   | `18.24 ns` (❌ *2.98x slower*)     | `6.13 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `271.13 ns` (❌ *6.33x slower*)   | `7.10 us` (❌ *165.69x slower*)    | `76.15 ns` (❌ *1.78x slower*)     | `42.84 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `237.82 ns` (❌ *6.51x slower*)   | `5.02 us` (❌ *137.31x slower*)    | `66.47 ns` (❌ *1.82x slower*)     | `36.54 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `16.16 us` (❌ *2.29x slower*)    | `27.58 us` (❌ *3.91x slower*)     | `14.88 us` (❌ *2.11x slower*)     | `7.05 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `580.07 ns` (❌ *9.43x slower*)   | `14.59 us` (❌ *237.08x slower*)   | `117.49 ns` (❌ *1.91x slower*)    | `61.54 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `566.13 ns` (❌ *6.19x slower*)   | `14.52 us` (❌ *158.59x slower*)   | `163.32 ns` (❌ *1.78x slower*)    | `91.53 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**)        | `8.67 ns` (❌ *1.14x slower*)    | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.71 ns` (✅ **1.00x**)        | `10.30 ns` (❌ *1.18x slower*)   | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`multiplication_by_2`**             | `4.71 ns` (✅ **1.00x**)        | `4.87 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`division_by_2`**                   | `4.68 ns` (✅ **1.00x**)        | `4.74 ns` (✅ **1.01x slower**)  | `N/A`                            | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                      | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `181.83 ns` (✅ **1.00x**)        | `223.96 ns` (❌ *1.23x slower*)   | `31.24 ns` (🚀 **5.82x faster**)    | `56.23 ns` (🚀 **3.23x faster**)    | `114.25 ns` (✅ **1.59x faster**)    | `700.31 ns` (❌ *3.85x slower*)    |
| **`serialize_uncompressed`**             | `209.72 ns` (✅ **1.00x**)        | `339.53 ns` (❌ *1.62x slower*)   | `30.42 ns` (🚀 **6.90x faster**)    | `56.36 ns` (🚀 **3.72x faster**)    | `114.32 ns` (🚀 **1.83x faster**)    | `700.54 ns` (❌ *3.34x slower*)    |
| **`deserialize_compressed`**             | `321.13 us` (✅ **1.00x**)        | `1.07 ms` (❌ *3.33x slower*)     | `52.77 ns` (🚀 **6085.49x faster**) | `94.05 ns` (🚀 **3414.34x faster**) | `213.24 ns` (🚀 **1505.95x faster**) | `1.29 us` (🚀 **248.56x faster**)  |
| **`deserialize_compressed_unchecked`**   | `68.61 us` (✅ **1.00x**)         | `184.88 us` (❌ *2.69x slower*)   | `52.63 ns` (🚀 **1303.45x faster**) | `93.96 ns` (🚀 **730.14x faster**)  | `213.84 ns` (🚀 **320.83x faster**)  | `1.29 us` (🚀 **53.10x faster**)   |
| **`deserialize_uncompressed`**           | `252.67 us` (✅ **1.00x**)        | `881.74 us` (❌ *3.49x slower*)   | `52.58 ns` (🚀 **4805.56x faster**) | `94.07 ns` (🚀 **2685.97x faster**) | `214.03 ns` (🚀 **1180.55x faster**) | `1.29 us` (🚀 **195.37x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `228.44 ns` (✅ **1.00x**)        | `492.88 ns` (❌ *2.16x slower*)   | `52.56 ns` (🚀 **4.35x faster**)    | `94.15 ns` (🚀 **2.43x faster**)    | `213.20 ns` (✅ **1.07x faster**)    | `1.29 us` (❌ *5.66x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.40 s` (✅ **1.00x**)           | `8.28 s` (❌ *3.45x slower*)       |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.17 us` (✅ **1.00x**) | `68.10 us` (❌ *2.19x slower*)   | `184.03 us` (❌ *5.90x slower*)    |
| **`legendre_for_qr`**    | `10.97 us` (✅ **1.00x**) | `32.44 us` (❌ *2.96x slower*)   | `32.01 us` (❌ *2.92x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.39 ns` (✅ **1.00x**)       | `90.17 ns` (❌ *1.86x slower*)    |
| **`from_big-endian_bits`**    | `48.73 ns` (✅ **1.00x**)       | `89.42 ns` (❌ *1.84x slower*)    |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)        | `5.20 ns` (✅ **1.06x slower**)   |
| **`equality`**                | `5.45 ns` (✅ **1.00x**)        | `5.67 ns` (✅ **1.04x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.99 ns` (✅ **1.00x**) | `75.08 ns` (❌ *1.83x slower*)    |
| **`into_bigint`** | `23.23 ns` (✅ **1.00x**) | `47.00 ns` (❌ *2.02x slower*)    |

### pairing_for_bls12_377optimized

|        | `g1_preparation_for_bls12_377optimized`          | `g2_preparation_for_bls12_377optimized`          | `miller_loop_for_bls12_377optimized`          | `final_exponentiation_for_bls12_377optimized`          | `full_pairing_for_bls12_377optimized`           |
|:-------|:-------------------------------------------------|:-------------------------------------------------|:----------------------------------------------|:-------------------------------------------------------|:----------------------------------------------- |
|        | `9.14 ns` (✅ **1.00x**)                          | `11.67 ns` (❌ *1.28x slower*)                    | `937.22 us` (❌ *102514.05x slower*)           | `1.27 ms` (❌ *139158.31x slower*)                      | `2.23 ms` (❌ *243507.81x slower*)               |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

