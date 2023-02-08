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
|        | `202.40 us` (✅ **1.00x**)        | `1.79 ms` (❌ *8.85x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                   | `fq2`                            | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.22 us` (✅ **1.00x**)   | `3.93 us` (❌ *3.23x slower*)     | `23.24 ns` (🚀 **52.34x faster**) | `194.04 ns` (🚀 **6.27x faster**)  | `12.68 ns` (🚀 **95.92x faster**) | `8.66 ns` (🚀 **140.44x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.25 us` (✅ **1.00x**)   | `4.00 us` (❌ *3.19x slower*)     | `23.49 ns` (🚀 **53.29x faster**) | `163.16 ns` (🚀 **7.67x faster**)  | `12.84 ns` (🚀 **97.45x faster**) | `8.79 ns` (🚀 **142.41x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `878.76 ns` (✅ **1.00x**) | `2.81 us` (❌ *3.20x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `907.56 ns` (✅ **1.00x**) | `2.86 us` (❌ *3.15x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `581.47 ns` (✅ **1.00x**) | `1.79 us` (❌ *3.08x slower*)     | `12.47 ns` (🚀 **46.63x faster**) | `68.95 ns` (🚀 **8.43x faster**)   | `7.23 ns` (🚀 **80.40x faster**)  | `5.89 ns` (🚀 **98.74x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `321.04 us` (✅ **1.00x**) | `956.08 us` (❌ *2.98x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `22.97 ns` (❌ *3.71x slower*)    | `94.63 ns` (❌ *15.30x slower*)    | `18.77 ns` (❌ *3.04x slower*)    | `6.18 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `238.98 ns` (❌ *5.20x slower*)   | `6.17 us` (❌ *134.39x slower*)    | `76.41 ns` (❌ *1.66x slower*)    | `45.94 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `172.98 ns` (❌ *4.57x slower*)   | `4.35 us` (❌ *114.86x slower*)    | `65.36 ns` (❌ *1.72x slower*)    | `37.89 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `15.22 us` (❌ *2.13x slower*)    | `25.43 us` (❌ *3.57x slower*)     | `14.88 us` (❌ *2.09x slower*)    | `7.13 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `520.42 ns` (❌ *6.23x slower*)   | `12.65 us` (❌ *151.36x slower*)   | `115.28 ns` (❌ *1.38x slower*)   | `83.58 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `509.62 ns` (❌ *5.82x slower*)   | `12.59 us` (❌ *143.73x slower*)   | `163.16 ns` (❌ *1.86x slower*)   | `87.62 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.63 ns` (✅ **1.00x**) | `8.64 ns` (❌ *1.13x slower*)    | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.64 ns` (✅ **1.00x**) | `10.31 ns` (❌ *1.19x slower*)   | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.88 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.80 ns` (✅ **1.00x**) | `4.79 ns` (✅ **1.00x faster**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `150.30 ns` (✅ **1.00x**) | `203.09 ns` (❌ *1.35x slower*)   | `32.53 ns` (🚀 **4.62x faster**)    | `56.03 ns` (🚀 **2.68x faster**)    | `109.35 ns` (✅ **1.37x faster**)   | `899.31 ns` (❌ *5.98x slower*)    |
| **`serialize_uncompressed`**             | `191.83 ns` (✅ **1.00x**) | `282.87 ns` (❌ *1.47x slower*)   | `31.91 ns` (🚀 **6.01x faster**)    | `55.22 ns` (🚀 **3.47x faster**)    | `109.36 ns` (✅ **1.75x faster**)   | `847.84 ns` (❌ *4.42x slower*)    |
| **`deserialize_compressed`**             | `131.59 us` (✅ **1.00x**) | `264.75 us` (❌ *2.01x slower*)   | `52.20 ns` (🚀 **2520.88x faster**) | `93.41 ns` (🚀 **1408.66x faster**) | `220.58 ns` (🚀 **596.56x faster**) | `1.27 us` (🚀 **103.53x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.80 us` (✅ **1.00x**)  | `132.68 us` (❌ *3.42x slower*)   | `52.18 ns` (🚀 **743.60x faster**)  | `93.58 ns` (🚀 **414.61x faster**)  | `220.60 ns` (🚀 **175.88x faster**) | `1.27 us` (🚀 **30.50x faster**)   |
| **`deserialize_uncompressed`**           | `92.66 us` (✅ **1.00x**)  | `131.85 us` (❌ *1.42x slower*)   | `52.09 ns` (🚀 **1779.00x faster**) | `93.50 ns` (🚀 **991.10x faster**)  | `220.53 ns` (🚀 **420.19x faster**) | `1.27 us` (🚀 **72.86x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `201.90 ns` (✅ **1.00x**) | `392.75 ns` (❌ *1.95x slower*)   | `52.08 ns` (🚀 **3.88x faster**)    | `93.49 ns` (🚀 **2.16x faster**)    | `220.59 ns` (✅ **1.09x slower**)   | `1.27 us` (❌ *6.30x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.32 s` (✅ **1.00x**)  | `7.01 s` (❌ *3.03x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `25.41 us` (✅ **1.00x**) | `38.31 us` (❌ *1.51x slower*)   | `131.57 us` (❌ *5.18x slower*)    |
| **`legendre_for_qr`**    | `14.39 us` (✅ **1.00x**) | `38.50 us` (❌ *2.67x slower*)   | `39.71 us` (❌ *2.76x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)  | `5.03 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.56 ns` (✅ **1.00x**) | `88.81 ns` (❌ *1.83x slower*)    |
| **`from_big-endian_bits`**    | `48.69 ns` (✅ **1.00x**) | `88.96 ns` (❌ *1.83x slower*)    |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)  | `5.12 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.37 ns` (✅ **1.00x**)  | `5.65 ns` (✅ **1.05x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.90 ns` (✅ **1.00x**) | `76.31 ns` (❌ *1.87x slower*)    |
| **`into_bigint`** | `22.46 ns` (✅ **1.00x**) | `47.92 ns` (❌ *2.13x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

