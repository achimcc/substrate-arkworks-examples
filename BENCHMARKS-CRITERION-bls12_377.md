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
|        | `195.10 us` (✅ **1.00x**)        | `2.03 ms` (❌ *10.43x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                            | `fq12`                            | `fq`                              | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.25 us` (✅ **1.00x**)   | `4.79 us` (❌ *3.84x slower*)   | `23.25 ns` (🚀 **53.73x faster**) | `177.44 ns` (🚀 **7.04x faster**)  | `12.52 ns` (🚀 **99.78x faster**)  | `8.75 ns` (🚀 **142.75x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.30 us` (✅ **1.00x**)   | `4.85 us` (❌ *3.74x slower*)   | `23.29 ns` (🚀 **55.76x faster**) | `158.97 ns` (🚀 **8.17x faster**)  | `12.74 ns` (🚀 **101.96x faster**) | `8.81 ns` (🚀 **147.46x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `906.30 ns` (✅ **1.00x**) | `3.44 us` (❌ *3.79x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `940.77 ns` (✅ **1.00x**) | `3.47 us` (❌ *3.69x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `593.41 ns` (✅ **1.00x**) | `2.27 us` (❌ *3.82x slower*)   | `12.33 ns` (🚀 **48.14x faster**) | `70.78 ns` (🚀 **8.38x faster**)   | `7.15 ns` (🚀 **82.99x faster**)   | `5.93 ns` (🚀 **100.07x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `325.84 us` (✅ **1.00x**) | `1.16 ms` (❌ *3.57x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `22.64 ns` (❌ *3.68x slower*)    | `93.72 ns` (❌ *15.23x slower*)    | `18.36 ns` (❌ *2.98x slower*)     | `6.15 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `270.71 ns` (❌ *6.28x slower*)   | `7.11 us` (❌ *164.74x slower*)    | `76.47 ns` (❌ *1.77x slower*)     | `43.14 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `243.61 ns` (❌ *6.63x slower*)   | `5.01 us` (❌ *136.43x slower*)    | `66.36 ns` (❌ *1.81x slower*)     | `36.72 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `15.17 us` (❌ *2.15x slower*)    | `27.52 us` (❌ *3.89x slower*)     | `14.83 us` (❌ *2.10x slower*)     | `7.07 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `587.37 ns` (❌ *9.54x slower*)   | `14.60 us` (❌ *236.99x slower*)   | `117.77 ns` (❌ *1.91x slower*)    | `61.60 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `577.03 ns` (❌ *6.47x slower*)   | `14.49 us` (❌ *162.47x slower*)   | `162.70 ns` (❌ *1.82x slower*)    | `89.21 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.62 ns` (✅ **1.00x**) | `8.63 ns` (❌ *1.13x slower*)    | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.72 ns` (✅ **1.00x**) | `10.32 ns` (❌ *1.18x slower*)   | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.88 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`division_by_2`**                   | `4.52 ns` (✅ **1.00x**) | `4.52 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `157.86 ns` (✅ **1.00x**) | `219.93 ns` (❌ *1.39x slower*)   | `30.95 ns` (🚀 **5.10x faster**)    | `56.65 ns` (🚀 **2.79x faster**)    | `110.19 ns` (✅ **1.43x faster**)    | `702.22 ns` (❌ *4.45x slower*)    |
| **`serialize_uncompressed`**             | `214.79 ns` (✅ **1.00x**) | `332.50 ns` (❌ *1.55x slower*)   | `31.40 ns` (🚀 **6.84x faster**)    | `55.42 ns` (🚀 **3.88x faster**)    | `110.10 ns` (🚀 **1.95x faster**)    | `700.43 ns` (❌ *3.26x slower*)    |
| **`deserialize_compressed`**             | `315.20 us` (✅ **1.00x**) | `1.06 ms` (❌ *3.37x slower*)     | `52.26 ns` (🚀 **6031.23x faster**) | `93.12 ns` (🚀 **3384.74x faster**) | `209.65 ns` (🚀 **1503.45x faster**) | `1.29 us` (🚀 **243.52x faster**)  |
| **`deserialize_compressed_unchecked`**   | `68.28 us` (✅ **1.00x**)  | `184.61 us` (❌ *2.70x slower*)   | `52.28 ns` (🚀 **1306.07x faster**) | `92.82 ns` (🚀 **735.57x faster**)  | `209.56 ns` (🚀 **325.81x faster**)  | `1.29 us` (🚀 **52.79x faster**)   |
| **`deserialize_uncompressed`**           | `247.27 us` (✅ **1.00x**) | `875.59 us` (❌ *3.54x slower*)   | `52.25 ns` (🚀 **4732.21x faster**) | `93.32 ns` (🚀 **2649.66x faster**) | `209.50 ns` (🚀 **1180.27x faster**) | `1.29 us` (🚀 **191.07x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `221.66 ns` (✅ **1.00x**) | `470.35 ns` (❌ *2.12x slower*)   | `52.21 ns` (🚀 **4.25x faster**)    | `92.85 ns` (🚀 **2.39x faster**)    | `209.63 ns` (✅ **1.06x faster**)    | `1.29 us` (❌ *5.84x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.38 s` (✅ **1.00x**)  | `8.33 s` (❌ *3.51x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.11 us` (✅ **1.00x**) | `67.93 us` (❌ *2.18x slower*)   | `183.05 us` (❌ *5.88x slower*)    |
| **`legendre_for_qr`**    | `10.95 us` (✅ **1.00x**) | `32.03 us` (❌ *2.92x slower*)   | `32.11 us` (❌ *2.93x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.84 ns` (✅ **1.00x**)  | `5.03 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `50.10 ns` (✅ **1.00x**) | `89.43 ns` (❌ *1.79x slower*)    |
| **`from_big-endian_bits`**    | `50.12 ns` (✅ **1.00x**) | `89.44 ns` (❌ *1.78x slower*)    |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)  | `5.12 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.36 ns` (✅ **1.00x**)  | `5.65 ns` (✅ **1.05x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.82 ns` (✅ **1.00x**) | `75.09 ns` (❌ *1.84x slower*)    |
| **`into_bigint`** | `22.89 ns` (✅ **1.00x**) | `47.11 ns` (❌ *2.06x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

