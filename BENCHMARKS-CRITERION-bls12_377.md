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
|        | `271.23 us` (✅ **1.00x**)        | `2.63 ms` (❌ *9.70x slower*)      |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`             | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                             | `fq12`                            | `fq`                             | `fr`                               |
|:--------------------------------------|:-------------------------|:--------------------------------|:--------------------------|:-------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                    | `N/A`                           | `1.83 us` (✅ **1.00x**)   | `6.04 us` (❌ *3.30x slower*)   | `34.22 ns` (🚀 **53.47x faster**)  | `228.93 ns` (🚀 **7.99x faster**)  | `24.23 ns` (🚀 **75.50x faster**) | `10.87 ns` (🚀 **168.35x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `1.90 us` (✅ **1.00x**)   | `6.18 us` (❌ *3.26x slower*)   | `34.05 ns` (🚀 **55.73x faster**)  | `223.17 ns` (🚀 **8.50x faster**)  | `19.80 ns` (🚀 **95.85x faster**) | `11.78 ns` (🚀 **161.10x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `1.35 us` (✅ **1.00x**)   | `4.40 us` (❌ *3.25x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `1.40 us` (✅ **1.00x**)   | `4.43 us` (❌ *3.16x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `887.64 ns` (✅ **1.00x**) | `3.02 us` (❌ *3.40x slower*)   | `17.46 ns` (🚀 **50.85x faster**)  | `133.46 ns` (🚀 **6.65x faster**)  | `9.77 ns` (🚀 **90.87x faster**)  | `10.69 ns` (🚀 **83.03x faster**)   |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `457.32 us` (✅ **1.00x**) | `1.48 ms` (❌ *3.23x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `27.44 ns` (❌ *3.37x slower*)     | `136.27 ns` (❌ *16.75x slower*)   | `22.00 ns` (❌ *2.70x slower*)    | `8.14 ns` (✅ **1.00x**)            |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `334.03 ns` (❌ *7.51x slower*)    | `8.25 us` (❌ *185.49x slower*)    | `92.43 ns` (❌ *2.08x slower*)    | `44.48 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `370.17 ns` (❌ *9.69x slower*)    | `5.82 us` (❌ *152.39x slower*)    | `81.60 ns` (❌ *2.14x slower*)    | `38.19 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `17.56 us` (❌ *2.52x slower*)     | `31.01 us` (❌ *4.44x slower*)     | `17.22 us` (❌ *2.47x slower*)    | `6.98 us` (✅ **1.00x**)            |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `718.15 ns` (❌ *11.20x slower*)   | `17.07 us` (❌ *266.26x slower*)   | `147.27 ns` (❌ *2.30x slower*)   | `64.12 ns` (✅ **1.00x**)           |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `682.91 ns` (❌ *6.76x slower*)    | `17.22 us` (❌ *170.46x slower*)   | `241.41 ns` (❌ *2.39x slower*)   | `101.01 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.95 ns` (✅ **1.00x**)  | `10.94 ns` (❌ *1.38x slower*)   | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.29 ns` (✅ **1.00x**) | `14.84 ns` (❌ *1.44x slower*)   | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`multiplication_by_2`**             | `4.49 ns` (✅ **1.00x**)  | `4.85 ns` (✅ **1.08x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`division_by_2`**                   | `4.27 ns` (✅ **1.00x**)  | `4.25 ns` (✅ **1.00x faster**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                                | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `193.42 ns` (✅ **1.00x**) | `286.46 ns` (❌ *1.48x slower*)   | `37.41 ns` (🚀 **5.17x faster**)    | `64.06 ns` (🚀 **3.02x faster**)     | `120.40 ns` (✅ **1.61x faster**)    | `773.64 ns` (❌ *4.00x slower*)    |
| **`serialize_uncompressed`**             | `265.17 ns` (✅ **1.00x**) | `419.32 ns` (❌ *1.58x slower*)   | `36.13 ns` (🚀 **7.34x faster**)    | `64.46 ns` (🚀 **4.11x faster**)     | `119.77 ns` (🚀 **2.21x faster**)    | `800.12 ns` (❌ *3.02x slower*)    |
| **`deserialize_compressed`**             | `425.81 us` (✅ **1.00x**) | `1.40 ms` (❌ *3.29x slower*)     | `57.15 ns` (🚀 **7450.42x faster**) | `122.80 ns` (🚀 **3467.51x faster**) | `299.22 ns` (🚀 **1423.05x faster**) | `1.76 us` (🚀 **241.84x faster**)  |
| **`deserialize_compressed_unchecked`**   | `92.61 us` (✅ **1.00x**)  | `259.98 us` (❌ *2.81x slower*)   | `56.26 ns` (🚀 **1646.10x faster**) | `122.57 ns` (🚀 **755.58x faster**)  | `296.82 ns` (🚀 **312.02x faster**)  | `1.83 us` (🚀 **50.60x faster**)   |
| **`deserialize_uncompressed`**           | `329.79 us` (✅ **1.00x**) | `1.15 ms` (❌ *3.48x slower*)     | `56.92 ns` (🚀 **5794.36x faster**) | `116.96 ns` (🚀 **2819.53x faster**) | `292.90 ns` (🚀 **1125.95x faster**) | `1.77 us` (🚀 **186.17x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `322.56 ns` (✅ **1.00x**) | `660.89 ns` (❌ *2.05x slower*)   | `57.67 ns` (🚀 **5.59x faster**)    | `121.41 ns` (🚀 **2.66x faster**)    | `277.70 ns` (✅ **1.16x faster**)    | `1.74 us` (❌ *5.40x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                  |
|:-------|:------------------------|:------------------------------- |
|        | `3.50 s` (✅ **1.00x**)  | `10.80 s` (❌ *3.09x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `35.42 us` (✅ **1.00x**) | `90.87 us` (❌ *2.57x slower*)   | `253.77 us` (❌ *7.16x slower*)    |
| **`legendre_for_qr`**    | `11.82 us` (✅ **1.00x**) | `46.36 us` (❌ *3.92x slower*)   | `45.27 us` (❌ *3.83x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.41 ns` (✅ **1.00x**)  | `4.92 ns` (❌ *1.12x slower*)      |
| **`from_little-endian_bits`** | `77.67 ns` (✅ **1.00x**) | `129.96 ns` (❌ *1.67x slower*)    |
| **`from_big-endian_bits`**    | `72.12 ns` (✅ **1.00x**) | `132.92 ns` (❌ *1.84x slower*)    |
| **`comparison`**              | `4.52 ns` (✅ **1.00x**)  | `4.81 ns` (✅ **1.06x slower**)    |
| **`equality`**                | `4.96 ns` (✅ **1.00x**)  | `5.92 ns` (❌ *1.19x slower*)      |
| **`is_zero`**                 | `4.42 ns` (✅ **1.00x**)  | `4.84 ns` (✅ **1.10x slower**)    |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `45.40 ns` (✅ **1.00x**) | `95.58 ns` (❌ *2.11x slower*)    |
| **`into_bigint`** | `28.33 ns` (✅ **1.00x**) | `55.04 ns` (❌ *1.94x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

