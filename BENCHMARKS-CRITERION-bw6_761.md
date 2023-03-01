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
|        | `1.90 ms` (✅ **1.00x**)          | `1.97 ms` (✅ **1.04x slower**)    |

### arithmetic_for_bw6_761

|                                       | `fr::bigint`             | `fq::bigint`                    | `g1projective`          | `g2projective`                 | `fq3`                            | `fq6`                             | `fq`                              | `fr`                               |
|:--------------------------------------|:-------------------------|:--------------------------------|:------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                    | `N/A`                           | `4.26 us` (✅ **1.00x**) | `4.76 us` (❌ *1.12x slower*)   | `99.97 ns` (🚀 **42.64x faster**) | `184.67 ns` (🚀 **23.08x faster**) | `31.02 ns` (🚀 **137.41x faster**) | `20.68 ns` (🚀 **206.16x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `4.29 us` (✅ **1.00x**) | `4.71 us` (✅ **1.10x slower**) | `86.18 ns` (🚀 **49.81x faster**) | `165.58 ns` (🚀 **25.93x faster**) | `29.53 ns` (🚀 **145.37x faster**) | `16.05 ns` (🚀 **267.45x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `3.12 us` (✅ **1.00x**) | `3.23 us` (✅ **1.04x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `3.03 us` (✅ **1.00x**) | `3.62 us` (❌ *1.20x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `2.03 us` (✅ **1.00x**) | `2.14 us` (✅ **1.06x slower**) | `73.75 ns` (🚀 **27.48x faster**) | `152.74 ns` (🚀 **13.27x faster**) | `23.88 ns` (🚀 **84.87x faster**)  | `11.54 ns` (🚀 **175.64x faster**)  |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `1.52 ms` (✅ **1.00x**) | `1.66 ms` (✅ **1.09x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `71.30 ns` (❌ *3.99x slower*)    | `131.21 ns` (❌ *7.35x slower*)    | `25.31 ns` (❌ *1.42x slower*)     | `17.85 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `2.30 us` (❌ *28.96x slower*)    | `7.41 us` (❌ *93.40x slower*)     | `297.94 ns` (❌ *3.75x slower*)    | `79.36 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `1.66 us` (❌ *27.27x slower*)    | `5.60 us` (❌ *91.79x slower*)     | `225.71 ns` (❌ *3.70x slower*)    | `60.99 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `55.84 us` (❌ *3.60x slower*)    | `60.82 us` (❌ *3.92x slower*)     | `48.63 us` (❌ *3.13x slower*)     | `15.53 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `5.07 us` (❌ *43.58x slower*)    | `14.70 us` (❌ *126.33x slower*)   | `471.10 ns` (❌ *4.05x slower*)    | `116.39 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `4.61 us` (❌ *26.12x slower*)    | `14.48 us` (❌ *82.07x slower*)    | `593.01 ns` (❌ *3.36x slower*)    | `176.37 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `8.08 ns` (✅ **1.00x**)  | `16.47 ns` (❌ *2.04x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `11.06 ns` (✅ **1.00x**) | `22.45 ns` (❌ *2.03x slower*)   | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.27 ns` (✅ **1.00x**)  | `4.25 ns` (✅ **1.01x faster**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `3.92 ns` (✅ **1.00x**)  | `3.87 ns` (✅ **1.01x faster**)  | `N/A`                   | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761

|                                          | `g1projective`            | `g2projective`                   | `fr`                                | `fq`                                | `fq3`                               | `fq6`                             |
|:-----------------------------------------|:--------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `526.16 ns` (✅ **1.00x**) | `524.43 ns` (✅ **1.00x faster**) | `52.32 ns` (🚀 **10.06x faster**)    | `163.66 ns` (🚀 **3.22x faster**)    | `487.74 ns` (✅ **1.08x faster**)    | `1.04 us` (❌ *1.97x slower*)      |
| **`serialize_uncompressed`**             | `666.93 ns` (✅ **1.00x**) | `746.76 ns` (❌ *1.12x slower*)   | `54.65 ns` (🚀 **12.20x faster**)    | `162.71 ns` (🚀 **4.10x faster**)    | `480.78 ns` (✅ **1.39x faster**)    | `1.04 us` (❌ *1.56x slower*)      |
| **`deserialize_compressed`**             | `1.39 ms` (✅ **1.00x**)   | `1.58 ms` (❌ *1.14x slower*)     | `99.40 ns` (🚀 **13986.15x faster**) | `323.91 ns` (🚀 **4291.80x faster**) | `1.03 us` (🚀 **1347.12x faster**)   | `2.09 us` (🚀 **664.19x faster**)  |
| **`deserialize_compressed_unchecked`**   | `253.36 us` (✅ **1.00x**) | `276.60 us` (✅ **1.09x slower**) | `102.21 ns` (🚀 **2478.85x faster**) | `331.93 ns` (🚀 **763.31x faster**)  | `998.66 ns` (🚀 **253.70x faster**)  | `2.12 us` (🚀 **119.67x faster**)  |
| **`deserialize_uncompressed`**           | `1.13 ms` (✅ **1.00x**)   | `1.36 ms` (❌ *1.21x slower*)     | `99.85 ns` (🚀 **11275.12x faster**) | `318.33 ns` (🚀 **3536.64x faster**) | `990.71 ns` (🚀 **1136.39x faster**) | `1.99 us` (🚀 **565.69x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `736.91 ns` (✅ **1.00x**) | `783.15 ns` (✅ **1.06x slower**) | `107.55 ns` (🚀 **6.85x faster**)    | `354.61 ns` (🚀 **2.08x faster**)    | `1.01 us` (❌ *1.38x slower*)        | `2.11 us` (❌ *2.86x slower*)      |

### msm_for_bw6_761

|        | `g1projective`          | `g2projective`                  |
|:-------|:------------------------|:------------------------------- |
|        | `11.86 s` (✅ **1.00x**) | `12.29 s` (✅ **1.04x slower**)  |

### squareroot_for_bw6_761

|                          | `fr`                     | `fq`                             | `fq3`                             |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `68.27 us` (✅ **1.00x**) | `262.23 us` (❌ *3.84x slower*)   | `6.42 ms` (❌ *94.08x slower*)     |
| **`legendre_for_qr`**    | `30.74 us` (✅ **1.00x**) | `270.81 us` (❌ *8.81x slower*)   | `273.30 us` (❌ *8.89x slower*)    |

### bitwise_operations_for_bw6_761

|                               | `fr::bigint`              | `fq::bigint`                      |
|:------------------------------|:--------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.60 ns` (✅ **1.00x**)   | `4.35 ns` (✅ **1.06x faster**)    |
| **`from_little-endian_bits`** | `112.11 ns` (✅ **1.00x**) | `226.13 ns` (❌ *2.02x slower*)    |
| **`from_big-endian_bits`**    | `112.66 ns` (✅ **1.00x**) | `237.95 ns` (❌ *2.11x slower*)    |
| **`comparison`**              | `4.34 ns` (✅ **1.00x**)   | `4.45 ns` (✅ **1.03x slower**)    |
| **`equality`**                | `5.52 ns` (✅ **1.00x**)   | `5.98 ns` (✅ **1.08x slower**)    |
| **`is_zero`**                 | `4.31 ns` (✅ **1.00x**)   | `4.30 ns` (✅ **1.00x faster**)    |

### conversions_for_bw6_761

|                   | `fr`                     | `fq`                              |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `83.20 ns` (✅ **1.00x**) | `292.42 ns` (❌ *3.51x slower*)    |
| **`into_bigint`** | `43.54 ns` (✅ **1.00x**) | `149.91 ns` (❌ *3.44x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

