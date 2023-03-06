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
    - [pairing_for_bw6_761](#pairing_for_bw6_761)

## Benchmark Results

### sample_bw6_761

|        | `g1projective_elements`          | `g2projective_elements`           |
|:-------|:---------------------------------|:--------------------------------- |
|        | `2.74 ms` (✅ **1.00x**)          | `2.79 ms` (✅ **1.02x slower**)    |

### arithmetic_for_bw6_761

|                                       | `fr::bigint`             | `fq::bigint`                    | `g1projective`          | `g2projective`                 | `fq3`                             | `fq6`                             | `fq`                              | `fr`                               |
|:--------------------------------------|:-------------------------|:--------------------------------|:------------------------|:-------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                    | `N/A`                           | `7.02 us` (✅ **1.00x**) | `6.92 us` (✅ **1.01x faster**) | `118.29 ns` (🚀 **59.35x faster**) | `230.05 ns` (🚀 **30.51x faster**) | `39.68 ns` (🚀 **176.93x faster**) | `24.20 ns` (🚀 **290.10x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `7.03 us` (✅ **1.00x**) | `6.76 us` (✅ **1.04x faster**) | `106.09 ns` (🚀 **66.23x faster**) | `199.30 ns` (🚀 **35.25x faster**) | `36.94 ns` (🚀 **190.20x faster**) | `19.55 ns` (🚀 **359.40x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `4.71 us` (✅ **1.00x**) | `4.81 us` (✅ **1.02x slower**) | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `4.94 us` (✅ **1.00x**) | `4.79 us` (✅ **1.03x faster**) | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `3.15 us` (✅ **1.00x**) | `3.14 us` (✅ **1.00x faster**) | `89.73 ns` (🚀 **35.07x faster**)  | `177.69 ns` (🚀 **17.71x faster**) | `26.80 ns` (🚀 **117.41x faster**) | `13.51 ns` (🚀 **232.94x faster**)  |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `2.51 ms` (✅ **1.00x**) | `2.54 ms` (✅ **1.01x slower**) | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `99.47 ns` (❌ *4.64x slower*)     | `175.43 ns` (❌ *8.18x slower*)    | `31.88 ns` (❌ *1.49x slower*)     | `21.44 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `3.43 us` (❌ *31.10x slower*)     | `10.83 us` (❌ *98.13x slower*)    | `439.96 ns` (❌ *3.99x slower*)    | `110.40 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `2.44 us` (❌ *30.17x slower*)     | `7.54 us` (❌ *93.26x slower*)     | `332.97 ns` (❌ *4.12x slower*)    | `80.88 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `66.98 us` (❌ *4.24x slower*)     | `81.99 us` (❌ *5.19x slower*)     | `61.25 us` (❌ *3.88x slower*)     | `15.80 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `6.98 us` (❌ *50.54x slower*)     | `21.98 us` (❌ *159.21x slower*)   | `495.82 ns` (❌ *3.59x slower*)    | `138.04 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `7.16 us` (❌ *32.12x slower*)     | `21.96 us` (❌ *98.49x slower*)    | `915.60 ns` (❌ *4.11x slower*)    | `223.00 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `24.28 ns` (✅ **1.00x**) | `20.32 ns` (✅ **1.19x faster**) | `N/A`                   | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `14.37 ns` (✅ **1.00x**) | `27.24 ns` (❌ *1.90x slower*)   | `N/A`                   | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)  | `4.89 ns` (✅ **1.04x slower**)  | `N/A`                   | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `4.22 ns` (✅ **1.00x**)  | `4.32 ns` (✅ **1.02x slower**)  | `N/A`                   | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761

|                                          | `g1projective`            | `g2projective`                   | `fr`                                 | `fq`                                | `fq3`                             | `fq6`                             |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-------------------------------------|:------------------------------------|:----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `744.87 ns` (✅ **1.00x**) | `732.52 ns` (✅ **1.02x faster**) | `64.13 ns` (🚀 **11.61x faster**)     | `232.33 ns` (🚀 **3.21x faster**)    | `685.99 ns` (✅ **1.09x faster**)  | `1.43 us` (❌ *1.93x slower*)      |
| **`serialize_uncompressed`**             | `950.54 ns` (✅ **1.00x**) | `941.95 ns` (✅ **1.01x faster**) | `62.65 ns` (🚀 **15.17x faster**)     | `229.44 ns` (🚀 **4.14x faster**)    | `688.00 ns` (✅ **1.38x faster**)  | `1.52 us` (❌ *1.60x slower*)      |
| **`deserialize_compressed`**             | `2.28 ms` (✅ **1.00x**)   | `2.22 ms` (✅ **1.02x faster**)   | `138.86 ns` (🚀 **16399.87x faster**) | `513.66 ns` (🚀 **4433.38x faster**) | `1.54 us` (🚀 **1477.27x faster**) | `3.06 us` (🚀 **744.84x faster**)  |
| **`deserialize_compressed_unchecked`**   | `427.59 us` (✅ **1.00x**) | `413.39 us` (✅ **1.03x faster**) | `137.33 ns` (🚀 **3113.61x faster**)  | `514.42 ns` (🚀 **831.21x faster**)  | `1.53 us` (🚀 **280.14x faster**)  | `3.19 us` (🚀 **134.13x faster**)  |
| **`deserialize_uncompressed`**           | `1.81 ms` (✅ **1.00x**)   | `1.80 ms` (✅ **1.00x faster**)   | `135.78 ns` (🚀 **13336.69x faster**) | `512.77 ns` (🚀 **3531.64x faster**) | `1.53 us` (🚀 **1184.79x faster**) | `3.06 us` (🚀 **592.74x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `1.21 us` (✅ **1.00x**)   | `1.17 us` (✅ **1.03x faster**)   | `135.83 ns` (🚀 **8.87x faster**)     | `531.87 ns` (🚀 **2.27x faster**)    | `1.53 us` (❌ *1.27x slower*)      | `3.02 us` (❌ *2.51x slower*)      |

### msm_for_bw6_761

|        | `g1projective`          | `g2projective`                  |
|:-------|:------------------------|:------------------------------- |
|        | `18.11 s` (✅ **1.00x**) | `18.03 s` (✅ **1.00x faster**)  |

### squareroot_for_bw6_761

|                          | `fr`                     | `fq`                             | `fq3`                             |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `91.20 us` (✅ **1.00x**) | `411.21 us` (❌ *4.51x slower*)   | `9.42 ms` (❌ *103.32x slower*)    |
| **`legendre_for_qr`**    | `44.37 us` (✅ **1.00x**) | `415.02 us` (❌ *9.35x slower*)   | `419.95 us` (❌ *9.47x slower*)    |

### bitwise_operations_for_bw6_761

|                               | `fr::bigint`              | `fq::bigint`                      |
|:------------------------------|:--------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.76 ns` (✅ **1.00x**)   | `4.96 ns` (✅ **1.04x slower**)    |
| **`from_little-endian_bits`** | `130.63 ns` (✅ **1.00x**) | `259.07 ns` (❌ *1.98x slower*)    |
| **`from_big-endian_bits`**    | `129.37 ns` (✅ **1.00x**) | `253.80 ns` (❌ *1.96x slower*)    |
| **`comparison`**              | `4.88 ns` (✅ **1.00x**)   | `6.97 ns` (❌ *1.43x slower*)      |
| **`equality`**                | `5.98 ns` (✅ **1.00x**)   | `6.90 ns` (❌ *1.15x slower*)      |
| **`is_zero`**                 | `4.82 ns` (✅ **1.00x**)   | `4.80 ns` (✅ **1.00x faster**)    |

### conversions_for_bw6_761

|                   | `fr`                      | `fq`                              |
|:------------------|:--------------------------|:--------------------------------- |
| **`from_bigint`** | `110.31 ns` (✅ **1.00x**) | `453.34 ns` (❌ *4.11x slower*)    |
| **`into_bigint`** | `54.17 ns` (✅ **1.00x**)  | `210.87 ns` (❌ *3.89x slower*)    |

### pairing_for_bw6_761

|        | `g1_preparation_for_bw6_761`          | `g2_preparation_for_bw6_761`          | `miller_loop_for_bw6_761`           | `final_exponentiation_for_bw6_761`          | `full_pairing_for_bw6_761`            |
|:-------|:--------------------------------------|:--------------------------------------|:------------------------------------|:--------------------------------------------|:------------------------------------- |
|        | `21.68 ns` (✅ **1.00x**)              | `1.38 ms` (❌ *63841.78x slower*)      | `4.91 ms` (❌ *226392.12x slower*)   | `5.60 ms` (❌ *258235.11x slower*)           | `11.97 ms` (❌ *551998.39x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

