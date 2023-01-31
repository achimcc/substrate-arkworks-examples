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
|        | `219.08 us` (✅ **1.00x**)        | `2.05 ms` (❌ *9.34x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`             | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                            | `fq12`                            | `fq`                             | `fr`                               |
|:--------------------------------------|:-------------------------|:--------------------------------|:--------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                    | `N/A`                           | `1.38 us` (✅ **1.00x**)   | `4.76 us` (❌ *3.45x slower*)   | `33.03 ns` (🚀 **41.74x faster**) | `245.83 ns` (🚀 **5.61x faster**)  | `23.19 ns` (🚀 **59.45x faster**) | `10.35 ns` (🚀 **133.17x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `1.41 us` (✅ **1.00x**)   | `4.71 us` (❌ *3.34x slower*)   | `33.80 ns` (🚀 **41.69x faster**) | `239.11 ns` (🚀 **5.89x faster**)  | `18.30 ns` (🚀 **77.01x faster**) | `13.31 ns` (🚀 **105.91x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `1.02 us` (✅ **1.00x**)   | `3.37 us` (❌ *3.31x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `1.03 us` (✅ **1.00x**)   | `3.40 us` (❌ *3.30x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `737.02 ns` (✅ **1.00x**) | `2.14 us` (❌ *2.91x slower*)   | `18.39 ns` (🚀 **40.09x faster**) | `202.81 ns` (🚀 **3.63x faster**)  | `9.50 ns` (🚀 **77.58x faster**)  | `6.55 ns` (🚀 **112.44x faster**)   |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `345.65 us` (✅ **1.00x**) | `1.12 ms` (❌ *3.25x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `29.78 ns` (❌ *4.07x slower*)    | `166.14 ns` (❌ *22.71x slower*)   | `19.88 ns` (❌ *2.72x slower*)    | `7.32 ns` (✅ **1.00x**)            |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `281.44 ns` (❌ *5.62x slower*)   | `6.86 us` (❌ *137.01x slower*)    | `86.32 ns` (❌ *1.72x slower*)    | `50.08 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `217.51 ns` (❌ *4.97x slower*)   | `5.14 us` (❌ *117.32x slower*)    | `72.02 ns` (❌ *1.64x slower*)    | `43.78 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `16.53 us` (❌ *2.17x slower*)    | `27.45 us` (❌ *3.61x slower*)     | `15.49 us` (❌ *2.04x slower*)    | `7.60 us` (✅ **1.00x**)            |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `594.68 ns` (❌ *5.70x slower*)   | `14.52 us` (❌ *139.25x slower*)   | `127.64 ns` (❌ *1.22x slower*)   | `104.27 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `593.23 ns` (❌ *5.94x slower*)   | `14.13 us` (❌ *141.38x slower*)   | `177.93 ns` (❌ *1.78x slower*)   | `99.92 ns` (✅ **1.00x**)           |
| **`addition_with_carry`**             | `7.98 ns` (✅ **1.00x**)  | `10.30 ns` (❌ *1.29x slower*)   | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.19 ns` (✅ **1.00x**) | `13.26 ns` (❌ *1.30x slower*)   | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`multiplication_by_2`**             | `4.29 ns` (✅ **1.00x**)  | `4.63 ns` (✅ **1.08x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`division_by_2`**                   | `4.17 ns` (✅ **1.00x**)  | `4.27 ns` (✅ **1.02x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                                | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:------------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `172.70 ns` (✅ **1.00x**) | `257.37 ns` (❌ *1.49x slower*)   | `36.80 ns` (🚀 **4.69x faster**)    | `58.20 ns` (🚀 **2.97x faster**)     | `124.13 ns` (✅ **1.39x faster**)   | `735.35 ns` (❌ *4.26x slower*)    |
| **`serialize_uncompressed`**             | `234.85 ns` (✅ **1.00x**) | `381.62 ns` (❌ *1.62x slower*)   | `36.46 ns` (🚀 **6.44x faster**)    | `59.46 ns` (🚀 **3.95x faster**)     | `127.97 ns` (🚀 **1.84x faster**)   | `751.17 ns` (❌ *3.20x slower*)    |
| **`deserialize_compressed`**             | `145.60 us` (✅ **1.00x**) | `328.80 us` (❌ *2.26x slower*)   | `65.58 ns` (🚀 **2220.33x faster**) | `119.85 ns` (🚀 **1214.84x faster**) | `258.87 ns` (🚀 **562.46x faster**) | `1.63 us` (🚀 **89.06x faster**)   |
| **`deserialize_compressed_unchecked`**   | `48.89 us` (✅ **1.00x**)  | `164.39 us` (❌ *3.36x slower*)   | `66.31 ns` (🚀 **737.27x faster**)  | `119.67 ns` (🚀 **408.53x faster**)  | `264.50 ns` (🚀 **184.84x faster**) | `1.62 us` (🚀 **30.10x faster**)   |
| **`deserialize_uncompressed`**           | `105.80 us` (✅ **1.00x**) | `153.56 us` (❌ *1.45x slower*)   | `65.49 ns` (🚀 **1615.62x faster**) | `124.13 ns` (🚀 **852.33x faster**)  | `267.15 ns` (🚀 **396.04x faster**) | `1.63 us` (🚀 **65.03x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `254.12 ns` (✅ **1.00x**) | `586.55 ns` (❌ *2.31x slower*)   | `63.76 ns` (🚀 **3.99x faster**)    | `119.47 ns` (🚀 **2.13x faster**)    | `255.25 ns` (✅ **1.00x slower**)   | `1.65 us` (❌ *6.48x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.96 s` (✅ **1.00x**)  | `8.93 s` (❌ *3.02x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `26.11 us` (✅ **1.00x**) | `46.16 us` (❌ *1.77x slower*)   | `150.37 us` (❌ *5.76x slower*)    |
| **`legendre_for_qr`**    | `15.00 us` (✅ **1.00x**) | `47.39 us` (❌ *3.16x slower*)   | `47.13 us` (❌ *3.14x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`              | `fq::bigint`                      |
|:------------------------------|:--------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.47 ns` (✅ **1.00x**)   | `4.75 ns` (✅ **1.06x slower**)    |
| **`from_little-endian_bits`** | `168.26 ns` (✅ **1.00x**) | `260.45 ns` (❌ *1.55x slower*)    |
| **`from_big-endian_bits`**    | `169.59 ns` (✅ **1.00x**) | `259.55 ns` (❌ *1.53x slower*)    |
| **`comparison`**              | `4.45 ns` (✅ **1.00x**)   | `4.98 ns` (❌ *1.12x slower*)      |
| **`equality`**                | `5.20 ns` (✅ **1.00x**)   | `5.89 ns` (❌ *1.13x slower*)      |
| **`is_zero`**                 | `4.30 ns` (✅ **1.00x**)   | `4.57 ns` (✅ **1.06x slower**)    |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `42.86 ns` (✅ **1.00x**) | `92.35 ns` (❌ *2.15x slower*)    |
| **`into_bigint`** | `30.41 ns` (✅ **1.00x**) | `50.06 ns` (❌ *1.65x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

