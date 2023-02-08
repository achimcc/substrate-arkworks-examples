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
|        | `202.44 us` (✅ **1.00x**)        | `1.79 ms` (❌ *8.84x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                   | `fq2`                            | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.21 us` (✅ **1.00x**)   | `3.93 us` (❌ *3.24x slower*)     | `23.23 ns` (🚀 **52.26x faster**) | `194.90 ns` (🚀 **6.23x faster**)  | `12.67 ns` (🚀 **95.80x faster**) | `8.66 ns` (🚀 **140.13x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.25 us` (✅ **1.00x**)   | `3.99 us` (❌ *3.19x slower*)     | `23.46 ns` (🚀 **53.44x faster**) | `160.30 ns` (🚀 **7.82x faster**)  | `12.83 ns` (🚀 **97.67x faster**) | `8.77 ns` (🚀 **142.90x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `879.29 ns` (✅ **1.00x**) | `2.81 us` (❌ *3.20x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `908.35 ns` (✅ **1.00x**) | `2.86 us` (❌ *3.14x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `583.61 ns` (✅ **1.00x**) | `1.79 us` (❌ *3.07x slower*)     | `12.46 ns` (🚀 **46.82x faster**) | `67.50 ns` (🚀 **8.65x faster**)   | `7.23 ns` (🚀 **80.71x faster**)  | `5.87 ns` (🚀 **99.43x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `321.06 us` (✅ **1.00x**) | `956.58 us` (❌ *2.98x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `22.71 ns` (❌ *3.68x slower*)    | `98.84 ns` (❌ *16.01x slower*)    | `18.29 ns` (❌ *2.96x slower*)    | `6.18 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `238.72 ns` (❌ *5.19x slower*)   | `6.17 us` (❌ *134.34x slower*)    | `76.42 ns` (❌ *1.66x slower*)    | `45.96 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `172.72 ns` (❌ *4.57x slower*)   | `4.35 us` (❌ *114.88x slower*)    | `65.18 ns` (❌ *1.72x slower*)    | `37.83 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `15.23 us` (❌ *2.13x slower*)    | `25.45 us` (❌ *3.57x slower*)     | `14.89 us` (❌ *2.09x slower*)    | `7.14 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `519.80 ns` (❌ *6.22x slower*)   | `12.64 us` (❌ *151.14x slower*)   | `115.23 ns` (❌ *1.38x slower*)   | `83.63 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `509.77 ns` (❌ *5.82x slower*)   | `12.59 us` (❌ *143.81x slower*)   | `163.14 ns` (❌ *1.86x slower*)   | `87.53 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.63 ns` (✅ **1.00x**) | `8.64 ns` (❌ *1.13x slower*)    | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.63 ns` (✅ **1.00x**) | `10.31 ns` (❌ *1.19x slower*)   | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.87 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.81 ns` (✅ **1.00x**) | `4.80 ns` (✅ **1.00x faster**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `151.39 ns` (✅ **1.00x**) | `202.92 ns` (❌ *1.34x slower*)   | `32.45 ns` (🚀 **4.67x faster**)    | `55.99 ns` (🚀 **2.70x faster**)    | `109.36 ns` (✅ **1.38x faster**)   | `700.87 ns` (❌ *4.63x slower*)    |
| **`serialize_uncompressed`**             | `192.03 ns` (✅ **1.00x**) | `282.53 ns` (❌ *1.47x slower*)   | `31.88 ns` (🚀 **6.02x faster**)    | `55.21 ns` (🚀 **3.48x faster**)    | `109.37 ns` (✅ **1.76x faster**)   | `698.88 ns` (❌ *3.64x slower*)    |
| **`deserialize_compressed`**             | `131.51 us` (✅ **1.00x**) | `264.94 us` (❌ *2.01x slower*)   | `52.42 ns` (🚀 **2508.81x faster**) | `94.39 ns` (🚀 **1393.31x faster**) | `215.55 ns` (🚀 **610.13x faster**) | `1.29 us` (🚀 **102.15x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.79 us` (✅ **1.00x**)  | `132.63 us` (❌ *3.42x slower*)   | `52.42 ns` (🚀 **739.96x faster**)  | `94.38 ns` (🚀 **411.03x faster**)  | `215.56 ns` (🚀 **179.96x faster**) | `1.29 us` (🚀 **30.13x faster**)   |
| **`deserialize_uncompressed`**           | `92.65 us` (✅ **1.00x**)  | `132.02 us` (❌ *1.43x slower*)   | `52.35 ns` (🚀 **1769.64x faster**) | `94.33 ns` (🚀 **982.12x faster**)  | `215.85 ns` (🚀 **429.21x faster**) | `1.29 us` (🚀 **71.96x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `204.13 ns` (✅ **1.00x**) | `401.35 ns` (❌ *1.97x slower*)   | `52.37 ns` (🚀 **3.90x faster**)    | `94.15 ns` (🚀 **2.17x faster**)    | `215.88 ns` (✅ **1.06x slower**)   | `1.29 us` (❌ *6.31x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.32 s` (✅ **1.00x**)  | `6.98 s` (❌ *3.01x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `25.40 us` (✅ **1.00x**) | `38.29 us` (❌ *1.51x slower*)   | `131.54 us` (❌ *5.18x slower*)    |
| **`legendre_for_qr`**    | `14.40 us` (✅ **1.00x**) | `38.46 us` (❌ *2.67x slower*)   | `39.72 us` (❌ *2.76x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.84 ns` (✅ **1.00x**)  | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.53 ns` (✅ **1.00x**) | `88.82 ns` (❌ *1.83x slower*)    |
| **`from_big-endian_bits`**    | `48.61 ns` (✅ **1.00x**) | `88.95 ns` (❌ *1.83x slower*)    |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)  | `5.13 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.36 ns` (✅ **1.00x**)  | `5.67 ns` (✅ **1.06x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.89 ns` (✅ **1.00x**) | `76.26 ns` (❌ *1.86x slower*)    |
| **`into_bigint`** | `22.45 ns` (✅ **1.00x**) | `47.94 ns` (❌ *2.14x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

