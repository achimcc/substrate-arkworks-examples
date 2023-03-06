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
|        | `231.12 us` (✅ **1.00x**)                 | `2.24 ms` (❌ *9.70x slower*)               |

### arithmetic_for_bls12_377_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `fq2optimized`                    | `fq12optimized`                   | `fqoptimized`                    | `froptimized`                     |
|:--------------------------------------|:-------------------------------|:--------------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `1.36 us` (✅ **1.00x**)          | `5.12 us` (❌ *3.75x slower*)     | `36.16 ns` (🚀 **37.74x faster**)  | `213.62 ns` (🚀 **6.39x faster**)  | `22.83 ns` (🚀 **59.77x faster**) | `9.97 ns` (🚀 **136.86x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `1.41 us` (✅ **1.00x**)          | `5.18 us` (❌ *3.68x slower*)     | `35.29 ns` (🚀 **39.87x faster**)  | `201.96 ns` (🚀 **6.97x faster**)  | `17.75 ns` (🚀 **79.25x faster**) | `16.50 ns` (🚀 **85.28x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `1.00 us` (✅ **1.00x**)          | `3.70 us` (❌ *3.69x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `1.03 us` (✅ **1.00x**)          | `3.73 us` (❌ *3.63x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                          | `N/A`                           | `682.43 ns` (✅ **1.00x**)        | `2.45 us` (❌ *3.60x slower*)     | `15.46 ns` (🚀 **44.13x faster**)  | `120.12 ns` (🚀 **5.68x faster**)  | `13.38 ns` (🚀 **51.00x faster**) | `6.37 ns` (🚀 **107.06x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `359.63 us` (✅ **1.00x**)        | `1.29 ms` (❌ *3.60x slower*)     | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `27.59 ns` (❌ *3.87x slower*)     | `131.05 ns` (❌ *18.37x slower*)   | `20.75 ns` (❌ *2.91x slower*)    | `7.13 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `321.60 ns` (❌ *7.19x slower*)    | `7.99 us` (❌ *178.57x slower*)    | `85.42 ns` (❌ *1.91x slower*)    | `44.73 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `299.28 ns` (❌ *7.84x slower*)    | `5.63 us` (❌ *147.42x slower*)    | `70.79 ns` (❌ *1.85x slower*)    | `38.18 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `16.41 us` (❌ *2.14x slower*)     | `30.07 us` (❌ *3.92x slower*)     | `15.99 us` (❌ *2.09x slower*)    | `7.66 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `683.86 ns` (❌ *10.76x slower*)   | `16.33 us` (❌ *256.83x slower*)   | `134.52 ns` (❌ *2.12x slower*)   | `63.57 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                            | `N/A`                            | `666.23 ns` (❌ *6.77x slower*)    | `16.21 us` (❌ *164.65x slower*)   | `189.25 ns` (❌ *1.92x slower*)   | `98.45 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.83 ns` (✅ **1.00x**)        | `9.40 ns` (❌ *1.20x slower*)    | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `9.41 ns` (✅ **1.00x**)        | `12.87 ns` (❌ *1.37x slower*)   | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.67 ns` (✅ **1.00x**)        | `4.85 ns` (✅ **1.04x slower**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.49 ns` (✅ **1.00x**)        | `4.49 ns` (✅ **1.00x slower**)  | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_377_optimized

|                                          | `g1projectivebls12_377`          | `g2projectivebls12_377`          | `froptimized`                      | `fqoptimized`                       | `fq2optimized`                      | `fq12optimized`                   |
|:-----------------------------------------|:---------------------------------|:---------------------------------|:-----------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `173.55 ns` (✅ **1.00x**)        | `254.78 ns` (❌ *1.47x slower*)   | `33.71 ns` (🚀 **5.15x faster**)    | `60.30 ns` (🚀 **2.88x faster**)     | `119.84 ns` (✅ **1.45x faster**)    | `754.55 ns` (❌ *4.35x slower*)    |
| **`serialize_uncompressed`**             | `237.10 ns` (✅ **1.00x**)        | `387.31 ns` (❌ *1.63x slower*)   | `33.63 ns` (🚀 **7.05x faster**)    | `60.03 ns` (🚀 **3.95x faster**)     | `119.81 ns` (🚀 **1.98x faster**)    | `752.82 ns` (❌ *3.18x slower*)    |
| **`deserialize_compressed`**             | `338.48 us` (✅ **1.00x**)        | `1.17 ms` (❌ *3.45x slower*)     | `55.67 ns` (🚀 **6080.50x faster**) | `112.51 ns` (🚀 **3008.48x faster**) | `249.72 ns` (🚀 **1355.45x faster**) | `1.51 us` (🚀 **224.10x faster**)  |
| **`deserialize_compressed_unchecked`**   | `78.02 us` (✅ **1.00x**)         | `207.85 us` (❌ *2.66x slower*)   | `55.71 ns` (🚀 **1400.47x faster**) | `112.65 ns` (🚀 **692.63x faster**)  | `249.52 ns` (🚀 **312.69x faster**)  | `1.51 us` (🚀 **51.66x faster**)   |
| **`deserialize_uncompressed`**           | `260.18 us` (✅ **1.00x**)        | `955.76 us` (❌ *3.67x slower*)   | `55.55 ns` (🚀 **4683.70x faster**) | `112.47 ns` (🚀 **2313.33x faster**) | `249.60 ns` (🚀 **1042.38x faster**) | `1.51 us` (🚀 **172.08x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `275.10 ns` (✅ **1.00x**)        | `571.05 ns` (❌ *2.08x slower*)   | `55.63 ns` (🚀 **4.94x faster**)    | `112.52 ns` (🚀 **2.44x faster**)    | `249.36 ns` (✅ **1.10x faster**)    | `1.51 us` (❌ *5.49x slower*)      |

### msm_for_bls12_377_optimized

|        | `g1projectivebls12_377`          | `g2projectivebls12_377`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.71 s` (✅ **1.00x**)           | `9.46 s` (❌ *3.49x slower*)       |

### squareroot_for_bls12_377_optimized

|                          | `froptimized`            | `fqoptimized`                   | `fq2optimized`                    |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `33.18 us` (✅ **1.00x**) | `77.52 us` (❌ *2.34x slower*)   | `206.94 us` (❌ *6.24x slower*)    |
| **`legendre_for_qr`**    | `11.43 us` (✅ **1.00x**) | `35.28 us` (❌ *3.09x slower*)   | `35.16 us` (❌ *3.08x slower*)     |

### bitwise_operations_for_bls12_377_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.78 ns` (✅ **1.00x**)        | `5.03 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `72.88 ns` (✅ **1.00x**)       | `130.61 ns` (❌ *1.79x slower*)    |
| **`from_big-endian_bits`**    | `73.03 ns` (✅ **1.00x**)       | `130.62 ns` (❌ *1.79x slower*)    |
| **`comparison`**              | `4.88 ns` (✅ **1.00x**)        | `5.19 ns` (✅ **1.07x slower**)    |
| **`equality`**                | `5.40 ns` (✅ **1.00x**)        | `5.69 ns` (✅ **1.05x slower**)    |
| **`is_zero`**                 | `4.68 ns` (✅ **1.00x**)        | `4.80 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_377_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `43.05 ns` (✅ **1.00x**) | `94.29 ns` (❌ *2.19x slower*)    |
| **`into_bigint`** | `26.12 ns` (✅ **1.00x**) | `49.67 ns` (❌ *1.90x slower*)    |

### pairing_for_bls12_377optimized

|        | `g1_preparation_for_bls12_377optimized`          | `g2_preparation_for_bls12_377optimized`          | `miller_loop_for_bls12_377optimized`          | `final_exponentiation_for_bls12_377optimized`          | `full_pairing_for_bls12_377optimized`           |
|:-------|:-------------------------------------------------|:-------------------------------------------------|:----------------------------------------------|:-------------------------------------------------------|:----------------------------------------------- |
|        | `10.85 ns` (✅ **1.00x**)                         | `13.34 ns` (❌ *1.23x slower*)                    | `1.04 ms` (❌ *96254.17x slower*)              | `1.41 ms` (❌ *130285.85x slower*)                      | `2.48 ms` (❌ *228358.16x slower*)               |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

