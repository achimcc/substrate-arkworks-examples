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
|        | `205.12 us` (✅ **1.00x**)        | `1.81 ms` (❌ *8.83x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                   | `fq2`                            | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.25 us` (✅ **1.00x**)   | `3.97 us` (❌ *3.19x slower*)     | `23.38 ns` (🚀 **53.28x faster**) | `194.31 ns` (🚀 **6.41x faster**)  | `12.73 ns` (🚀 **97.86x faster**) | `8.64 ns` (🚀 **144.15x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.28 us` (✅ **1.00x**)   | `4.03 us` (❌ *3.14x slower*)     | `23.46 ns` (🚀 **54.62x faster**) | `159.51 ns` (🚀 **8.03x faster**)  | `12.85 ns` (🚀 **99.73x faster**) | `8.77 ns` (🚀 **146.05x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `897.86 ns` (✅ **1.00x**) | `2.85 us` (❌ *3.17x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `928.23 ns` (✅ **1.00x**) | `2.89 us` (❌ *3.11x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `585.60 ns` (✅ **1.00x**) | `1.80 us` (❌ *3.08x slower*)     | `12.43 ns` (🚀 **47.11x faster**) | `67.82 ns` (🚀 **8.63x faster**)   | `7.24 ns` (🚀 **80.89x faster**)  | `5.87 ns` (🚀 **99.74x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `322.53 us` (✅ **1.00x**) | `967.04 us` (❌ *3.00x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `22.81 ns` (❌ *3.72x slower*)    | `101.77 ns` (❌ *16.61x slower*)   | `18.24 ns` (❌ *2.98x slower*)    | `6.13 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `238.95 ns` (❌ *5.17x slower*)   | `6.21 us` (❌ *134.32x slower*)    | `76.92 ns` (❌ *1.67x slower*)    | `46.20 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `175.80 ns` (❌ *4.70x slower*)   | `4.36 us` (❌ *116.63x slower*)    | `65.43 ns` (❌ *1.75x slower*)    | `37.42 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `15.42 us` (❌ *2.17x slower*)    | `25.68 us` (❌ *3.61x slower*)     | `15.09 us` (❌ *2.12x slower*)    | `7.12 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `519.48 ns` (❌ *6.12x slower*)   | `12.74 us` (❌ *150.00x slower*)   | `116.34 ns` (❌ *1.37x slower*)   | `84.91 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `509.18 ns` (❌ *5.81x slower*)   | `12.67 us` (❌ *144.49x slower*)   | `163.48 ns` (❌ *1.86x slower*)   | `87.67 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**) | `8.64 ns` (❌ *1.13x slower*)    | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.65 ns` (✅ **1.00x**) | `10.29 ns` (❌ *1.19x slower*)   | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.78 ns` (✅ **1.00x**) | `4.87 ns` (✅ **1.02x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.56 ns` (✅ **1.00x**) | `4.56 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `151.54 ns` (✅ **1.00x**) | `202.23 ns` (❌ *1.33x slower*)   | `29.12 ns` (🚀 **5.20x faster**)    | `56.32 ns` (🚀 **2.69x faster**)    | `109.32 ns` (✅ **1.39x faster**)   | `711.80 ns` (❌ *4.70x slower*)    |
| **`serialize_uncompressed`**             | `190.98 ns` (✅ **1.00x**) | `283.18 ns` (❌ *1.48x slower*)   | `29.38 ns` (🚀 **6.50x faster**)    | `57.19 ns` (🚀 **3.34x faster**)    | `109.22 ns` (✅ **1.75x faster**)   | `717.51 ns` (❌ *3.76x slower*)    |
| **`deserialize_compressed`**             | `131.97 us` (✅ **1.00x**) | `267.67 us` (❌ *2.03x slower*)   | `52.97 ns` (🚀 **2491.40x faster**) | `94.47 ns` (🚀 **1396.93x faster**) | `213.94 ns` (🚀 **616.85x faster**) | `1.32 us` (🚀 **99.61x faster**)   |
| **`deserialize_compressed_unchecked`**   | `39.43 us` (✅ **1.00x**)  | `134.70 us` (❌ *3.42x slower*)   | `52.70 ns` (🚀 **748.32x faster**)  | `94.38 ns` (🚀 **417.82x faster**)  | `213.90 ns` (🚀 **184.36x faster**) | `1.33 us` (🚀 **29.75x faster**)   |
| **`deserialize_uncompressed`**           | `92.62 us` (✅ **1.00x**)  | `132.22 us` (❌ *1.43x slower*)   | `53.05 ns` (🚀 **1746.02x faster**) | `94.38 ns` (🚀 **981.37x faster**)  | `213.72 ns` (🚀 **433.37x faster**) | `1.33 us` (🚀 **69.90x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `192.46 ns` (✅ **1.00x**) | `397.91 ns` (❌ *2.07x slower*)   | `53.01 ns` (🚀 **3.63x faster**)    | `94.61 ns` (🚀 **2.03x faster**)    | `213.61 ns` (✅ **1.11x slower**)   | `1.33 us` (❌ *6.90x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.35 s` (✅ **1.00x**)  | `7.04 s` (❌ *3.00x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `25.45 us` (✅ **1.00x**) | `39.12 us` (❌ *1.54x slower*)   | `134.41 us` (❌ *5.28x slower*)    |
| **`legendre_for_qr`**    | `14.32 us` (✅ **1.00x**) | `39.12 us` (❌ *2.73x slower*)   | `39.03 us` (❌ *2.73x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.84 ns` (✅ **1.00x**)  | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.85 ns` (✅ **1.00x**) | `84.42 ns` (❌ *1.73x slower*)    |
| **`from_big-endian_bits`**    | `48.99 ns` (✅ **1.00x**) | `83.71 ns` (❌ *1.71x slower*)    |
| **`comparison`**              | `5.01 ns` (✅ **1.00x**)  | `5.12 ns` (✅ **1.02x slower**)   |
| **`equality`**                | `5.42 ns` (✅ **1.00x**)  | `5.67 ns` (✅ **1.05x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.50 ns` (✅ **1.00x**) | `76.13 ns` (❌ *1.83x slower*)    |
| **`into_bigint`** | `22.45 ns` (✅ **1.00x**) | `47.90 ns` (❌ *2.13x slower*)    |

### pairing_for_bls12_381

|        | `g1_preparation_for_bls12_381`          | `g2_preparation_for_bls12_381`          | `miller_loop_for_bls12_381`          | `final_exponentiation_for_bls12_381`          | `full_pairing_for_bls12_381`           |
|:-------|:----------------------------------------|:----------------------------------------|:-------------------------------------|:----------------------------------------------|:-------------------------------------- |
|        | `9.15 ns` (✅ **1.00x**)                 | `211.43 us` (❌ *23108.27x slower*)      | `590.67 us` (❌ *64556.81x slower*)   | `1.06 ms` (❌ *115889.64x slower*)             | `1.88 ms` (❌ *205486.18x slower*)      |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

