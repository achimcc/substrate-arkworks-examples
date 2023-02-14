# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [sample_bw6_761](#sample_bw6_761)
    - [arithmetic_for_bw6_761](#arithmetic_for_bw6_761)
    - [serialization_for_bw6_761](#serialization_for_bw6_761)
    - [msm_for_bw6_761](#msm_for_bw6_761)
    - [squareroot_for_bw6_761](#squareroot_for_bw6_761)
    - [bitwise_operations_for_bw6_761](#bitwise_operations_for_bw6_761)
    - [conversions_for_bw6_761](#conversions_for_bw6_761)

## Benchmark Results

### sample_bw6_761

|        | `g1projective_elements`          | `g2projective_elements`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.75 ms` (✅ **1.00x**)          | `2.71 ms` (✅ **1.02x faster**)    |

### arithmetic_for_bw6_761

|                                       | `fr::bigint`             | `fq::bigint`                    | `g1projective`          | `g2projective`                 | `fq3`                             | `fq6`                             | `fq`                              | `fr`                               |
|:--------------------------------------|:-------------------------|:--------------------------------|:------------------------|:-------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                    | `N/A`                           | `6.83 us` (✅ **1.00x**) | `6.68 us` (✅ **1.02x faster**) | `120.94 ns` (🚀 **56.51x faster**) | `232.80 ns` (🚀 **29.36x faster**) | `40.78 ns` (🚀 **167.59x faster**) | `24.00 ns` (🚀 **284.76x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `6.81 us` (✅ **1.00x**) | `6.85 us` (✅ **1.01x slower**) | `105.71 ns` (🚀 **64.46x faster**) | `198.44 ns` (🚀 **34.34x faster**) | `35.24 ns` (🚀 **193.34x faster**) | `19.95 ns` (🚀 **341.55x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `4.65 us` (✅ **1.00x**) | `4.70 us` (✅ **1.01x slower**) | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `4.74 us` (✅ **1.00x**) | `4.70 us` (✅ **1.01x faster**) | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `3.12 us` (✅ **1.00x**) | `3.10 us` (✅ **1.01x faster**) | `90.24 ns` (🚀 **34.63x faster**)  | `173.53 ns` (🚀 **18.01x faster**) | `27.16 ns` (🚀 **115.04x faster**) | `9.58 ns` (🚀 **326.01x faster**)   |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `2.40 ms` (✅ **1.00x**) | `2.41 ms` (✅ **1.00x slower**) | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `100.77 ns` (❌ *4.64x slower*)    | `175.55 ns` (❌ *8.09x slower*)    | `31.62 ns` (❌ *1.46x slower*)     | `21.70 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `3.44 us` (❌ *38.60x slower*)     | `10.63 us` (❌ *119.30x slower*)   | `447.30 ns` (❌ *5.02x slower*)    | `89.11 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `2.50 us` (❌ *31.80x slower*)     | `7.48 us` (❌ *95.01x slower*)     | `335.39 ns` (❌ *4.26x slower*)    | `78.69 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `68.11 us` (❌ *4.37x slower*)     | `78.27 us` (❌ *5.02x slower*)     | `60.72 us` (❌ *3.89x slower*)     | `15.59 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `6.92 us` (❌ *52.23x slower*)     | `21.75 us` (❌ *164.10x slower*)   | `498.99 ns` (❌ *3.77x slower*)    | `132.53 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `6.89 us` (❌ *30.44x slower*)     | `21.71 us` (❌ *95.89x slower*)    | `917.81 ns` (❌ *4.05x slower*)    | `226.43 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `20.97 ns` (✅ **1.00x**) | `20.68 ns` (✅ **1.01x faster**) | `N/A`                   | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `14.69 ns` (✅ **1.00x**) | `26.70 ns` (❌ *1.82x slower*)   | `N/A`                   | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.73 ns` (✅ **1.00x**)  | `4.98 ns` (✅ **1.05x slower**)  | `N/A`                   | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `4.29 ns` (✅ **1.00x**)  | `4.23 ns` (✅ **1.01x faster**)  | `N/A`                   | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761

|                                          | `g1projective`            | `g2projective`                   | `fr`                                 | `fq`                                | `fq3`                             | `fq6`                             |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-------------------------------------|:------------------------------------|:----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `688.17 ns` (✅ **1.00x**) | `693.89 ns` (✅ **1.01x slower**) | `63.60 ns` (🚀 **10.82x faster**)     | `229.94 ns` (🚀 **2.99x faster**)    | `663.81 ns` (✅ **1.04x faster**)  | `1.40 us` (❌ *2.04x slower*)      |
| **`serialize_uncompressed`**             | `920.07 ns` (✅ **1.00x**) | `925.68 ns` (✅ **1.01x slower**) | `64.86 ns` (🚀 **14.19x faster**)     | `228.29 ns` (🚀 **4.03x faster**)    | `681.61 ns` (✅ **1.35x faster**)  | `1.42 us` (❌ *1.55x slower*)      |
| **`deserialize_compressed`**             | `2.19 ms` (✅ **1.00x**)   | `2.21 ms` (✅ **1.01x slower**)   | `119.98 ns` (🚀 **18222.02x faster**) | `503.37 ns` (🚀 **4343.22x faster**) | `1.52 us` (🚀 **1440.76x faster**) | `3.04 us` (🚀 **718.63x faster**)  |
| **`deserialize_compressed_unchecked`**   | `407.30 us` (✅ **1.00x**) | `406.93 us` (✅ **1.00x faster**) | `119.23 ns` (🚀 **3416.12x faster**)  | `498.98 ns` (🚀 **816.26x faster**)  | `1.51 us` (🚀 **268.86x faster**)  | `3.06 us` (🚀 **133.29x faster**)  |
| **`deserialize_uncompressed`**           | `1.77 ms` (✅ **1.00x**)   | `1.78 ms` (✅ **1.00x slower**)   | `117.17 ns` (🚀 **15104.34x faster**) | `495.01 ns` (🚀 **3575.25x faster**) | `1.52 us` (🚀 **1161.57x faster**) | `3.01 us` (🚀 **587.07x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `1.08 us` (✅ **1.00x**)   | `1.08 us` (✅ **1.00x slower**)   | `117.91 ns` (🚀 **9.18x faster**)     | `497.10 ns` (🚀 **2.18x faster**)    | `1.54 us` (❌ *1.42x slower*)      | `3.01 us` (❌ *2.78x slower*)      |

### msm_for_bw6_761

|        | `g1projective`          | `g2projective`                  |
|:-------|:------------------------|:------------------------------- |
|        | `17.55 s` (✅ **1.00x**) | `17.57 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761

|                          | `fr`                     | `fq`                             | `fq3`                             |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `90.83 us` (✅ **1.00x**) | `413.68 us` (❌ *4.55x slower*)   | `9.38 ms` (❌ *103.32x slower*)    |
| **`legendre_for_qr`**    | `44.72 us` (✅ **1.00x**) | `423.79 us` (❌ *9.48x slower*)   | `418.78 us` (❌ *9.36x slower*)    |

### bitwise_operations_for_bw6_761

|                               | `fr::bigint`              | `fq::bigint`                      |
|:------------------------------|:--------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.89 ns` (✅ **1.00x**)   | `4.98 ns` (✅ **1.02x slower**)    |
| **`from_little-endian_bits`** | `132.24 ns` (✅ **1.00x**) | `250.40 ns` (❌ *1.89x slower*)    |
| **`from_big-endian_bits`**    | `130.63 ns` (✅ **1.00x**) | `249.49 ns` (❌ *1.91x slower*)    |
| **`comparison`**              | `4.85 ns` (✅ **1.00x**)   | `6.94 ns` (❌ *1.43x slower*)      |
| **`equality`**                | `5.73 ns` (✅ **1.00x**)   | `5.74 ns` (✅ **1.00x slower**)    |
| **`is_zero`**                 | `4.76 ns` (✅ **1.00x**)   | `4.82 ns` (✅ **1.01x slower**)    |

### conversions_for_bw6_761

|                   | `fr`                     | `fq`                              |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `96.95 ns` (✅ **1.00x**) | `450.13 ns` (❌ *4.64x slower*)    |
| **`into_bigint`** | `53.80 ns` (✅ **1.00x**) | `211.14 ns` (❌ *3.92x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

