# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [sample_bls12_377](#sample_bls12_377)
    - [arithmetic_for_bls12_377](#arithmetic_for_bls12_377)
    - [serialization_for_bls12_377](#serialization_for_bls12_377)
    - [msm_for_bls12_377](#msm_for_bls12_377)
    - [squareroot_for_bls12_377](#squareroot_for_bls12_377)
    - [bitwise_operations_for_bls12_377](#bitwise_operations_for_bls12_377)
    - [conversions_for_bls12_377](#conversions_for_bls12_377)

## Benchmark Results

### sample_bls12_377

|        | `g1projective_elements`          | `g2projective_elements`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `193.01 us` (✅ **1.00x**)        | `2.02 ms` (❌ *10.48x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                            | `fq12`                            | `fq`                              | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.26 us` (✅ **1.00x**)   | `4.79 us` (❌ *3.80x slower*)   | `23.20 ns` (🚀 **54.35x faster**) | `178.44 ns` (🚀 **7.07x faster**)  | `12.51 ns` (🚀 **100.80x faster**) | `8.70 ns` (🚀 **145.03x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.30 us` (✅ **1.00x**)   | `4.85 us` (❌ *3.73x slower*)   | `23.34 ns` (🚀 **55.82x faster**) | `160.63 ns` (🚀 **8.11x faster**)  | `12.81 ns` (🚀 **101.67x faster**) | `8.79 ns` (🚀 **148.26x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `913.40 ns` (✅ **1.00x**) | `3.42 us` (❌ *3.74x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `948.16 ns` (✅ **1.00x**) | `3.47 us` (❌ *3.66x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `592.91 ns` (✅ **1.00x**) | `2.25 us` (❌ *3.80x slower*)   | `12.35 ns` (🚀 **48.02x faster**) | `68.90 ns` (🚀 **8.61x faster**)   | `7.12 ns` (🚀 **83.29x faster**)   | `5.92 ns` (🚀 **100.23x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `323.97 us` (✅ **1.00x**) | `1.16 ms` (❌ *3.58x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `23.22 ns` (❌ *3.78x slower*)    | `95.07 ns` (❌ *15.47x slower*)    | `18.86 ns` (❌ *3.07x slower*)     | `6.14 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `272.48 ns` (❌ *6.36x slower*)   | `7.11 us` (❌ *166.07x slower*)    | `76.03 ns` (❌ *1.78x slower*)     | `42.82 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `245.28 ns` (❌ *6.92x slower*)   | `5.02 us` (❌ *141.75x slower*)    | `66.34 ns` (❌ *1.87x slower*)     | `35.44 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `15.31 us` (❌ *2.18x slower*)    | `27.65 us` (❌ *3.94x slower*)     | `14.97 us` (❌ *2.14x slower*)     | `7.01 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `590.98 ns` (❌ *9.65x slower*)   | `14.57 us` (❌ *237.95x slower*)   | `117.91 ns` (❌ *1.93x slower*)    | `61.24 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `579.37 ns` (❌ *6.51x slower*)   | `14.50 us` (❌ *162.82x slower*)   | `163.14 ns` (❌ *1.83x slower*)    | `89.05 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**) | `8.66 ns` (❌ *1.14x slower*)    | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.68 ns` (✅ **1.00x**) | `10.49 ns` (❌ *1.21x slower*)   | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.87 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`division_by_2`**                   | `4.56 ns` (✅ **1.00x**) | `4.56 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `155.87 ns` (✅ **1.00x**) | `220.04 ns` (❌ *1.41x slower*)   | `32.97 ns` (🚀 **4.73x faster**)    | `58.35 ns` (🚀 **2.67x faster**)    | `110.15 ns` (✅ **1.41x faster**)    | `701.11 ns` (❌ *4.50x slower*)    |
| **`serialize_uncompressed`**             | `212.04 ns` (✅ **1.00x**) | `337.26 ns` (❌ *1.59x slower*)   | `31.95 ns` (🚀 **6.64x faster**)    | `55.99 ns` (🚀 **3.79x faster**)    | `109.63 ns` (🚀 **1.93x faster**)    | `701.01 ns` (❌ *3.31x slower*)    |
| **`deserialize_compressed`**             | `312.55 us` (✅ **1.00x**) | `1.06 ms` (❌ *3.38x slower*)     | `52.87 ns` (🚀 **5911.39x faster**) | `90.78 ns` (🚀 **3442.79x faster**) | `209.19 ns` (🚀 **1494.10x faster**) | `1.26 us` (🚀 **248.66x faster**)  |
| **`deserialize_compressed_unchecked`**   | `68.04 us` (✅ **1.00x**)  | `183.74 us` (❌ *2.70x slower*)   | `52.82 ns` (🚀 **1288.06x faster**) | `90.53 ns` (🚀 **751.53x faster**)  | `209.28 ns` (🚀 **325.09x faster**)  | `1.26 us` (🚀 **54.13x faster**)   |
| **`deserialize_uncompressed`**           | `244.81 us` (✅ **1.00x**) | `870.20 us` (❌ *3.55x slower*)   | `52.79 ns` (🚀 **4637.84x faster**) | `90.66 ns` (🚀 **2700.28x faster**) | `209.10 ns` (🚀 **1170.80x faster**) | `1.25 us` (🚀 **195.79x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `226.76 ns` (✅ **1.00x**) | `465.15 ns` (❌ *2.05x slower*)   | `52.84 ns` (🚀 **4.29x faster**)    | `90.72 ns` (🚀 **2.50x faster**)    | `208.49 ns` (✅ **1.09x faster**)    | `1.26 us` (❌ *5.55x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.38 s` (✅ **1.00x**)  | `8.26 s` (❌ *3.47x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.15 us` (✅ **1.00x**) | `67.61 us` (❌ *2.17x slower*)   | `182.73 us` (❌ *5.87x slower*)    |
| **`legendre_for_qr`**    | `10.92 us` (✅ **1.00x**) | `31.39 us` (❌ *2.88x slower*)   | `31.56 us` (❌ *2.89x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.84 ns` (✅ **1.00x**)  | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.52 ns` (✅ **1.00x**) | `83.80 ns` (❌ *1.73x slower*)    |
| **`from_big-endian_bits`**    | `48.52 ns` (✅ **1.00x**) | `84.00 ns` (❌ *1.73x slower*)    |
| **`comparison`**              | `4.87 ns` (✅ **1.00x**)  | `5.13 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.45 ns` (✅ **1.00x**)  | `5.74 ns` (✅ **1.05x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.97 ns` (✅ **1.00x**) | `74.68 ns` (❌ *1.82x slower*)    |
| **`into_bigint`** | `23.46 ns` (✅ **1.00x**) | `46.93 ns` (❌ *2.00x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

