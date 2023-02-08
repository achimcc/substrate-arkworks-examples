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
|        | `202.54 us` (✅ **1.00x**)        | `1.82 ms` (❌ *8.99x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                   | `fq2`                            | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.24 us` (✅ **1.00x**)   | `3.99 us` (❌ *3.23x slower*)     | `23.34 ns` (🚀 **52.94x faster**) | `181.87 ns` (🚀 **6.80x faster**)  | `12.67 ns` (🚀 **97.52x faster**) | `8.65 ns` (🚀 **142.86x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.28 us` (✅ **1.00x**)   | `4.04 us` (❌ *3.15x slower*)     | `23.44 ns` (🚀 **54.79x faster**) | `159.67 ns` (🚀 **8.04x faster**)  | `12.87 ns` (🚀 **99.77x faster**) | `8.78 ns` (🚀 **146.25x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `890.18 ns` (✅ **1.00x**) | `2.86 us` (❌ *3.21x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `920.58 ns` (✅ **1.00x**) | `2.90 us` (❌ *3.15x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `584.69 ns` (✅ **1.00x**) | `1.83 us` (❌ *3.13x slower*)     | `12.52 ns` (🚀 **46.71x faster**) | `72.43 ns` (🚀 **8.07x faster**)   | `7.24 ns` (🚀 **80.78x faster**)  | `5.91 ns` (🚀 **98.93x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `322.89 us` (✅ **1.00x**) | `973.06 us` (❌ *3.01x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `22.67 ns` (❌ *3.69x slower*)    | `95.32 ns` (❌ *15.50x slower*)    | `18.66 ns` (❌ *3.03x slower*)    | `6.15 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `238.79 ns` (❌ *5.17x slower*)   | `6.15 us` (❌ *133.13x slower*)    | `76.64 ns` (❌ *1.66x slower*)    | `46.21 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `177.32 ns` (❌ *4.69x slower*)   | `4.35 us` (❌ *115.07x slower*)    | `65.00 ns` (❌ *1.72x slower*)    | `37.81 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `15.17 us` (❌ *2.12x slower*)    | `25.44 us` (❌ *3.56x slower*)     | `14.86 us` (❌ *2.08x slower*)    | `7.14 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `518.80 ns` (❌ *6.17x slower*)   | `12.61 us` (❌ *150.05x slower*)   | `115.40 ns` (❌ *1.37x slower*)   | `84.05 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `509.11 ns` (❌ *5.79x slower*)   | `12.55 us` (❌ *142.82x slower*)   | `164.49 ns` (❌ *1.87x slower*)   | `87.87 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.62 ns` (✅ **1.00x**) | `8.69 ns` (❌ *1.14x slower*)    | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.65 ns` (✅ **1.00x**) | `10.38 ns` (❌ *1.20x slower*)   | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.87 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.55 ns` (✅ **1.00x**) | `4.55 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `150.88 ns` (✅ **1.00x**) | `202.63 ns` (❌ *1.34x slower*)   | `32.27 ns` (🚀 **4.68x faster**)    | `55.67 ns` (🚀 **2.71x faster**)    | `109.45 ns` (✅ **1.38x faster**)   | `700.32 ns` (❌ *4.64x slower*)    |
| **`serialize_uncompressed`**             | `191.89 ns` (✅ **1.00x**) | `283.34 ns` (❌ *1.48x slower*)   | `32.70 ns` (🚀 **5.87x faster**)    | `55.63 ns` (🚀 **3.45x faster**)    | `109.49 ns` (✅ **1.75x faster**)   | `704.65 ns` (❌ *3.67x slower*)    |
| **`deserialize_compressed`**             | `131.91 us` (✅ **1.00x**) | `265.22 us` (❌ *2.01x slower*)   | `53.46 ns` (🚀 **2467.57x faster**) | `94.14 ns` (🚀 **1401.18x faster**) | `212.13 ns` (🚀 **621.82x faster**) | `1.32 us` (🚀 **99.73x faster**)   |
| **`deserialize_compressed_unchecked`**   | `38.79 us` (✅ **1.00x**)  | `133.13 us` (❌ *3.43x slower*)   | `53.58 ns` (🚀 **724.03x faster**)  | `94.08 ns` (🚀 **412.31x faster**)  | `211.90 ns` (🚀 **183.05x faster**) | `1.32 us` (🚀 **29.32x faster**)   |
| **`deserialize_uncompressed`**           | `93.00 us` (✅ **1.00x**)  | `131.89 us` (❌ *1.42x slower*)   | `53.40 ns` (🚀 **1741.65x faster**) | `93.99 ns` (🚀 **989.47x faster**)  | `211.47 ns` (🚀 **439.76x faster**) | `1.32 us` (🚀 **70.24x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `194.60 ns` (✅ **1.00x**) | `399.42 ns` (❌ *2.05x slower*)   | `53.43 ns` (🚀 **3.64x faster**)    | `93.98 ns` (🚀 **2.07x faster**)    | `211.50 ns` (✅ **1.09x slower**)   | `1.32 us` (❌ *6.80x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.33 s` (✅ **1.00x**)  | `7.11 s` (❌ *3.05x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `25.38 us` (✅ **1.00x**) | `38.26 us` (❌ *1.51x slower*)   | `132.05 us` (❌ *5.20x slower*)    |
| **`legendre_for_qr`**    | `14.35 us` (✅ **1.00x**) | `39.43 us` (❌ *2.75x slower*)   | `39.50 us` (❌ *2.75x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.92 ns` (✅ **1.00x**)  | `5.02 ns` (✅ **1.02x slower**)   |
| **`from_little-endian_bits`** | `48.17 ns` (✅ **1.00x**) | `93.97 ns` (❌ *1.95x slower*)    |
| **`from_big-endian_bits`**    | `48.15 ns` (✅ **1.00x**) | `89.72 ns` (❌ *1.86x slower*)    |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)  | `5.12 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.43 ns` (✅ **1.00x**)  | `5.65 ns` (✅ **1.04x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.98 ns` (✅ **1.00x**) | `76.27 ns` (❌ *1.86x slower*)    |
| **`into_bigint`** | `22.46 ns` (✅ **1.00x**) | `47.89 ns` (❌ *2.13x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

