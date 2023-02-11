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
|        | `195.47 us` (✅ **1.00x**)        | `2.02 ms` (❌ *10.33x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                            | `fq12`                            | `fq`                              | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.24 us` (✅ **1.00x**)   | `4.78 us` (❌ *3.87x slower*)   | `23.20 ns` (🚀 **53.28x faster**) | `182.18 ns` (🚀 **6.78x faster**)  | `12.50 ns` (🚀 **98.86x faster**)  | `8.69 ns` (🚀 **142.21x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.28 us` (✅ **1.00x**)   | `4.85 us` (❌ *3.77x slower*)   | `23.37 ns` (🚀 **54.96x faster**) | `159.16 ns` (🚀 **8.07x faster**)  | `12.80 ns` (🚀 **100.34x faster**) | `8.81 ns` (🚀 **145.79x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `894.75 ns` (✅ **1.00x**) | `3.42 us` (❌ *3.82x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `929.46 ns` (✅ **1.00x**) | `3.47 us` (❌ *3.73x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `599.47 ns` (✅ **1.00x**) | `2.25 us` (❌ *3.75x slower*)   | `12.34 ns` (🚀 **48.58x faster**) | `71.01 ns` (🚀 **8.44x faster**)   | `7.12 ns` (🚀 **84.23x faster**)   | `5.83 ns` (🚀 **102.90x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `324.34 us` (✅ **1.00x**) | `1.16 ms` (❌ *3.57x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `23.26 ns` (❌ *3.77x slower*)    | `94.85 ns` (❌ *15.38x slower*)    | `18.75 ns` (❌ *3.04x slower*)     | `6.17 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `272.74 ns` (❌ *6.26x slower*)   | `7.12 us` (❌ *163.53x slower*)    | `76.27 ns` (❌ *1.75x slower*)     | `43.56 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `244.46 ns` (❌ *6.82x slower*)   | `5.03 us` (❌ *140.29x slower*)    | `66.40 ns` (❌ *1.85x slower*)     | `35.87 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `15.35 us` (❌ *2.18x slower*)    | `27.72 us` (❌ *3.94x slower*)     | `15.01 us` (❌ *2.13x slower*)     | `7.04 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `590.15 ns` (❌ *9.62x slower*)   | `14.59 us` (❌ *237.76x slower*)   | `117.97 ns` (❌ *1.92x slower*)    | `61.36 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `579.20 ns` (❌ *6.49x slower*)   | `14.51 us` (❌ *162.70x slower*)   | `163.91 ns` (❌ *1.84x slower*)    | `89.19 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**) | `8.66 ns` (❌ *1.14x slower*)    | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.65 ns` (✅ **1.00x**) | `10.34 ns` (❌ *1.20x slower*)   | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.87 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`division_by_2`**                   | `4.56 ns` (✅ **1.00x**) | `4.56 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `154.75 ns` (✅ **1.00x**) | `220.16 ns` (❌ *1.42x slower*)   | `30.62 ns` (🚀 **5.05x faster**)    | `55.42 ns` (🚀 **2.79x faster**)    | `109.69 ns` (✅ **1.41x faster**)    | `701.04 ns` (❌ *4.53x slower*)    |
| **`serialize_uncompressed`**             | `211.13 ns` (✅ **1.00x**) | `332.07 ns` (❌ *1.57x slower*)   | `31.02 ns` (🚀 **6.81x faster**)    | `55.30 ns` (🚀 **3.82x faster**)    | `109.68 ns` (🚀 **1.92x faster**)    | `696.64 ns` (❌ *3.30x slower*)    |
| **`deserialize_compressed`**             | `317.38 us` (✅ **1.00x**) | `1.05 ms` (❌ *3.32x slower*)     | `52.41 ns` (🚀 **6055.13x faster**) | `92.59 ns` (🚀 **3427.60x faster**) | `209.94 ns` (🚀 **1511.74x faster**) | `1.28 us` (🚀 **247.76x faster**)  |
| **`deserialize_compressed_unchecked`**   | `68.23 us` (✅ **1.00x**)  | `183.91 us` (❌ *2.70x slower*)   | `52.42 ns` (🚀 **1301.51x faster**) | `92.29 ns` (🚀 **739.27x faster**)  | `209.93 ns` (🚀 **325.00x faster**)  | `1.28 us` (🚀 **53.38x faster**)   |
| **`deserialize_uncompressed`**           | `249.17 us` (✅ **1.00x**) | `868.03 us` (❌ *3.48x slower*)   | `52.33 ns` (🚀 **4761.24x faster**) | `92.54 ns` (🚀 **2692.67x faster**) | `210.17 ns` (🚀 **1185.57x faster**) | `1.28 us` (🚀 **194.83x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `223.89 ns` (✅ **1.00x**) | `466.61 ns` (❌ *2.08x slower*)   | `52.34 ns` (🚀 **4.28x faster**)    | `91.74 ns` (🚀 **2.44x faster**)    | `210.10 ns` (✅ **1.07x faster**)    | `1.28 us` (❌ *5.72x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.39 s` (✅ **1.00x**)  | `8.26 s` (❌ *3.45x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.06 us` (✅ **1.00x**) | `67.52 us` (❌ *2.17x slower*)   | `182.92 us` (❌ *5.89x slower*)    |
| **`legendre_for_qr`**    | `10.85 us` (✅ **1.00x**) | `31.62 us` (❌ *2.91x slower*)   | `31.74 us` (❌ *2.92x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.84 ns` (✅ **1.00x**)  | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.07 ns` (✅ **1.00x**) | `83.35 ns` (❌ *1.73x slower*)    |
| **`from_big-endian_bits`**    | `48.05 ns` (✅ **1.00x**) | `83.37 ns` (❌ *1.74x slower*)    |
| **`comparison`**              | `4.87 ns` (✅ **1.00x**)  | `5.13 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.45 ns` (✅ **1.00x**)  | `5.74 ns` (✅ **1.05x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.71 ns` (✅ **1.00x**) | `75.02 ns` (❌ *1.84x slower*)    |
| **`into_bigint`** | `23.73 ns` (✅ **1.00x**) | `46.90 ns` (❌ *1.98x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

