# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [sample_bls12_381](#sample_bls12_381)
    - [arithmetic_for_bls12_381](#arithmetic_for_bls12_381)
    - [serialization_for_bls12_381](#serialization_for_bls12_381)
    - [msm_for_bls12_381](#msm_for_bls12_381)
    - [squareroot_for_bls12_381](#squareroot_for_bls12_381)
    - [bitwise_operations_for_bls12_381](#bitwise_operations_for_bls12_381)
    - [conversions_for_bls12_381](#conversions_for_bls12_381)

## Benchmark Results

### sample_bls12_381

|        | `g1projective_elements`          | `g2projective_elements`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `205.31 us` (✅ **1.00x**)        | `1.82 ms` (❌ *8.86x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                   | `fq2`                            | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.22 us` (✅ **1.00x**)   | `4.01 us` (❌ *3.28x slower*)     | `23.21 ns` (🚀 **52.69x faster**) | `194.99 ns` (🚀 **6.27x faster**)  | `12.66 ns` (🚀 **96.55x faster**) | `8.69 ns` (🚀 **140.74x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.27 us` (✅ **1.00x**)   | `4.07 us` (❌ *3.22x slower*)     | `23.48 ns` (🚀 **53.92x faster**) | `157.83 ns` (🚀 **8.02x faster**)  | `12.86 ns` (🚀 **98.43x faster**) | `8.76 ns` (🚀 **144.46x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `889.44 ns` (✅ **1.00x**) | `2.87 us` (❌ *3.23x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `923.32 ns` (✅ **1.00x**) | `2.93 us` (❌ *3.17x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `585.11 ns` (✅ **1.00x**) | `1.82 us` (❌ *3.10x slower*)     | `12.49 ns` (🚀 **46.86x faster**) | `67.40 ns` (🚀 **8.68x faster**)   | `7.27 ns` (🚀 **80.46x faster**)  | `5.86 ns` (🚀 **99.86x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `324.88 us` (✅ **1.00x**) | `972.55 us` (❌ *2.99x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `23.37 ns` (❌ *3.79x slower*)    | `95.71 ns` (❌ *15.53x slower*)    | `18.90 ns` (❌ *3.07x slower*)    | `6.16 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `243.50 ns` (❌ *5.30x slower*)   | `6.24 us` (❌ *135.90x slower*)    | `76.07 ns` (❌ *1.66x slower*)    | `45.95 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `173.19 ns` (❌ *4.60x slower*)   | `4.42 us` (❌ *117.24x slower*)    | `64.97 ns` (❌ *1.73x slower*)    | `37.66 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `15.35 us` (❌ *2.16x slower*)    | `25.60 us` (❌ *3.60x slower*)     | `14.96 us` (❌ *2.11x slower*)    | `7.10 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `528.65 ns` (❌ *6.32x slower*)   | `12.95 us` (❌ *154.86x slower*)   | `116.51 ns` (❌ *1.39x slower*)   | `83.61 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `519.65 ns` (❌ *5.94x slower*)   | `12.84 us` (❌ *146.79x slower*)   | `164.16 ns` (❌ *1.88x slower*)   | `87.45 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**) | `8.64 ns` (❌ *1.14x slower*)    | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.64 ns` (✅ **1.00x**) | `10.33 ns` (❌ *1.20x slower*)   | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.87 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.57 ns` (✅ **1.00x**) | `4.54 ns` (✅ **1.01x faster**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `150.75 ns` (✅ **1.00x**) | `203.46 ns` (❌ *1.35x slower*)   | `32.20 ns` (🚀 **4.68x faster**)    | `56.67 ns` (🚀 **2.66x faster**)    | `109.27 ns` (✅ **1.38x faster**)   | `1.08 us` (❌ *7.13x slower*)      |
| **`serialize_uncompressed`**             | `191.27 ns` (✅ **1.00x**) | `283.67 ns` (❌ *1.48x slower*)   | `33.19 ns` (🚀 **5.76x faster**)    | `55.48 ns` (🚀 **3.45x faster**)    | `109.68 ns` (✅ **1.74x faster**)   | `1.07 us` (❌ *5.62x slower*)      |
| **`deserialize_compressed`**             | `133.33 us` (✅ **1.00x**) | `267.05 us` (❌ *2.00x slower*)   | `52.76 ns` (🚀 **2527.29x faster**) | `93.31 ns` (🚀 **1428.99x faster**) | `213.67 ns` (🚀 **624.02x faster**) | `1.33 us` (🚀 **100.47x faster**)  |
| **`deserialize_compressed_unchecked`**   | `39.66 us` (✅ **1.00x**)  | `134.27 us` (❌ *3.39x slower*)   | `52.75 ns` (🚀 **751.93x faster**)  | `93.33 ns` (🚀 **425.00x faster**)  | `213.63 ns` (🚀 **185.67x faster**) | `1.33 us` (🚀 **29.91x faster**)   |
| **`deserialize_uncompressed`**           | `93.62 us` (✅ **1.00x**)  | `132.64 us` (❌ *1.42x slower*)   | `52.71 ns` (🚀 **1776.04x faster**) | `93.32 ns` (🚀 **1003.26x faster**) | `213.55 ns` (🚀 **438.41x faster**) | `1.33 us` (🚀 **70.56x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `196.22 ns` (✅ **1.00x**) | `416.93 ns` (❌ *2.12x slower*)   | `52.73 ns` (🚀 **3.72x faster**)    | `93.28 ns` (🚀 **2.10x faster**)    | `213.54 ns` (✅ **1.09x slower**)   | `1.33 us` (❌ *6.77x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.38 s` (✅ **1.00x**)  | `7.07 s` (❌ *2.97x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `25.44 us` (✅ **1.00x**) | `39.25 us` (❌ *1.54x slower*)   | `133.28 us` (❌ *5.24x slower*)    |
| **`legendre_for_qr`**    | `14.39 us` (✅ **1.00x**) | `39.55 us` (❌ *2.75x slower*)   | `38.53 us` (❌ *2.68x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)  | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `50.12 ns` (✅ **1.00x**) | `83.40 ns` (❌ *1.66x slower*)    |
| **`from_big-endian_bits`**    | `50.10 ns` (✅ **1.00x**) | `83.25 ns` (❌ *1.66x slower*)    |
| **`comparison`**              | `4.88 ns` (✅ **1.00x**)  | `5.13 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.45 ns` (✅ **1.00x**)  | `5.65 ns` (✅ **1.04x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.68 ns` (✅ **1.00x**) | `76.15 ns` (❌ *1.87x slower*)    |
| **`into_bigint`** | `22.46 ns` (✅ **1.00x**) | `47.97 ns` (❌ *2.14x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

