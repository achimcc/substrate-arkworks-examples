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
    - [pairing_for_bls12_381](#pairing_for_bls12_381)

## Benchmark Results

### sample_bls12_381

|        | `g1projective_elements`          | `g2projective_elements`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `202.21 us` (✅ **1.00x**)        | `1.80 ms` (❌ *8.91x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                   | `fq2`                            | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.21 us` (✅ **1.00x**)   | `3.99 us` (❌ *3.31x slower*)     | `23.46 ns` (🚀 **51.45x faster**) | `199.32 ns` (🚀 **6.06x faster**)  | `12.57 ns` (🚀 **96.05x faster**) | `8.66 ns` (🚀 **139.31x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.26 us` (✅ **1.00x**)   | `4.04 us` (❌ *3.21x slower*)     | `23.46 ns` (🚀 **53.58x faster**) | `158.28 ns` (🚀 **7.94x faster**)  | `12.72 ns` (🚀 **98.85x faster**) | `8.79 ns` (🚀 **143.01x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `877.56 ns` (✅ **1.00x**) | `2.85 us` (❌ *3.25x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `901.83 ns` (✅ **1.00x**) | `2.90 us` (❌ *3.21x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `581.61 ns` (✅ **1.00x**) | `1.79 us` (❌ *3.08x slower*)     | `12.47 ns` (🚀 **46.63x faster**) | `74.42 ns` (🚀 **7.82x faster**)   | `7.27 ns` (🚀 **80.05x faster**)  | `6.13 ns` (🚀 **94.92x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `320.16 us` (✅ **1.00x**) | `965.05 us` (❌ *3.01x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `22.27 ns` (❌ *3.61x slower*)    | `101.44 ns` (❌ *16.45x slower*)   | `18.08 ns` (❌ *2.93x slower*)    | `6.17 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `244.32 ns` (❌ *5.31x slower*)   | `6.26 us` (❌ *136.21x slower*)    | `76.32 ns` (❌ *1.66x slower*)    | `45.97 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `173.47 ns` (❌ *4.58x slower*)   | `4.41 us` (❌ *116.28x slower*)    | `65.10 ns` (❌ *1.72x slower*)    | `37.89 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `15.14 us` (❌ *2.13x slower*)    | `25.47 us` (❌ *3.58x slower*)     | `14.82 us` (❌ *2.08x slower*)    | `7.12 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `528.38 ns` (❌ *6.32x slower*)   | `12.85 us` (❌ *153.57x slower*)   | `116.36 ns` (❌ *1.39x slower*)   | `83.65 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `519.56 ns` (❌ *5.77x slower*)   | `12.76 us` (❌ *141.74x slower*)   | `163.32 ns` (❌ *1.81x slower*)   | `90.00 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**) | `8.64 ns` (❌ *1.13x slower*)    | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.64 ns` (✅ **1.00x**) | `10.35 ns` (❌ *1.20x slower*)   | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.78 ns` (✅ **1.00x**) | `4.88 ns` (✅ **1.02x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.63 ns` (✅ **1.00x**) | `4.63 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `151.91 ns` (✅ **1.00x**) | `201.62 ns` (❌ *1.33x slower*)   | `32.53 ns` (🚀 **4.67x faster**)    | `57.22 ns` (🚀 **2.65x faster**)    | `111.19 ns` (✅ **1.37x faster**)   | `712.44 ns` (❌ *4.69x slower*)    |
| **`serialize_uncompressed`**             | `191.21 ns` (✅ **1.00x**) | `282.41 ns` (❌ *1.48x slower*)   | `33.12 ns` (🚀 **5.77x faster**)    | `54.64 ns` (🚀 **3.50x faster**)    | `111.20 ns` (✅ **1.72x faster**)   | `712.27 ns` (❌ *3.73x slower*)    |
| **`deserialize_compressed`**             | `131.24 us` (✅ **1.00x**) | `265.03 us` (❌ *2.02x slower*)   | `52.14 ns` (🚀 **2517.34x faster**) | `95.28 ns` (🚀 **1377.48x faster**) | `213.29 ns` (🚀 **615.33x faster**) | `1.33 us` (🚀 **98.75x faster**)   |
| **`deserialize_compressed_unchecked`**   | `38.94 us` (✅ **1.00x**)  | `133.56 us` (❌ *3.43x slower*)   | `52.23 ns` (🚀 **745.49x faster**)  | `95.35 ns` (🚀 **408.40x faster**)  | `213.43 ns` (🚀 **182.44x faster**) | `1.33 us` (🚀 **29.32x faster**)   |
| **`deserialize_uncompressed`**           | `92.16 us` (✅ **1.00x**)  | `131.18 us` (❌ *1.42x slower*)   | `51.88 ns` (🚀 **1776.47x faster**) | `95.38 ns` (🚀 **966.14x faster**)  | `212.93 ns` (🚀 **432.80x faster**) | `1.33 us` (🚀 **69.37x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `193.21 ns` (✅ **1.00x**) | `398.76 ns` (❌ *2.06x slower*)   | `52.08 ns` (🚀 **3.71x faster**)    | `95.44 ns` (🚀 **2.02x faster**)    | `212.96 ns` (✅ **1.10x slower**)   | `1.33 us` (❌ *6.87x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.32 s` (✅ **1.00x**)  | `7.10 s` (❌ *3.05x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `25.36 us` (✅ **1.00x**) | `38.49 us` (❌ *1.52x slower*)   | `132.56 us` (❌ *5.23x slower*)    |
| **`legendre_for_qr`**    | `14.36 us` (✅ **1.00x**) | `38.87 us` (❌ *2.71x slower*)   | `39.82 us` (❌ *2.77x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)  | `5.03 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.91 ns` (✅ **1.00x**) | `89.19 ns` (❌ *1.82x slower*)    |
| **`from_big-endian_bits`**    | `48.92 ns` (✅ **1.00x**) | `89.16 ns` (❌ *1.82x slower*)    |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)  | `5.13 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.36 ns` (✅ **1.00x**)  | `5.72 ns` (✅ **1.07x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.07 ns` (✅ **1.00x**) | `77.87 ns` (❌ *1.90x slower*)    |
| **`into_bigint`** | `22.39 ns` (✅ **1.00x**) | `47.85 ns` (❌ *2.14x slower*)    |

### pairing_for_bls12_381

|        | `g1_preparation_for_bls12_381`          | `g2_preparation_for_bls12_381`          | `miller_loop_for_bls12_381`          | `final_exponentiation_for_bls12_381`          | `full_pairing_for_bls12_381`           |
|:-------|:----------------------------------------|:----------------------------------------|:-------------------------------------|:----------------------------------------------|:-------------------------------------- |
|        | `9.13 ns` (✅ **1.00x**)                 | `209.71 us` (❌ *22965.00x slower*)      | `592.69 us` (❌ *64903.76x slower*)   | `1.06 ms` (❌ *116607.15x slower*)             | `1.88 ms` (❌ *206040.43x slower*)      |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

