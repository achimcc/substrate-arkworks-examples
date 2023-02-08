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
|        | `202.28 us` (✅ **1.00x**)        | `1.79 ms` (❌ *8.84x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                   | `fq2`                            | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.21 us` (✅ **1.00x**)   | `3.94 us` (❌ *3.24x slower*)     | `23.20 ns` (🚀 **52.36x faster**) | `194.89 ns` (🚀 **6.23x faster**)  | `12.68 ns` (🚀 **95.83x faster**) | `8.67 ns` (🚀 **140.20x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.25 us` (✅ **1.00x**)   | `3.99 us` (❌ *3.18x slower*)     | `23.51 ns` (🚀 **53.29x faster**) | `159.51 ns` (🚀 **7.85x faster**)  | `12.84 ns` (🚀 **97.59x faster**) | `8.78 ns` (🚀 **142.65x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `879.29 ns` (✅ **1.00x**) | `2.83 us` (❌ *3.21x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `908.52 ns` (✅ **1.00x**) | `2.85 us` (❌ *3.14x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `582.02 ns` (✅ **1.00x**) | `1.79 us` (❌ *3.08x slower*)     | `12.55 ns` (🚀 **46.38x faster**) | `67.33 ns` (🚀 **8.64x faster**)   | `7.23 ns` (🚀 **80.45x faster**)  | `5.88 ns` (🚀 **98.91x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `320.97 us` (✅ **1.00x**) | `956.83 us` (❌ *2.98x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `22.34 ns` (❌ *3.62x slower*)    | `93.57 ns` (❌ *15.15x slower*)    | `18.31 ns` (❌ *2.96x slower*)    | `6.17 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `238.64 ns` (❌ *5.19x slower*)   | `6.18 us` (❌ *134.46x slower*)    | `76.46 ns` (❌ *1.66x slower*)    | `45.95 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `173.58 ns` (❌ *4.59x slower*)   | `4.33 us` (❌ *114.49x slower*)    | `65.19 ns` (❌ *1.72x slower*)    | `37.84 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `15.21 us` (❌ *2.13x slower*)    | `25.41 us` (❌ *3.57x slower*)     | `14.89 us` (❌ *2.09x slower*)    | `7.13 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `517.85 ns` (❌ *6.19x slower*)   | `12.65 us` (❌ *151.28x slower*)   | `115.21 ns` (❌ *1.38x slower*)   | `83.62 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `509.60 ns` (❌ *5.82x slower*)   | `12.58 us` (❌ *143.57x slower*)   | `163.11 ns` (❌ *1.86x slower*)   | `87.60 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.63 ns` (✅ **1.00x**) | `8.65 ns` (❌ *1.13x slower*)    | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.64 ns` (✅ **1.00x**) | `10.31 ns` (❌ *1.19x slower*)   | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.87 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.80 ns` (✅ **1.00x**) | `4.79 ns` (✅ **1.00x faster**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `151.13 ns` (✅ **1.00x**) | `202.98 ns` (❌ *1.34x slower*)   | `32.87 ns` (🚀 **4.60x faster**)    | `55.96 ns` (🚀 **2.70x faster**)    | `109.31 ns` (✅ **1.38x faster**)   | `700.53 ns` (❌ *4.64x slower*)    |
| **`serialize_uncompressed`**             | `191.92 ns` (✅ **1.00x**) | `282.63 ns` (❌ *1.47x slower*)   | `32.02 ns` (🚀 **5.99x faster**)    | `55.21 ns` (🚀 **3.48x faster**)    | `109.31 ns` (✅ **1.76x faster**)   | `698.76 ns` (❌ *3.64x slower*)    |
| **`deserialize_compressed`**             | `131.50 us` (✅ **1.00x**) | `264.68 us` (❌ *2.01x slower*)   | `52.10 ns` (🚀 **2523.83x faster**) | `94.93 ns` (🚀 **1385.19x faster**) | `222.01 ns` (🚀 **592.32x faster**) | `1.27 us` (🚀 **103.43x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.78 us` (✅ **1.00x**)  | `132.63 us` (❌ *3.42x slower*)   | `52.10 ns` (🚀 **744.44x faster**)  | `94.81 ns` (🚀 **409.08x faster**)  | `222.05 ns` (🚀 **174.66x faster**) | `1.27 us` (🚀 **30.49x faster**)   |
| **`deserialize_uncompressed`**           | `92.65 us` (✅ **1.00x**)  | `131.85 us` (❌ *1.42x slower*)   | `52.02 ns` (🚀 **1780.91x faster**) | `94.79 ns` (🚀 **977.33x faster**)  | `222.15 ns` (🚀 **417.04x faster**) | `1.27 us` (🚀 **72.81x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `201.80 ns` (✅ **1.00x**) | `393.83 ns` (❌ *1.95x slower*)   | `52.04 ns` (🚀 **3.88x faster**)    | `94.92 ns` (🚀 **2.13x faster**)    | `222.08 ns` (✅ **1.10x slower**)   | `1.27 us` (❌ *6.30x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.32 s` (✅ **1.00x**)  | `6.99 s` (❌ *3.01x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `25.40 us` (✅ **1.00x**) | `38.31 us` (❌ *1.51x slower*)   | `131.49 us` (❌ *5.18x slower*)    |
| **`legendre_for_qr`**    | `14.39 us` (✅ **1.00x**) | `38.47 us` (❌ *2.67x slower*)   | `39.70 us` (❌ *2.76x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)  | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `49.89 ns` (✅ **1.00x**) | `88.72 ns` (❌ *1.78x slower*)    |
| **`from_big-endian_bits`**    | `49.70 ns` (✅ **1.00x**) | `88.74 ns` (❌ *1.79x slower*)    |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)  | `5.12 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.37 ns` (✅ **1.00x**)  | `5.65 ns` (✅ **1.05x slower**)   |
| **`is_zero`**                 | `4.92 ns` (✅ **1.00x**)  | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.91 ns` (✅ **1.00x**) | `76.33 ns` (❌ *1.87x slower*)    |
| **`into_bigint`** | `22.45 ns` (✅ **1.00x**) | `47.92 ns` (❌ *2.13x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

