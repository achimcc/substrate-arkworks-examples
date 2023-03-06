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
|        | `202.12 us` (✅ **1.00x**)        | `1.80 ms` (❌ *8.91x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                   | `fq2`                            | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.21 us` (✅ **1.00x**)   | `3.98 us` (❌ *3.30x slower*)     | `23.44 ns` (🚀 **51.50x faster**) | `197.02 ns` (🚀 **6.13x faster**)  | `12.56 ns` (🚀 **96.12x faster**) | `8.66 ns` (🚀 **139.34x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.26 us` (✅ **1.00x**)   | `4.04 us` (❌ *3.22x slower*)     | `23.46 ns` (🚀 **53.55x faster**) | `159.52 ns` (🚀 **7.87x faster**)  | `12.67 ns` (🚀 **99.18x faster**) | `8.78 ns` (🚀 **143.09x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `877.74 ns` (✅ **1.00x**) | `2.85 us` (❌ *3.25x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `901.12 ns` (✅ **1.00x**) | `2.89 us` (❌ *3.21x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `579.78 ns` (✅ **1.00x**) | `1.79 us` (❌ *3.09x slower*)     | `12.50 ns` (🚀 **46.37x faster**) | `67.68 ns` (🚀 **8.57x faster**)   | `7.22 ns` (🚀 **80.27x faster**)  | `6.13 ns` (🚀 **94.55x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `320.12 us` (✅ **1.00x**) | `964.45 us` (❌ *3.01x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `22.92 ns` (❌ *3.72x slower*)    | `102.49 ns` (❌ *16.64x slower*)   | `18.60 ns` (❌ *3.02x slower*)    | `6.16 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `243.88 ns` (❌ *5.30x slower*)   | `6.25 us` (❌ *136.05x slower*)    | `76.33 ns` (❌ *1.66x slower*)    | `45.97 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `172.56 ns` (❌ *4.55x slower*)   | `4.44 us` (❌ *117.16x slower*)    | `65.05 ns` (❌ *1.72x slower*)    | `37.90 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `15.13 us` (❌ *2.12x slower*)    | `25.43 us` (❌ *3.57x slower*)     | `14.81 us` (❌ *2.08x slower*)    | `7.12 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `529.70 ns` (❌ *6.33x slower*)   | `12.84 us` (❌ *153.48x slower*)   | `116.37 ns` (❌ *1.39x slower*)   | `83.66 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `519.28 ns` (❌ *5.77x slower*)   | `12.75 us` (❌ *141.69x slower*)   | `163.13 ns` (❌ *1.81x slower*)   | `89.97 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.62 ns` (✅ **1.00x**) | `8.85 ns` (❌ *1.16x slower*)    | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.64 ns` (✅ **1.00x**) | `10.44 ns` (❌ *1.21x slower*)   | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.78 ns` (✅ **1.00x**) | `4.88 ns` (✅ **1.02x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.69 ns` (✅ **1.00x**) | `4.71 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `151.13 ns` (✅ **1.00x**) | `201.17 ns` (❌ *1.33x slower*)   | `31.79 ns` (🚀 **4.75x faster**)    | `57.21 ns` (🚀 **2.64x faster**)    | `111.33 ns` (✅ **1.36x faster**)   | `713.46 ns` (❌ *4.72x slower*)    |
| **`serialize_uncompressed`**             | `191.10 ns` (✅ **1.00x**) | `281.82 ns` (❌ *1.47x slower*)   | `32.17 ns` (🚀 **5.94x faster**)    | `54.73 ns` (🚀 **3.49x faster**)    | `111.18 ns` (✅ **1.72x faster**)   | `713.53 ns` (❌ *3.73x slower*)    |
| **`deserialize_compressed`**             | `131.45 us` (✅ **1.00x**) | `264.99 us` (❌ *2.02x slower*)   | `52.12 ns` (🚀 **2522.04x faster**) | `93.92 ns` (🚀 **1399.50x faster**) | `213.77 ns` (🚀 **614.92x faster**) | `1.33 us` (🚀 **98.59x faster**)   |
| **`deserialize_compressed_unchecked`**   | `38.99 us` (✅ **1.00x**)  | `133.43 us` (❌ *3.42x slower*)   | `52.10 ns` (🚀 **748.28x faster**)  | `93.90 ns` (🚀 **415.22x faster**)  | `213.74 ns` (🚀 **182.41x faster**) | `1.33 us` (🚀 **29.28x faster**)   |
| **`deserialize_uncompressed`**           | `92.13 us` (✅ **1.00x**)  | `131.23 us` (❌ *1.42x slower*)   | `52.05 ns` (🚀 **1770.03x faster**) | `93.99 ns` (🚀 **980.22x faster**)  | `213.51 ns` (🚀 **431.51x faster**) | `1.33 us` (🚀 **69.23x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `192.81 ns` (✅ **1.00x**) | `396.30 ns` (❌ *2.06x slower*)   | `52.16 ns` (🚀 **3.70x faster**)    | `93.99 ns` (🚀 **2.05x faster**)    | `213.58 ns` (✅ **1.11x slower**)   | `1.33 us` (❌ *6.91x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.32 s` (✅ **1.00x**)  | `7.04 s` (❌ *3.03x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `25.35 us` (✅ **1.00x**) | `38.44 us` (❌ *1.52x slower*)   | `132.47 us` (❌ *5.23x slower*)    |
| **`legendre_for_qr`**    | `14.35 us` (✅ **1.00x**) | `38.86 us` (❌ *2.71x slower*)   | `39.80 us` (❌ *2.77x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)  | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.93 ns` (✅ **1.00x**) | `90.80 ns` (❌ *1.86x slower*)    |
| **`from_big-endian_bits`**    | `48.89 ns` (✅ **1.00x**) | `90.71 ns` (❌ *1.86x slower*)    |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)  | `5.12 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.37 ns` (✅ **1.00x**)  | `5.72 ns` (✅ **1.07x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.07 ns` (✅ **1.00x**) | `76.45 ns` (❌ *1.86x slower*)    |
| **`into_bigint`** | `22.39 ns` (✅ **1.00x**) | `47.85 ns` (❌ *2.14x slower*)    |

### pairing_for_bls12_381

|        | `g1_preparation_for_bls12_381`          | `g2_preparation_for_bls12_381`          | `miller_loop_for_bls12_381`          | `final_exponentiation_for_bls12_381`          | `full_pairing_for_bls12_381`           |
|:-------|:----------------------------------------|:----------------------------------------|:-------------------------------------|:----------------------------------------------|:-------------------------------------- |
|        | `9.14 ns` (✅ **1.00x**)                 | `209.42 us` (❌ *22914.03x slower*)      | `592.54 us` (❌ *64833.38x slower*)   | `1.06 ms` (❌ *116195.81x slower*)             | `1.88 ms` (❌ *205835.88x slower*)      |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

