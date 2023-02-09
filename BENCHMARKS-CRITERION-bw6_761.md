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
|        | `1.98 ms` (✅ **1.00x**)          | `1.96 ms` (✅ **1.01x faster**)    |

### arithmetic_for_bw6_761

|                                       | `fr::bigint`             | `fq::bigint`                    | `g1projective`          | `g2projective`                 | `fq3`                            | `fq6`                             | `fq`                              | `fr`                               |
|:--------------------------------------|:-------------------------|:--------------------------------|:------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                    | `N/A`                           | `4.83 us` (✅ **1.00x**) | `4.83 us` (✅ **1.00x slower**) | `76.74 ns` (🚀 **62.90x faster**) | `160.83 ns` (🚀 **30.01x faster**) | `27.73 ns` (🚀 **174.11x faster**) | `12.61 ns` (🚀 **382.70x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `4.89 us` (✅ **1.00x**) | `4.89 us` (✅ **1.00x slower**) | `77.05 ns` (🚀 **63.50x faster**) | `153.20 ns` (🚀 **31.94x faster**) | `25.95 ns` (🚀 **188.53x faster**) | `13.36 ns` (🚀 **366.34x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `3.37 us` (✅ **1.00x**) | `3.37 us` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `3.41 us` (✅ **1.00x**) | `3.41 us` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `2.22 us` (✅ **1.00x**) | `2.22 us` (✅ **1.00x faster**) | `53.91 ns` (🚀 **41.10x faster**) | `117.90 ns` (🚀 **18.79x faster**) | `19.34 ns` (🚀 **114.54x faster**) | `7.14 ns` (🚀 **310.50x faster**)   |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `1.75 ms` (✅ **1.00x**) | `1.75 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `70.81 ns` (❌ *3.77x slower*)    | `121.02 ns` (❌ *6.44x slower*)    | `23.66 ns` (❌ *1.26x slower*)     | `18.79 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `2.47 us` (❌ *32.69x slower*)    | `7.90 us` (❌ *104.67x slower*)    | `310.48 ns` (❌ *4.11x slower*)    | `75.47 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `1.78 us` (❌ *26.81x slower*)    | `5.49 us` (❌ *82.65x slower*)     | `243.44 ns` (❌ *3.66x slower*)    | `66.48 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `52.33 us` (❌ *3.63x slower*)    | `61.42 us` (❌ *4.26x slower*)     | `48.18 us` (❌ *3.35x slower*)     | `14.40 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `5.09 us` (❌ *43.33x slower*)    | `16.07 us` (❌ *136.89x slower*)   | `417.68 ns` (❌ *3.56x slower*)    | `117.39 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `5.02 us` (❌ *30.70x slower*)    | `16.00 us` (❌ *97.78x slower*)    | `652.00 ns` (❌ *3.98x slower*)    | `163.67 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `8.65 ns` (✅ **1.00x**)  | `17.20 ns` (❌ *1.99x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.45 ns` (✅ **1.00x**) | `21.89 ns` (❌ *2.09x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.87 ns` (✅ **1.00x**)  | `4.95 ns` (✅ **1.02x slower**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `4.54 ns` (✅ **1.00x**)  | `4.54 ns` (✅ **1.00x slower**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761

|                                          | `g1projective`            | `g2projective`                   | `fr`                                | `fq`                                | `fq3`                             | `fq6`                             |
|:-----------------------------------------|:--------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `516.55 ns` (✅ **1.00x**) | `513.17 ns` (✅ **1.01x faster**) | `56.74 ns` (🚀 **9.10x faster**)     | `169.60 ns` (🚀 **3.05x faster**)    | `512.64 ns` (✅ **1.01x faster**)  | `1.10 us` (❌ *2.14x slower*)      |
| **`serialize_uncompressed`**             | `694.29 ns` (✅ **1.00x**) | `694.46 ns` (✅ **1.00x slower**) | `56.64 ns` (🚀 **12.26x faster**)    | `171.86 ns` (🚀 **4.04x faster**)    | `512.69 ns` (✅ **1.35x faster**)  | `1.11 us` (❌ *1.60x slower*)      |
| **`deserialize_compressed`**             | `1.59 ms` (✅ **1.00x**)   | `1.59 ms` (✅ **1.00x slower**)   | `93.06 ns` (🚀 **17062.73x faster**) | `338.95 ns` (🚀 **4684.56x faster**) | `1.05 us` (🚀 **1507.12x faster**) | `2.08 us` (🚀 **762.71x faster**)  |
| **`deserialize_compressed_unchecked`**   | `291.93 us` (✅ **1.00x**) | `292.10 us` (✅ **1.00x slower**) | `93.07 ns` (🚀 **3136.59x faster**)  | `338.99 ns` (🚀 **861.19x faster**)  | `1.05 us` (🚀 **277.13x faster**)  | `2.08 us` (🚀 **140.20x faster**)  |
| **`deserialize_uncompressed`**           | `1.30 ms` (✅ **1.00x**)   | `1.30 ms` (✅ **1.00x slower**)   | `93.49 ns` (🚀 **13868.78x faster**) | `338.90 ns` (🚀 **3826.02x faster**) | `1.05 us` (🚀 **1230.84x faster**) | `2.08 us` (🚀 **622.79x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `770.08 ns` (✅ **1.00x**) | `765.67 ns` (✅ **1.01x faster**) | `94.16 ns` (🚀 **8.18x faster**)     | `338.93 ns` (🚀 **2.27x faster**)    | `1.05 us` (❌ *1.37x slower*)      | `2.08 us` (❌ *2.70x slower*)      |

### msm_for_bw6_761

|        | `g1projective`          | `g2projective`                  |
|:-------|:------------------------|:------------------------------- |
|        | `12.60 s` (✅ **1.00x**) | `12.61 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761

|                          | `fr`                     | `fq`                             | `fq3`                             |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `67.08 us` (✅ **1.00x**) | `290.55 us` (❌ *4.33x slower*)   | `6.95 ms` (❌ *103.55x slower*)    |
| **`legendre_for_qr`**    | `31.96 us` (✅ **1.00x**) | `291.75 us` (❌ *9.13x slower*)   | `299.83 us` (❌ *9.38x slower*)    |

### bitwise_operations_for_bw6_761

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `5.03 ns` (✅ **1.00x**)  | `5.12 ns` (✅ **1.02x slower**)    |
| **`from_little-endian_bits`** | `83.22 ns` (✅ **1.00x**) | `181.70 ns` (❌ *2.18x slower*)    |
| **`from_big-endian_bits`**    | `83.23 ns` (✅ **1.00x**) | `181.42 ns` (❌ *2.18x slower*)    |
| **`comparison`**              | `5.08 ns` (✅ **1.00x**)  | `5.07 ns` (✅ **1.00x faster**)    |
| **`equality`**                | `5.67 ns` (✅ **1.00x**)  | `5.75 ns` (✅ **1.01x slower**)    |
| **`is_zero`**                 | `5.21 ns` (✅ **1.00x**)  | `5.35 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761

|                   | `fr`                     | `fq`                              |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `75.07 ns` (✅ **1.00x**) | `310.66 ns` (❌ *4.14x slower*)    |
| **`into_bigint`** | `47.05 ns` (✅ **1.00x**) | `156.66 ns` (❌ *3.33x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

