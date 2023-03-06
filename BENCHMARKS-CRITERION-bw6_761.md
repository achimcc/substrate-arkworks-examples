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
|        | `2.78 ms` (✅ **1.00x**)          | `2.76 ms` (✅ **1.01x faster**)    |

### arithmetic_for_bw6_761

|                                       | `fr::bigint`             | `fq::bigint`                    | `g1projective`          | `g2projective`                 | `fq3`                             | `fq6`                             | `fq`                              | `fr`                               |
|:--------------------------------------|:-------------------------|:--------------------------------|:------------------------|:-------------------------------|:----------------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                    | `N/A`                           | `6.98 us` (✅ **1.00x**) | `6.85 us` (✅ **1.02x faster**) | `119.16 ns` (🚀 **58.57x faster**) | `238.19 ns` (🚀 **29.30x faster**) | `38.75 ns` (🚀 **180.10x faster**) | `24.64 ns` (🚀 **283.26x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `7.05 us` (✅ **1.00x**) | `6.97 us` (✅ **1.01x faster**) | `105.75 ns` (🚀 **66.63x faster**) | `203.32 ns` (🚀 **34.65x faster**) | `35.99 ns` (🚀 **195.75x faster**) | `20.66 ns` (🚀 **341.08x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `4.73 us` (✅ **1.00x**) | `4.75 us` (✅ **1.00x slower**) | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `4.80 us` (✅ **1.00x**) | `4.84 us` (✅ **1.01x slower**) | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `3.09 us` (✅ **1.00x**) | `3.13 us` (✅ **1.01x slower**) | `89.03 ns` (🚀 **34.68x faster**)  | `181.14 ns` (🚀 **17.05x faster**) | `28.00 ns` (🚀 **110.30x faster**) | `13.56 ns` (🚀 **227.66x faster**)  |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `2.45 ms` (✅ **1.00x**) | `2.47 ms` (✅ **1.01x slower**) | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `99.48 ns` (❌ *4.54x slower*)     | `175.72 ns` (❌ *8.02x slower*)    | `32.84 ns` (❌ *1.50x slower*)     | `21.91 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `3.45 us` (❌ *38.30x slower*)     | `11.03 us` (❌ *122.64x slower*)   | `448.41 ns` (❌ *4.98x slower*)    | `89.97 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `2.48 us` (❌ *31.19x slower*)     | `7.47 us` (❌ *93.85x slower*)     | `334.89 ns` (❌ *4.21x slower*)    | `79.58 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `70.72 us` (❌ *4.44x slower*)     | `81.08 us` (❌ *5.09x slower*)     | `62.41 us` (❌ *3.92x slower*)     | `15.92 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `6.93 us` (❌ *52.08x slower*)     | `22.27 us` (❌ *167.37x slower*)   | `502.20 ns` (❌ *3.77x slower*)    | `133.08 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                   | `N/A`                          | `6.92 us` (❌ *31.26x slower*)     | `21.78 us` (❌ *98.47x slower*)    | `941.83 ns` (❌ *4.26x slower*)    | `221.20 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `22.89 ns` (✅ **1.00x**) | `21.34 ns` (✅ **1.07x faster**) | `N/A`                   | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `14.48 ns` (✅ **1.00x**) | `26.95 ns` (❌ *1.86x slower*)   | `N/A`                   | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.81 ns` (✅ **1.00x**)  | `4.90 ns` (✅ **1.02x slower**)  | `N/A`                   | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `4.33 ns` (✅ **1.00x**)  | `4.30 ns` (✅ **1.01x faster**)  | `N/A`                   | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761

|                                          | `g1projective`            | `g2projective`                   | `fr`                                 | `fq`                                | `fq3`                             | `fq6`                             |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-------------------------------------|:------------------------------------|:----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `699.68 ns` (✅ **1.00x**) | `702.31 ns` (✅ **1.00x slower**) | `64.12 ns` (🚀 **10.91x faster**)     | `236.57 ns` (🚀 **2.96x faster**)    | `688.73 ns` (✅ **1.02x faster**)  | `1.46 us` (❌ *2.09x slower*)      |
| **`serialize_uncompressed`**             | `944.41 ns` (✅ **1.00x**) | `930.70 ns` (✅ **1.01x faster**) | `63.20 ns` (🚀 **14.94x faster**)     | `237.92 ns` (🚀 **3.97x faster**)    | `693.24 ns` (✅ **1.36x faster**)  | `1.44 us` (❌ *1.53x slower*)      |
| **`deserialize_compressed`**             | `2.20 ms` (✅ **1.00x**)   | `2.20 ms` (✅ **1.00x faster**)   | `131.79 ns` (🚀 **16713.67x faster**) | `512.45 ns` (🚀 **4298.28x faster**) | `1.59 us` (🚀 **1388.86x faster**) | `3.08 us` (🚀 **714.47x faster**)  |
| **`deserialize_compressed_unchecked`**   | `419.34 us` (✅ **1.00x**) | `409.83 us` (✅ **1.02x faster**) | `134.20 ns` (🚀 **3124.68x faster**)  | `516.20 ns` (🚀 **812.36x faster**)  | `1.57 us` (🚀 **266.94x faster**)  | `3.05 us` (🚀 **137.39x faster**)  |
| **`deserialize_uncompressed`**           | `1.80 ms` (✅ **1.00x**)   | `1.80 ms` (✅ **1.00x slower**)   | `129.52 ns` (🚀 **13896.48x faster**) | `505.97 ns` (🚀 **3557.32x faster**) | `1.58 us` (🚀 **1138.94x faster**) | `3.06 us` (🚀 **587.89x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `1.08 us` (✅ **1.00x**)   | `1.09 us` (✅ **1.01x slower**)   | `129.84 ns` (🚀 **8.32x faster**)     | `507.73 ns` (🚀 **2.13x faster**)    | `1.60 us` (❌ *1.48x slower*)      | `3.07 us` (❌ *2.85x slower*)      |

### msm_for_bw6_761

|        | `g1projective`          | `g2projective`                  |
|:-------|:------------------------|:------------------------------- |
|        | `18.07 s` (✅ **1.00x**) | `17.93 s` (✅ **1.01x faster**)  |

### squareroot_for_bw6_761

|                          | `fr`                     | `fq`                             | `fq3`                             |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `93.22 us` (✅ **1.00x**) | `414.91 us` (❌ *4.45x slower*)   | `9.42 ms` (❌ *101.06x slower*)    |
| **`legendre_for_qr`**    | `46.79 us` (✅ **1.00x**) | `421.80 us` (❌ *9.02x slower*)   | `416.18 us` (❌ *8.90x slower*)    |

### bitwise_operations_for_bw6_761

|                               | `fr::bigint`              | `fq::bigint`                      |
|:------------------------------|:--------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.82 ns` (✅ **1.00x**)   | `5.02 ns` (✅ **1.04x slower**)    |
| **`from_little-endian_bits`** | `135.50 ns` (✅ **1.00x**) | `255.86 ns` (❌ *1.89x slower*)    |
| **`from_big-endian_bits`**    | `130.55 ns` (✅ **1.00x**) | `259.35 ns` (❌ *1.99x slower*)    |
| **`comparison`**              | `4.87 ns` (✅ **1.00x**)   | `7.09 ns` (❌ *1.45x slower*)      |
| **`equality`**                | `5.82 ns` (✅ **1.00x**)   | `6.97 ns` (❌ *1.20x slower*)      |
| **`is_zero`**                 | `4.77 ns` (✅ **1.00x**)   | `4.81 ns` (✅ **1.01x slower**)    |

### conversions_for_bw6_761

|                   | `fr`                     | `fq`                              |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `96.79 ns` (✅ **1.00x**) | `462.19 ns` (❌ *4.78x slower*)    |
| **`into_bigint`** | `53.76 ns` (✅ **1.00x**) | `218.10 ns` (❌ *4.06x slower*)    |

### pairing_for_bw6_761

|        | `g1_preparation_for_bw6_761`          | `g2_preparation_for_bw6_761`          | `miller_loop_for_bw6_761`           | `final_exponentiation_for_bw6_761`          | `full_pairing_for_bw6_761`            |
|:-------|:--------------------------------------|:--------------------------------------|:------------------------------------|:--------------------------------------------|:------------------------------------- |
|        | `21.64 ns` (✅ **1.00x**)              | `1.37 ms` (❌ *63440.01x slower*)      | `4.93 ms` (❌ *227992.32x slower*)   | `5.78 ms` (❌ *266930.92x slower*)           | `11.98 ms` (❌ *553423.56x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

