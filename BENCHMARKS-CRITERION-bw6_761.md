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
|        | `1.50 ms` (✅ **1.00x**)          | `1.68 ms` (❌ *1.12x slower*)      |

### arithmetic_for_bw6_761

|                                       | `fr::bigint`             | `fq::bigint`                    | `g1projective`          | `g2projective`                 | `fq3`                            | `fq6`                             | `fq`                              | `fr`                               |
|:--------------------------------------|:-------------------------|:--------------------------------|:------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                    | `N/A`                           | `3.67 us` (✅ **1.00x**) | `4.16 us` (❌ *1.13x slower*)   | `89.57 ns` (🚀 **40.94x faster**) | `180.36 ns` (🚀 **20.33x faster**) | `30.24 ns` (🚀 **121.29x faster**) | `16.83 ns` (🚀 **217.92x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `3.71 us` (✅ **1.00x**) | `4.20 us` (❌ *1.13x slower*)   | `81.02 ns` (🚀 **45.74x faster**) | `157.25 ns` (🚀 **23.57x faster**) | `24.27 ns` (🚀 **152.70x faster**) | `13.85 ns` (🚀 **267.55x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `2.93 us` (✅ **1.00x**) | `2.93 us` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `2.96 us` (✅ **1.00x**) | `2.96 us` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `1.93 us` (✅ **1.00x**) | `1.93 us` (✅ **1.00x faster**) | `70.44 ns` (🚀 **27.43x faster**) | `142.86 ns` (🚀 **13.53x faster**) | `19.28 ns` (🚀 **100.23x faster**) | `11.15 ns` (🚀 **173.32x faster**)  |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `1.50 ms` (✅ **1.00x**) | `1.50 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `68.05 ns` (❌ *4.01x slower*)    | `123.46 ns` (❌ *7.27x slower*)    | `24.12 ns` (❌ *1.42x slower*)     | `16.99 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `1.90 us` (❌ *26.92x slower*)    | `6.89 us` (❌ *97.51x slower*)     | `271.10 ns` (❌ *3.84x slower*)    | `70.64 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `1.56 us` (❌ *26.46x slower*)    | `4.83 us` (❌ *81.93x slower*)     | `216.72 ns` (❌ *3.68x slower*)    | `58.94 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `50.25 us` (❌ *3.75x slower*)    | `57.97 us` (❌ *4.32x slower*)     | `46.80 us` (❌ *3.49x slower*)     | `13.42 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `4.46 us` (❌ *42.10x slower*)    | `14.01 us` (❌ *132.27x slower*)   | `400.71 ns` (❌ *3.78x slower*)    | `105.91 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `4.41 us` (❌ *31.82x slower*)    | `13.90 us` (❌ *100.30x slower*)   | `568.77 ns` (❌ *4.10x slower*)    | `138.63 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.83 ns` (✅ **1.00x**)  | `15.71 ns` (❌ *2.01x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.74 ns` (✅ **1.00x**) | `21.14 ns` (❌ *1.97x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.04 ns` (✅ **1.00x**)  | `3.60 ns` (✅ **1.12x faster**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**)  | `3.74 ns` (✅ **1.00x slower**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761

|                                          | `g1projective`            | `g2projective`                   | `fr`                                | `fq`                                | `fq3`                               | `fq6`                             |
|:-----------------------------------------|:--------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `412.65 ns` (✅ **1.00x**) | `468.16 ns` (❌ *1.13x slower*)   | `50.30 ns` (🚀 **8.20x faster**)     | `157.04 ns` (🚀 **2.63x faster**)    | `466.40 ns` (❌ *1.13x slower*)      | `871.59 ns` (❌ *2.11x slower*)    |
| **`serialize_uncompressed`**             | `628.72 ns` (✅ **1.00x**) | `630.12 ns` (✅ **1.00x slower**) | `44.22 ns` (🚀 **14.22x faster**)    | `157.87 ns` (🚀 **3.98x faster**)    | `411.35 ns` (✅ **1.53x faster**)    | `987.78 ns` (❌ *1.57x slower*)    |
| **`deserialize_compressed`**             | `1.36 ms` (✅ **1.00x**)   | `1.36 ms` (✅ **1.00x faster**)   | `94.09 ns` (🚀 **14496.82x faster**) | `305.53 ns` (🚀 **4464.34x faster**) | `944.77 ns` (🚀 **1443.72x faster**) | `1.93 us` (🚀 **707.65x faster**)  |
| **`deserialize_compressed_unchecked`**   | `222.46 us` (✅ **1.00x**) | `252.13 us` (❌ *1.13x slower*)   | `93.62 ns` (🚀 **2376.24x faster**)  | `306.86 ns` (🚀 **724.96x faster**)  | `944.83 ns` (🚀 **235.45x faster**)  | `1.93 us` (🚀 **115.41x faster**)  |
| **`deserialize_uncompressed`**           | `1.11 ms` (✅ **1.00x**)   | `1.11 ms` (✅ **1.00x slower**)   | `93.63 ns` (🚀 **11872.14x faster**) | `307.02 ns` (🚀 **3620.67x faster**) | `941.40 ns` (🚀 **1180.82x faster**) | `1.92 us` (🚀 **579.35x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `692.60 ns` (✅ **1.00x**) | `693.16 ns` (✅ **1.00x slower**) | `82.33 ns` (🚀 **8.41x faster**)     | `306.98 ns` (🚀 **2.26x faster**)    | `940.99 ns` (❌ *1.36x slower*)      | `1.69 us` (❌ *2.44x slower*)      |

### msm_for_bw6_761

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `9.78 s` (✅ **1.00x**)  | `9.81 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761

|                          | `fr`                     | `fq`                             | `fq3`                             |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `64.71 us` (✅ **1.00x**) | `250.75 us` (❌ *3.88x slower*)   | `5.99 ms` (❌ *92.54x slower*)     |
| **`legendre_for_qr`**    | `25.73 us` (✅ **1.00x**) | `250.89 us` (❌ *9.75x slower*)   | `256.94 us` (❌ *9.99x slower*)    |

### bitwise_operations_for_bw6_761

|                               | `fr::bigint`              | `fq::bigint`                      |
|:------------------------------|:--------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.19 ns` (✅ **1.00x**)   | `4.25 ns` (✅ **1.01x slower**)    |
| **`from_little-endian_bits`** | `95.05 ns` (✅ **1.00x**)  | `219.07 ns` (❌ *2.30x slower*)    |
| **`from_big-endian_bits`**    | `108.07 ns` (✅ **1.00x**) | `218.87 ns` (❌ *2.03x slower*)    |
| **`comparison`**              | `3.71 ns` (✅ **1.00x**)   | `12.61 ns` (❌ *3.40x slower*)     |
| **`equality`**                | `4.67 ns` (✅ **1.00x**)   | `5.44 ns` (❌ *1.16x slower*)      |
| **`is_zero`**                 | `4.01 ns` (✅ **1.00x**)   | `4.11 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761

|                   | `fr`                     | `fq`                              |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `78.88 ns` (✅ **1.00x**) | `278.24 ns` (❌ *3.53x slower*)    |
| **`into_bigint`** | `41.47 ns` (✅ **1.00x**) | `125.19 ns` (❌ *3.02x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

