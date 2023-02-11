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
|        | `195.55 us` (✅ **1.00x**)        | `2.02 ms` (❌ *10.34x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                            | `fq12`                            | `fq`                              | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.23 us` (✅ **1.00x**)   | `4.79 us` (❌ *3.88x slower*)   | `23.21 ns` (🚀 **53.18x faster**) | `180.30 ns` (🚀 **6.84x faster**)  | `12.49 ns` (🚀 **98.80x faster**)  | `8.70 ns` (🚀 **141.89x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.29 us` (✅ **1.00x**)   | `4.85 us` (❌ *3.77x slower*)   | `23.35 ns` (🚀 **55.05x faster**) | `160.45 ns` (🚀 **8.01x faster**)  | `12.81 ns` (🚀 **100.36x faster**) | `8.81 ns` (🚀 **145.83x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `895.37 ns` (✅ **1.00x**) | `3.44 us` (❌ *3.84x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `929.07 ns` (✅ **1.00x**) | `3.48 us` (❌ *3.75x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `600.64 ns` (✅ **1.00x**) | `2.26 us` (❌ *3.77x slower*)   | `12.28 ns` (🚀 **48.91x faster**) | `69.82 ns` (🚀 **8.60x faster**)   | `7.11 ns` (🚀 **84.48x faster**)   | `5.82 ns` (🚀 **103.16x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `324.34 us` (✅ **1.00x**) | `1.16 ms` (❌ *3.57x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `22.87 ns` (❌ *3.71x slower*)    | `95.90 ns` (❌ *15.57x slower*)    | `18.38 ns` (❌ *2.98x slower*)     | `6.16 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `271.76 ns` (❌ *6.27x slower*)   | `7.13 us` (❌ *164.40x slower*)    | `76.31 ns` (❌ *1.76x slower*)     | `43.36 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `244.98 ns` (❌ *6.82x slower*)   | `5.03 us` (❌ *139.89x slower*)    | `66.37 ns` (❌ *1.85x slower*)     | `35.93 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `15.31 us` (❌ *2.17x slower*)    | `27.70 us` (❌ *3.93x slower*)     | `14.99 us` (❌ *2.13x slower*)     | `7.04 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `590.08 ns` (❌ *9.62x slower*)   | `14.58 us` (❌ *237.59x slower*)   | `117.93 ns` (❌ *1.92x slower*)    | `61.36 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `578.87 ns` (❌ *6.49x slower*)   | `14.55 us` (❌ *163.16x slower*)   | `163.86 ns` (❌ *1.84x slower*)    | `89.16 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**) | `8.63 ns` (❌ *1.13x slower*)    | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.64 ns` (✅ **1.00x**) | `10.34 ns` (❌ *1.20x slower*)   | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.87 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`division_by_2`**                   | `4.55 ns` (✅ **1.00x**) | `4.56 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `154.10 ns` (✅ **1.00x**) | `220.02 ns` (❌ *1.43x slower*)   | `30.51 ns` (🚀 **5.05x faster**)    | `55.49 ns` (🚀 **2.78x faster**)    | `109.52 ns` (✅ **1.41x faster**)    | `700.46 ns` (❌ *4.55x slower*)    |
| **`serialize_uncompressed`**             | `210.53 ns` (✅ **1.00x**) | `331.86 ns` (❌ *1.58x slower*)   | `30.98 ns` (🚀 **6.80x faster**)    | `55.32 ns` (🚀 **3.81x faster**)    | `109.31 ns` (🚀 **1.93x faster**)    | `696.37 ns` (❌ *3.31x slower*)    |
| **`deserialize_compressed`**             | `316.88 us` (✅ **1.00x**) | `1.05 ms` (❌ *3.32x slower*)     | `52.63 ns` (🚀 **6020.85x faster**) | `93.25 ns` (🚀 **3397.96x faster**) | `209.84 ns` (🚀 **1510.10x faster**) | `1.29 us` (🚀 **245.05x faster**)  |
| **`deserialize_compressed_unchecked`**   | `68.17 us` (✅ **1.00x**)  | `183.83 us` (❌ *2.70x slower*)   | `52.64 ns` (🚀 **1295.18x faster**) | `92.94 ns` (🚀 **733.51x faster**)  | `209.83 ns` (🚀 **324.90x faster**)  | `1.29 us` (🚀 **52.75x faster**)   |
| **`deserialize_uncompressed`**           | `249.17 us` (✅ **1.00x**) | `867.78 us` (❌ *3.48x slower*)   | `52.52 ns` (🚀 **4744.17x faster**) | `93.13 ns` (🚀 **2675.64x faster**) | `209.42 ns` (🚀 **1189.82x faster**) | `1.29 us` (🚀 **192.58x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `223.73 ns` (✅ **1.00x**) | `484.55 ns` (❌ *2.17x slower*)   | `52.50 ns` (🚀 **4.26x faster**)    | `92.75 ns` (🚀 **2.41x faster**)    | `209.35 ns` (✅ **1.07x faster**)    | `1.29 us` (❌ *5.79x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.39 s` (✅ **1.00x**)  | `8.25 s` (❌ *3.45x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.04 us` (✅ **1.00x**) | `67.51 us` (❌ *2.17x slower*)   | `182.77 us` (❌ *5.89x slower*)    |
| **`legendre_for_qr`**    | `10.87 us` (✅ **1.00x**) | `31.61 us` (❌ *2.91x slower*)   | `31.72 us` (❌ *2.92x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.84 ns` (✅ **1.00x**)  | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.23 ns` (✅ **1.00x**) | `83.49 ns` (❌ *1.73x slower*)    |
| **`from_big-endian_bits`**    | `48.11 ns` (✅ **1.00x**) | `83.29 ns` (❌ *1.73x slower*)    |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)  | `5.12 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.45 ns` (✅ **1.00x**)  | `5.75 ns` (✅ **1.06x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.76 ns` (✅ **1.00x**) | `74.98 ns` (❌ *1.84x slower*)    |
| **`into_bigint`** | `23.74 ns` (✅ **1.00x**) | `46.91 ns` (❌ *1.98x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

