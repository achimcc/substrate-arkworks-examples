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
|        | `203.82 us` (✅ **1.00x**)        | `1.80 ms` (❌ *8.83x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                   | `fq2`                            | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:---------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.22 us` (✅ **1.00x**)   | `3.98 us` (❌ *3.27x slower*)     | `23.32 ns` (🚀 **52.22x faster**) | `181.94 ns` (🚀 **6.69x faster**)  | `12.65 ns` (🚀 **96.24x faster**) | `8.68 ns` (🚀 **140.26x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.26 us` (✅ **1.00x**)   | `4.05 us` (❌ *3.20x slower*)     | `23.30 ns` (🚀 **54.26x faster**) | `160.96 ns` (🚀 **7.86x faster**)  | `12.84 ns` (🚀 **98.48x faster**) | `8.78 ns` (🚀 **144.04x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `882.34 ns` (✅ **1.00x**) | `2.85 us` (❌ *3.23x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `925.24 ns` (✅ **1.00x**) | `2.89 us` (❌ *3.12x slower*)     | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `572.66 ns` (✅ **1.00x**) | `1.82 us` (❌ *3.17x slower*)     | `12.47 ns` (🚀 **45.91x faster**) | `72.11 ns` (🚀 **7.94x faster**)   | `7.20 ns` (🚀 **79.50x faster**)  | `6.12 ns` (🚀 **93.52x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `322.26 us` (✅ **1.00x**) | `965.54 us` (❌ *3.00x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `22.26 ns` (❌ *3.61x slower*)    | `94.11 ns` (❌ *15.26x slower*)    | `18.28 ns` (❌ *2.96x slower*)    | `6.17 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `243.93 ns` (❌ *5.31x slower*)   | `6.25 us` (❌ *135.95x slower*)    | `76.39 ns` (❌ *1.66x slower*)    | `45.96 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `175.12 ns` (❌ *4.68x slower*)   | `4.39 us` (❌ *117.29x slower*)    | `64.99 ns` (❌ *1.74x slower*)    | `37.39 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `15.07 us` (❌ *2.11x slower*)    | `25.45 us` (❌ *3.56x slower*)     | `14.79 us` (❌ *2.07x slower*)    | `7.15 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `528.54 ns` (❌ *6.32x slower*)   | `12.79 us` (❌ *152.86x slower*)   | `116.60 ns` (❌ *1.39x slower*)   | `83.69 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                            | `518.13 ns` (❌ *5.93x slower*)   | `12.72 us` (❌ *145.48x slower*)   | `164.30 ns` (❌ *1.88x slower*)   | `87.44 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**) | `8.68 ns` (❌ *1.14x slower*)    | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.65 ns` (✅ **1.00x**) | `10.40 ns` (❌ *1.20x slower*)   | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.87 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `4.77 ns` (✅ **1.00x**) | `4.78 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                            | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `150.44 ns` (✅ **1.00x**) | `202.87 ns` (❌ *1.35x slower*)   | `32.52 ns` (🚀 **4.63x faster**)    | `57.09 ns` (🚀 **2.64x faster**)    | `110.57 ns` (✅ **1.36x faster**)   | `700.83 ns` (❌ *4.66x slower*)    |
| **`serialize_uncompressed`**             | `191.46 ns` (✅ **1.00x**) | `282.66 ns` (❌ *1.48x slower*)   | `33.03 ns` (🚀 **5.80x faster**)    | `55.71 ns` (🚀 **3.44x faster**)    | `110.56 ns` (✅ **1.73x faster**)   | `700.87 ns` (❌ *3.66x slower*)    |
| **`deserialize_compressed`**             | `131.84 us` (✅ **1.00x**) | `268.05 us` (❌ *2.03x slower*)   | `51.90 ns` (🚀 **2540.30x faster**) | `93.60 ns` (🚀 **1408.53x faster**) | `215.19 ns` (🚀 **612.68x faster**) | `1.31 us` (🚀 **100.34x faster**)  |
| **`deserialize_compressed_unchecked`**   | `39.85 us` (✅ **1.00x**)  | `135.66 us` (❌ *3.40x slower*)   | `51.87 ns` (🚀 **768.40x faster**)  | `93.72 ns` (🚀 **425.23x faster**)  | `215.16 ns` (🚀 **185.23x faster**) | `1.31 us` (🚀 **30.38x faster**)   |
| **`deserialize_uncompressed`**           | `92.00 us` (✅ **1.00x**)  | `132.03 us` (❌ *1.44x slower*)   | `51.82 ns` (🚀 **1775.24x faster**) | `93.69 ns` (🚀 **981.94x faster**)  | `215.02 ns` (🚀 **427.87x faster**) | `1.31 us` (🚀 **70.08x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `195.03 ns` (✅ **1.00x**) | `398.97 ns` (❌ *2.05x slower*)   | `51.79 ns` (🚀 **3.77x faster**)    | `93.54 ns` (🚀 **2.08x faster**)    | `215.02 ns` (✅ **1.10x slower**)   | `1.31 us` (❌ *6.73x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.33 s` (✅ **1.00x**)  | `7.07 s` (❌ *3.03x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `25.45 us` (✅ **1.00x**) | `39.48 us` (❌ *1.55x slower*)   | `134.68 us` (❌ *5.29x slower*)    |
| **`legendre_for_qr`**    | `14.36 us` (✅ **1.00x**) | `39.83 us` (❌ *2.77x slower*)   | `39.69 us` (❌ *2.76x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.84 ns` (✅ **1.00x**)  | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.18 ns` (✅ **1.00x**) | `83.18 ns` (❌ *1.73x slower*)    |
| **`from_big-endian_bits`**    | `48.01 ns` (✅ **1.00x**) | `83.15 ns` (❌ *1.73x slower*)    |
| **`comparison`**              | `4.87 ns` (✅ **1.00x**)  | `5.13 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.43 ns` (✅ **1.00x**)  | `5.69 ns` (✅ **1.05x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.05 ns` (✅ **1.00x**) | `75.95 ns` (❌ *1.85x slower*)    |
| **`into_bigint`** | `22.41 ns` (✅ **1.00x**) | `48.06 ns` (❌ *2.14x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

