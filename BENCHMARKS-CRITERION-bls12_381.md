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
|        | `205.49 us` (✅ **1.00x**)        | `1.82 ms` (❌ *8.85x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                   | `fq2`                            | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.22 us` (✅ **1.00x**)   | `4.01 us` (❌ *3.28x slower*)     | `23.24 ns` (🚀 **52.70x faster**) | `194.16 ns` (🚀 **6.31x faster**)  | `12.72 ns` (🚀 **96.23x faster**) | `8.69 ns` (🚀 **140.90x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.27 us` (✅ **1.00x**)   | `4.07 us` (❌ *3.21x slower*)     | `23.52 ns` (🚀 **53.87x faster**) | `157.87 ns` (🚀 **8.02x faster**)  | `12.90 ns` (🚀 **98.18x faster**) | `8.76 ns` (🚀 **144.58x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `891.72 ns` (✅ **1.00x**) | `2.88 us` (❌ *3.23x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `924.22 ns` (✅ **1.00x**) | `2.91 us` (❌ *3.14x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `585.15 ns` (✅ **1.00x**) | `1.82 us` (❌ *3.10x slower*)     | `12.47 ns` (🚀 **46.92x faster**) | `67.39 ns` (🚀 **8.68x faster**)   | `7.28 ns` (🚀 **80.42x faster**)  | `5.87 ns` (🚀 **99.72x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `324.78 us` (✅ **1.00x**) | `973.71 us` (❌ *3.00x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `22.45 ns` (❌ *3.65x slower*)    | `94.02 ns` (❌ *15.27x slower*)    | `18.55 ns` (❌ *3.01x slower*)    | `6.16 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `244.05 ns` (❌ *5.31x slower*)   | `6.26 us` (❌ *136.30x slower*)    | `76.15 ns` (❌ *1.66x slower*)    | `45.96 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `173.09 ns` (❌ *4.60x slower*)   | `4.40 us` (❌ *116.76x slower*)    | `64.95 ns` (❌ *1.73x slower*)    | `37.64 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `15.34 us` (❌ *2.16x slower*)    | `25.59 us` (❌ *3.61x slower*)     | `14.96 us` (❌ *2.11x slower*)    | `7.09 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `528.05 ns` (❌ *6.32x slower*)   | `12.89 us` (❌ *154.18x slower*)   | `116.66 ns` (❌ *1.40x slower*)   | `83.58 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `519.12 ns` (❌ *5.94x slower*)   | `12.84 us` (❌ *146.93x slower*)   | `164.24 ns` (❌ *1.88x slower*)   | `87.41 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**) | `8.65 ns` (❌ *1.14x slower*)    | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.65 ns` (✅ **1.00x**) | `10.35 ns` (❌ *1.20x slower*)   | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.87 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.57 ns` (✅ **1.00x**) | `4.56 ns` (✅ **1.00x faster**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `151.07 ns` (✅ **1.00x**) | `203.52 ns` (❌ *1.35x slower*)   | `32.48 ns` (🚀 **4.65x faster**)    | `56.57 ns` (🚀 **2.67x faster**)    | `109.25 ns` (✅ **1.38x faster**)   | `700.13 ns` (❌ *4.63x slower*)    |
| **`serialize_uncompressed`**             | `191.72 ns` (✅ **1.00x**) | `283.06 ns` (❌ *1.48x slower*)   | `32.43 ns` (🚀 **5.91x faster**)    | `55.58 ns` (🚀 **3.45x faster**)    | `109.57 ns` (✅ **1.75x faster**)   | `698.96 ns` (❌ *3.65x slower*)    |
| **`deserialize_compressed`**             | `133.36 us` (✅ **1.00x**) | `267.34 us` (❌ *2.00x slower*)   | `53.11 ns` (🚀 **2511.11x faster**) | `93.58 ns` (🚀 **1425.06x faster**) | `214.00 ns` (🚀 **623.17x faster**) | `1.33 us` (🚀 **100.54x faster**)  |
| **`deserialize_compressed_unchecked`**   | `39.68 us` (✅ **1.00x**)  | `134.32 us` (❌ *3.38x slower*)   | `53.06 ns` (🚀 **747.90x faster**)  | `93.50 ns` (🚀 **424.40x faster**)  | `214.21 ns` (🚀 **185.26x faster**) | `1.32 us` (🚀 **29.95x faster**)   |
| **`deserialize_uncompressed`**           | `93.64 us` (✅ **1.00x**)  | `132.81 us` (❌ *1.42x slower*)   | `52.94 ns` (🚀 **1768.64x faster**) | `93.38 ns` (🚀 **1002.74x faster**) | `213.39 ns` (🚀 **438.83x faster**) | `1.33 us` (🚀 **70.60x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `196.41 ns` (✅ **1.00x**) | `424.77 ns` (❌ *2.16x slower*)   | `52.86 ns` (🚀 **3.72x faster**)    | `93.36 ns` (🚀 **2.10x faster**)    | `213.36 ns` (✅ **1.09x slower**)   | `1.33 us` (❌ *6.75x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.38 s` (✅ **1.00x**)  | `7.14 s` (❌ *2.99x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `25.45 us` (✅ **1.00x**) | `39.24 us` (❌ *1.54x slower*)   | `133.34 us` (❌ *5.24x slower*)    |
| **`legendre_for_qr`**    | `14.40 us` (✅ **1.00x**) | `39.56 us` (❌ *2.75x slower*)   | `38.54 us` (❌ *2.68x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)  | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.88 ns` (✅ **1.00x**) | `83.80 ns` (❌ *1.71x slower*)    |
| **`from_big-endian_bits`**    | `48.82 ns` (✅ **1.00x**) | `83.71 ns` (❌ *1.71x slower*)    |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)  | `5.23 ns` (✅ **1.07x slower**)   |
| **`equality`**                | `5.45 ns` (✅ **1.00x**)  | `5.65 ns` (✅ **1.04x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.75 ns` (✅ **1.00x**) | `76.35 ns` (❌ *1.87x slower*)    |
| **`into_bigint`** | `22.47 ns` (✅ **1.00x**) | `47.96 ns` (❌ *2.13x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

