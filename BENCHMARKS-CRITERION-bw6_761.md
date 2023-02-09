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
|        | `2.19 ms` (✅ **1.00x**)          | `2.10 ms` (✅ **1.04x faster**)    |

### arithmetic_for_bw6_761

|                                       | `fr::bigint`             | `fq::bigint`                    | `g1projective`          | `g2projective`                 | `fq3`                             | `fq6`                             | `fq`                              | `fr`                               |
|:--------------------------------------|:-------------------------|:--------------------------------|:------------------------|:-------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                    | `N/A`                           | `5.20 us` (✅ **1.00x**) | `5.37 us` (✅ **1.03x slower**) | `116.15 ns` (🚀 **44.73x faster**) | `246.51 ns` (🚀 **21.08x faster**) | `37.50 ns` (🚀 **138.53x faster**) | `24.47 ns` (🚀 **212.35x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `5.26 us` (✅ **1.00x**) | `5.27 us` (✅ **1.00x slower**) | `110.23 ns` (🚀 **47.72x faster**) | `212.24 ns` (🚀 **24.78x faster**) | `35.38 ns` (🚀 **148.68x faster**) | `19.50 ns` (🚀 **269.72x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `3.65 us` (✅ **1.00x**) | `3.77 us` (✅ **1.03x slower**) | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `3.85 us` (✅ **1.00x**) | `3.70 us` (✅ **1.04x faster**) | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `2.43 us` (✅ **1.00x**) | `2.42 us` (✅ **1.00x faster**) | `88.81 ns` (🚀 **27.32x faster**)  | `180.71 ns` (🚀 **13.42x faster**) | `27.37 ns` (🚀 **88.63x faster**)  | `9.36 ns` (🚀 **259.31x faster**)   |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `1.88 ms` (✅ **1.00x**) | `1.87 ms` (✅ **1.00x faster**) | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `91.61 ns` (❌ *4.22x slower*)     | `159.75 ns` (❌ *7.35x slower*)    | `30.74 ns` (❌ *1.41x slower*)     | `21.73 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `2.72 us` (❌ *31.32x slower*)     | `9.05 us` (❌ *104.01x slower*)    | `337.98 ns` (❌ *3.89x slower*)    | `86.96 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `1.95 us` (❌ *26.50x slower*)     | `6.26 us` (❌ *84.90x slower*)     | `277.33 ns` (❌ *3.76x slower*)    | `73.76 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `64.86 us` (❌ *3.91x slower*)     | `73.01 us` (❌ *4.41x slower*)     | `60.64 us` (❌ *3.66x slower*)     | `16.57 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `5.59 us` (❌ *42.00x slower*)     | `18.17 us` (❌ *136.47x slower*)   | `496.85 ns` (❌ *3.73x slower*)    | `133.13 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `5.52 us` (❌ *28.24x slower*)     | `17.73 us` (❌ *90.64x slower*)    | `711.46 ns` (❌ *3.64x slower*)    | `195.59 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `9.80 ns` (✅ **1.00x**)  | `19.63 ns` (❌ *2.00x slower*)   | `N/A`                   | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `13.29 ns` (✅ **1.00x**) | `26.52 ns` (❌ *2.00x slower*)   | `N/A`                   | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `5.24 ns` (✅ **1.00x**)  | `5.26 ns` (✅ **1.00x slower**)  | `N/A`                   | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `4.74 ns` (✅ **1.00x**)  | `4.68 ns` (✅ **1.01x faster**)  | `N/A`                   | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761

|                                          | `g1projective`            | `g2projective`                   | `fr`                                 | `fq`                                | `fq3`                             | `fq6`                             |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-------------------------------------|:------------------------------------|:----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `585.46 ns` (✅ **1.00x**) | `587.75 ns` (✅ **1.00x slower**) | `62.91 ns` (🚀 **9.31x faster**)      | `196.72 ns` (🚀 **2.98x faster**)    | `578.67 ns` (✅ **1.01x faster**)  | `1.24 us` (❌ *2.12x slower*)      |
| **`serialize_uncompressed`**             | `789.08 ns` (✅ **1.00x**) | `787.06 ns` (✅ **1.00x faster**) | `65.08 ns` (🚀 **12.12x faster**)     | `195.78 ns` (🚀 **4.03x faster**)    | `578.67 ns` (✅ **1.36x faster**)  | `1.25 us` (❌ *1.58x slower*)      |
| **`deserialize_compressed`**             | `1.71 ms` (✅ **1.00x**)   | `1.71 ms` (✅ **1.00x faster**)   | `130.62 ns` (🚀 **13094.20x faster**) | `395.42 ns` (🚀 **4325.50x faster**) | `1.18 us` (🚀 **1446.71x faster**) | `2.38 us` (🚀 **718.41x faster**)  |
| **`deserialize_compressed_unchecked`**   | `315.09 us` (✅ **1.00x**) | `328.06 us` (✅ **1.04x slower**) | `130.97 ns` (🚀 **2405.92x faster**)  | `379.67 ns` (🚀 **829.90x faster**)  | `1.18 us` (🚀 **266.48x faster**)  | `2.38 us` (🚀 **132.35x faster**)  |
| **`deserialize_uncompressed`**           | `1.43 ms` (✅ **1.00x**)   | `1.45 ms` (✅ **1.01x slower**)   | `130.62 ns` (🚀 **10967.28x faster**) | `394.36 ns` (🚀 **3632.59x faster**) | `1.18 us` (🚀 **1209.49x faster**) | `2.39 us` (🚀 **598.70x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `870.83 ns` (✅ **1.00x**) | `871.35 ns` (✅ **1.00x slower**) | `134.19 ns` (🚀 **6.49x faster**)     | `382.06 ns` (🚀 **2.28x faster**)    | `1.23 us` (❌ *1.41x slower*)      | `2.39 us` (❌ *2.75x slower*)      |

### msm_for_bw6_761

|        | `g1projective`          | `g2projective`                  |
|:-------|:------------------------|:------------------------------- |
|        | `13.89 s` (✅ **1.00x**) | `13.91 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761

|                          | `fr`                     | `fq`                             | `fq3`                             |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `81.32 us` (✅ **1.00x**) | `330.19 us` (❌ *4.06x slower*)   | `7.74 ms` (❌ *95.17x slower*)     |
| **`legendre_for_qr`**    | `36.72 us` (✅ **1.00x**) | `315.07 us` (❌ *8.58x slower*)   | `331.56 us` (❌ *9.03x slower*)    |

### bitwise_operations_for_bw6_761

|                               | `fr::bigint`              | `fq::bigint`                      |
|:------------------------------|:--------------------------|:--------------------------------- |
| **`number_of_bits`**          | `5.24 ns` (✅ **1.00x**)   | `5.31 ns` (✅ **1.01x slower**)    |
| **`from_little-endian_bits`** | `144.06 ns` (✅ **1.00x**) | `271.91 ns` (❌ *1.89x slower*)    |
| **`from_big-endian_bits`**    | `138.68 ns` (✅ **1.00x**) | `270.70 ns` (❌ *1.95x slower*)    |
| **`comparison`**              | `5.26 ns` (✅ **1.00x**)   | `5.37 ns` (✅ **1.02x slower**)    |
| **`equality`**                | `5.82 ns` (✅ **1.00x**)   | `6.30 ns` (✅ **1.08x slower**)    |
| **`is_zero`**                 | `5.15 ns` (✅ **1.00x**)   | `5.31 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761

|                   | `fr`                      | `fq`                              |
|:------------------|:--------------------------|:--------------------------------- |
| **`from_bigint`** | `101.48 ns` (✅ **1.00x**) | `364.61 ns` (❌ *3.59x slower*)    |
| **`into_bigint`** | `51.62 ns` (✅ **1.00x**)  | `177.45 ns` (❌ *3.44x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

