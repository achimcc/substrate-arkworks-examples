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
|        | `227.31 us` (✅ **1.00x**)        | `2.07 ms` (❌ *9.12x slower*)      |

### arithmetic_for_bls12_381

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                            | `fq12`                            | `fq`                             | `fr`                               |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:---------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.45 us` (✅ **1.00x**)   | `4.82 us` (❌ *3.32x slower*)   | `36.06 ns` (🚀 **40.18x faster**) | `231.62 ns` (🚀 **6.26x faster**)  | `24.05 ns` (🚀 **60.25x faster**) | `10.27 ns` (🚀 **141.05x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.44 us` (✅ **1.00x**)   | `4.61 us` (❌ *3.19x slower*)   | `34.16 ns` (🚀 **42.27x faster**) | `221.36 ns` (🚀 **6.52x faster**)  | `18.36 ns` (🚀 **78.68x faster**) | `11.10 ns` (🚀 **130.09x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `1.02 us` (✅ **1.00x**)   | `3.44 us` (❌ *3.37x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `1.05 us` (✅ **1.00x**)   | `3.31 us` (❌ *3.16x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`double`**                          | `N/A`                   | `N/A`                           | `713.13 ns` (✅ **1.00x**) | `2.06 us` (❌ *2.88x slower*)   | `16.36 ns` (🚀 **43.60x faster**) | `133.83 ns` (🚀 **5.33x faster**)  | `14.71 ns` (🚀 **48.47x faster**) | `6.75 ns` (🚀 **105.68x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `367.87 us` (✅ **1.00x**) | `1.11 ms` (❌ *3.02x slower*)   | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `28.29 ns` (❌ *3.91x slower*)    | `127.26 ns` (❌ *17.58x slower*)   | `23.21 ns` (❌ *3.21x slower*)    | `7.24 ns` (✅ **1.00x**)            |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `292.56 ns` (❌ *5.78x slower*)   | `7.21 us` (❌ *142.60x slower*)    | `87.97 ns` (❌ *1.74x slower*)    | `50.58 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `217.05 ns` (❌ *4.74x slower*)   | `5.05 us` (❌ *110.32x slower*)    | `75.55 ns` (❌ *1.65x slower*)    | `45.78 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `17.13 us` (❌ *2.07x slower*)    | `28.65 us` (❌ *3.46x slower*)     | `16.76 us` (❌ *2.03x slower*)    | `8.28 us` (✅ **1.00x**)            |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `619.37 ns` (❌ *5.85x slower*)   | `15.33 us` (❌ *144.89x slower*)   | `139.50 ns` (❌ *1.32x slower*)   | `105.82 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `590.77 ns` (❌ *5.87x slower*)   | `14.60 us` (❌ *145.12x slower*)   | `197.56 ns` (❌ *1.96x slower*)   | `100.59 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `8.50 ns` (✅ **1.00x**) | `10.05 ns` (❌ *1.18x slower*)   | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`subtraction_with_borrow`**         | `9.45 ns` (✅ **1.00x**) | `13.66 ns` (❌ *1.45x slower*)   | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`multiplication_by_2`**             | `4.91 ns` (✅ **1.00x**) | `5.11 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |
| **`division_by_2`**                   | `4.68 ns` (✅ **1.00x**) | `4.80 ns` (✅ **1.03x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                            | `N/A`                              |

### serialization_for_bls12_381

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                                | `fq2`                              | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:------------------------------------|:-----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `177.54 ns` (✅ **1.00x**) | `238.78 ns` (❌ *1.34x slower*)   | `37.44 ns` (🚀 **4.74x faster**)    | `62.06 ns` (🚀 **2.86x faster**)     | `125.98 ns` (✅ **1.41x faster**)   | `786.79 ns` (❌ *4.43x slower*)    |
| **`serialize_uncompressed`**             | `226.46 ns` (✅ **1.00x**) | `337.49 ns` (❌ *1.49x slower*)   | `37.33 ns` (🚀 **6.07x faster**)    | `62.01 ns` (🚀 **3.65x faster**)     | `122.47 ns` (🚀 **1.85x faster**)   | `804.43 ns` (❌ *3.55x slower*)    |
| **`deserialize_compressed`**             | `147.19 us` (✅ **1.00x**) | `322.70 us` (❌ *2.19x slower*)   | `58.60 ns` (🚀 **2511.83x faster**) | `118.19 ns` (🚀 **1245.37x faster**) | `259.56 ns` (🚀 **567.07x faster**) | `1.69 us` (🚀 **86.99x faster**)   |
| **`deserialize_compressed_unchecked`**   | `45.14 us` (✅ **1.00x**)  | `161.14 us` (❌ *3.57x slower*)   | `58.10 ns` (🚀 **776.98x faster**)  | `118.16 ns` (🚀 **382.02x faster**)  | `266.12 ns` (🚀 **169.62x faster**) | `1.65 us` (🚀 **27.35x faster**)   |
| **`deserialize_uncompressed`**           | `102.13 us` (✅ **1.00x**) | `148.25 us` (❌ *1.45x slower*)   | `58.49 ns` (🚀 **1746.06x faster**) | `121.35 ns` (🚀 **841.62x faster**)  | `259.50 ns` (🚀 **393.56x faster**) | `1.64 us` (🚀 **62.23x faster**)   |
| **`deserialize_uncompressed_unchecked`** | `250.61 ns` (✅ **1.00x**) | `521.78 ns` (❌ *2.08x slower*)   | `59.61 ns` (🚀 **4.20x faster**)    | `122.31 ns` (🚀 **2.05x faster**)    | `259.51 ns` (✅ **1.04x slower**)   | `1.59 us` (❌ *6.34x slower*)      |

### msm_for_bls12_381

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.80 s` (✅ **1.00x**)  | `8.40 s` (❌ *3.00x slower*)    |

### squareroot_for_bls12_381

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `27.82 us` (✅ **1.00x**) | `46.06 us` (❌ *1.66x slower*)   | `151.97 us` (❌ *5.46x slower*)    |
| **`legendre_for_qr`**    | `15.47 us` (✅ **1.00x**) | `46.19 us` (❌ *2.99x slower*)   | `45.08 us` (❌ *2.91x slower*)     |

### bitwise_operations_for_bls12_381

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.96 ns` (✅ **1.00x**)  | `5.24 ns` (✅ **1.06x slower**)    |
| **`from_little-endian_bits`** | `79.95 ns` (✅ **1.00x**) | `136.95 ns` (❌ *1.71x slower*)    |
| **`from_big-endian_bits`**    | `74.98 ns` (✅ **1.00x**) | `136.86 ns` (❌ *1.83x slower*)    |
| **`comparison`**              | `5.12 ns` (✅ **1.00x**)  | `5.38 ns` (✅ **1.05x slower**)    |
| **`equality`**                | `5.88 ns` (✅ **1.00x**)  | `5.83 ns` (✅ **1.01x faster**)    |
| **`is_zero`**                 | `4.88 ns` (✅ **1.00x**)  | `5.01 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_381

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `45.04 ns` (✅ **1.00x**) | `98.98 ns` (❌ *2.20x slower*)    |
| **`into_bigint`** | `27.04 ns` (✅ **1.00x**) | `53.79 ns` (❌ *1.99x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

