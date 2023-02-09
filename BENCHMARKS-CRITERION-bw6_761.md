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
| **`addition`**                        | `N/A`                    | `N/A`                           | `4.83 us` (✅ **1.00x**) | `4.83 us` (✅ **1.00x faster**) | `78.13 ns` (🚀 **61.79x faster**) | `159.14 ns` (🚀 **30.34x faster**) | `27.67 ns` (🚀 **174.46x faster**) | `12.62 ns` (🚀 **382.70x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `4.89 us` (✅ **1.00x**) | `4.89 us` (✅ **1.00x faster**) | `78.73 ns` (🚀 **62.09x faster**) | `150.76 ns` (🚀 **32.42x faster**) | `25.96 ns` (🚀 **188.26x faster**) | `13.36 ns` (🚀 **365.90x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `3.37 us` (✅ **1.00x**) | `3.37 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `3.41 us` (✅ **1.00x**) | `3.41 us` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `2.22 us` (✅ **1.00x**) | `2.22 us` (✅ **1.00x faster**) | `55.06 ns` (🚀 **40.32x faster**) | `116.62 ns` (🚀 **19.04x faster**) | `19.14 ns` (🚀 **115.98x faster**) | `7.14 ns` (🚀 **311.15x faster**)   |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `1.75 ms` (✅ **1.00x**) | `1.75 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `71.93 ns` (❌ *3.90x slower*)    | `119.32 ns` (❌ *6.47x slower*)    | `22.32 ns` (❌ *1.21x slower*)     | `18.45 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `2.47 us` (❌ *32.72x slower*)    | `7.89 us` (❌ *104.54x slower*)    | `310.13 ns` (❌ *4.11x slower*)    | `75.46 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `1.78 us` (❌ *26.85x slower*)    | `5.49 us` (❌ *82.67x slower*)     | `243.36 ns` (❌ *3.67x slower*)    | `66.38 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `52.34 us` (❌ *3.63x slower*)    | `61.35 us` (❌ *4.26x slower*)     | `48.15 us` (❌ *3.34x slower*)     | `14.40 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `5.09 us` (❌ *43.37x slower*)    | `16.12 us` (❌ *137.35x slower*)   | `417.48 ns` (❌ *3.56x slower*)    | `117.35 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `5.03 us` (❌ *30.74x slower*)    | `15.98 us` (❌ *97.69x slower*)    | `643.77 ns` (❌ *3.94x slower*)    | `163.58 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `8.65 ns` (✅ **1.00x**)  | `17.21 ns` (❌ *1.99x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.46 ns` (✅ **1.00x**) | `21.88 ns` (❌ *2.09x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.87 ns` (✅ **1.00x**)  | `4.95 ns` (✅ **1.02x slower**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `4.55 ns` (✅ **1.00x**)  | `4.56 ns` (✅ **1.00x slower**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761

|                                          | `g1projective`            | `g2projective`                   | `fr`                                | `fq`                                | `fq3`                             | `fq6`                             |
|:-----------------------------------------|:--------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `514.05 ns` (✅ **1.00x**) | `513.54 ns` (✅ **1.00x faster**) | `56.32 ns` (🚀 **9.13x faster**)     | `170.37 ns` (🚀 **3.02x faster**)    | `515.50 ns` (✅ **1.00x slower**)  | `1.11 us` (❌ *2.16x slower*)      |
| **`serialize_uncompressed`**             | `694.36 ns` (✅ **1.00x**) | `694.18 ns` (✅ **1.00x faster**) | `56.05 ns` (🚀 **12.39x faster**)    | `171.94 ns` (🚀 **4.04x faster**)    | `515.73 ns` (✅ **1.35x faster**)  | `1.11 us` (❌ *1.60x slower*)      |
| **`deserialize_compressed`**             | `1.59 ms` (✅ **1.00x**)   | `1.59 ms` (✅ **1.00x slower**)   | `93.77 ns` (🚀 **16935.33x faster**) | `338.61 ns` (🚀 **4689.71x faster**) | `1.05 us` (🚀 **1518.74x faster**) | `2.08 us` (🚀 **764.36x faster**)  |
| **`deserialize_compressed_unchecked`**   | `291.85 us` (✅ **1.00x**) | `291.97 us` (✅ **1.00x slower**) | `94.25 ns` (🚀 **3096.59x faster**)  | `338.62 ns` (🚀 **861.87x faster**)  | `1.05 us` (🚀 **279.19x faster**)  | `2.08 us` (🚀 **140.47x faster**)  |
| **`deserialize_uncompressed`**           | `1.30 ms` (✅ **1.00x**)   | `1.30 ms` (✅ **1.00x faster**)   | `94.19 ns` (🚀 **13772.12x faster**) | `338.58 ns` (🚀 **3831.18x faster**) | `1.05 us` (🚀 **1240.85x faster**) | `2.08 us` (🚀 **624.46x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `769.94 ns` (✅ **1.00x**) | `765.76 ns` (✅ **1.01x faster**) | `94.11 ns` (🚀 **8.18x faster**)     | `338.58 ns` (🚀 **2.27x faster**)    | `1.05 us` (❌ *1.36x slower*)      | `2.08 us` (❌ *2.70x slower*)      |

### msm_for_bw6_761

|        | `g1projective`          | `g2projective`                  |
|:-------|:------------------------|:------------------------------- |
|        | `12.58 s` (✅ **1.00x**) | `12.58 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761

|                          | `fr`                     | `fq`                             | `fq3`                             |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `67.07 us` (✅ **1.00x**) | `290.51 us` (❌ *4.33x slower*)   | `6.94 ms` (❌ *103.52x slower*)    |
| **`legendre_for_qr`**    | `31.95 us` (✅ **1.00x**) | `291.76 us` (❌ *9.13x slower*)   | `299.80 us` (❌ *9.38x slower*)    |

### bitwise_operations_for_bw6_761

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `5.03 ns` (✅ **1.00x**)  | `5.12 ns` (✅ **1.02x slower**)    |
| **`from_little-endian_bits`** | `83.08 ns` (✅ **1.00x**) | `182.75 ns` (❌ *2.20x slower*)    |
| **`from_big-endian_bits`**    | `83.24 ns` (✅ **1.00x**) | `182.73 ns` (❌ *2.20x slower*)    |
| **`comparison`**              | `5.08 ns` (✅ **1.00x**)  | `5.07 ns` (✅ **1.00x faster**)    |
| **`equality`**                | `5.67 ns` (✅ **1.00x**)  | `5.75 ns` (✅ **1.01x slower**)    |
| **`is_zero`**                 | `5.21 ns` (✅ **1.00x**)  | `5.34 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761

|                   | `fr`                     | `fq`                              |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `75.02 ns` (✅ **1.00x**) | `310.62 ns` (❌ *4.14x slower*)    |
| **`into_bigint`** | `47.12 ns` (✅ **1.00x**) | `156.68 ns` (❌ *3.33x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

