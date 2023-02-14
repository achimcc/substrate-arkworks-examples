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
|        | `208.76 us` (✅ **1.00x**)        | `2.18 ms` (❌ *10.45x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`             | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                             | `fq12`                            | `fq`                             | `fr`                               |
|:--------------------------------------|:-------------------------|:--------------------------------|:--------------------------|:-------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                    | `N/A`                           | `1.39 us` (✅ **1.00x**)   | `4.99 us` (❌ *3.59x slower*)   | `30.67 ns` (🚀 **45.35x faster**)  | `213.98 ns` (🚀 **6.50x faster**)  | `22.74 ns` (🚀 **61.16x faster**) | `10.14 ns` (🚀 **137.11x faster**)  |
| **`subtraction`**                     | `N/A`                    | `N/A`                           | `1.45 us` (✅ **1.00x**)   | `5.17 us` (❌ *3.56x slower*)   | `32.62 ns` (🚀 **44.56x faster**)  | `205.82 ns` (🚀 **7.06x faster**)  | `18.90 ns` (🚀 **76.92x faster**) | `10.79 ns` (🚀 **134.73x faster**)  |
| **`mixed_addition`**                  | `N/A`                    | `N/A`                           | `976.04 ns` (✅ **1.00x**) | `3.68 us` (❌ *3.77x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                    | `N/A`                           | `1.03 us` (✅ **1.00x**)   | `3.80 us` (❌ *3.68x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`double`**                          | `N/A`                    | `N/A`                           | `679.34 ns` (✅ **1.00x**) | `2.47 us` (❌ *3.63x slower*)   | `15.70 ns` (🚀 **43.26x faster**)  | `136.80 ns` (🚀 **4.97x faster**)  | `8.87 ns` (🚀 **76.60x faster**)  | `10.62 ns` (🚀 **64.00x faster**)   |
| **`scalar_multiplication`**           | `N/A`                    | `N/A`                           | `339.30 us` (✅ **1.00x**) | `1.31 ms` (❌ *3.85x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`negation`**                        | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `27.02 ns` (❌ *3.61x slower*)     | `133.42 ns` (❌ *17.84x slower*)   | `21.10 ns` (❌ *2.82x slower*)    | `7.48 ns` (✅ **1.00x**)            |
| **`multiplication`**                  | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `304.13 ns` (❌ *6.48x slower*)    | `8.23 us` (❌ *175.43x slower*)    | `83.53 ns` (❌ *1.78x slower*)    | `46.93 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `295.19 ns` (❌ *7.91x slower*)    | `5.63 us` (❌ *150.78x slower*)    | `74.43 ns` (❌ *1.99x slower*)    | `37.34 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `15.41 us` (❌ *2.26x slower*)     | `30.36 us` (❌ *4.46x slower*)     | `16.08 us` (❌ *2.36x slower*)    | `6.81 us` (✅ **1.00x**)            |
| **`sum_of_products_of_size_2`**       | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `672.17 ns` (❌ *10.82x slower*)   | `16.37 us` (❌ *263.61x slower*)   | `140.89 ns` (❌ *2.27x slower*)   | `62.11 ns` (✅ **1.00x**)           |
| **`naive_sum_of_products_of_size_2`** | `N/A`                    | `N/A`                           | `N/A`                     | `N/A`                          | `639.04 ns` (❌ *6.72x slower*)    | `17.18 us` (❌ *180.58x slower*)   | `191.91 ns` (❌ *2.02x slower*)   | `95.13 ns` (✅ **1.00x**)           |
| **`addition_with_carry`**             | `7.93 ns` (✅ **1.00x**)  | `10.69 ns` (❌ *1.35x slower*)   | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.08 ns` (✅ **1.00x**) | `13.51 ns` (❌ *1.34x slower*)   | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`multiplication_by_2`**             | `4.55 ns` (✅ **1.00x**)  | `4.97 ns` (✅ **1.09x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |
| **`division_by_2`**                   | `4.32 ns` (✅ **1.00x**)  | `4.44 ns` (✅ **1.03x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                              |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                                | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:------------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `183.08 ns` (✅ **1.00x**) | `264.35 ns` (❌ *1.44x slower*)   | `34.77 ns` (🚀 **5.26x faster**)    | `59.87 ns` (🚀 **3.06x faster**)     | `115.79 ns` (✅ **1.58x faster**)    | `771.53 ns` (❌ *4.21x slower*)    |
| **`serialize_uncompressed`**             | `251.05 ns` (✅ **1.00x**) | `394.97 ns` (❌ *1.57x slower*)   | `36.24 ns` (🚀 **6.93x faster**)    | `61.14 ns` (🚀 **4.11x faster**)     | `114.24 ns` (🚀 **2.20x faster**)    | `775.63 ns` (❌ *3.09x slower*)    |
| **`deserialize_compressed`**             | `353.77 us` (✅ **1.00x**) | `1.12 ms` (❌ *3.18x slower*)     | `55.66 ns` (🚀 **6355.67x faster**) | `113.13 ns` (🚀 **3127.24x faster**) | `243.20 ns` (🚀 **1454.64x faster**) | `1.60 us` (🚀 **221.08x faster**)  |
| **`deserialize_compressed_unchecked`**   | `85.75 us` (✅ **1.00x**)  | `217.10 us` (❌ *2.53x slower*)   | `56.14 ns` (🚀 **1527.36x faster**) | `115.64 ns` (🚀 **741.51x faster**)  | `267.86 ns` (🚀 **320.14x faster**)  | `1.63 us` (🚀 **52.47x faster**)   |
| **`deserialize_uncompressed`**           | `273.71 us` (✅ **1.00x**) | `933.45 us` (❌ *3.41x slower*)   | `57.38 ns` (🚀 **4769.82x faster**) | `111.04 ns` (🚀 **2464.99x faster**) | `249.71 ns` (🚀 **1096.13x faster**) | `1.63 us` (🚀 **168.41x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `279.84 ns` (✅ **1.00x**) | `619.21 ns` (❌ *2.21x slower*)   | `56.57 ns` (🚀 **4.95x faster**)    | `109.93 ns` (🚀 **2.55x faster**)    | `242.31 ns` (✅ **1.15x faster**)    | `1.62 us` (❌ *5.78x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.71 s` (✅ **1.00x**)  | `9.54 s` (❌ *3.52x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `32.54 us` (✅ **1.00x**) | `81.37 us` (❌ *2.50x slower*)   | `226.58 us` (❌ *6.96x slower*)    |
| **`legendre_for_qr`**    | `11.85 us` (✅ **1.00x**) | `38.17 us` (❌ *3.22x slower*)   | `39.06 us` (❌ *3.30x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `4.62 ns` (✅ **1.00x**)  | `4.99 ns` (✅ **1.08x slower**)    |
| **`from_little-endian_bits`** | `71.36 ns` (✅ **1.00x**) | `134.30 ns` (❌ *1.88x slower*)    |
| **`from_big-endian_bits`**    | `74.52 ns` (✅ **1.00x**) | `132.38 ns` (❌ *1.78x slower*)    |
| **`comparison`**              | `4.73 ns` (✅ **1.00x**)  | `5.01 ns` (✅ **1.06x slower**)    |
| **`equality`**                | `5.02 ns` (✅ **1.00x**)  | `5.93 ns` (❌ *1.18x slower*)      |
| **`is_zero`**                 | `4.37 ns` (✅ **1.00x**)  | `4.53 ns` (✅ **1.04x slower**)    |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `41.92 ns` (✅ **1.00x**) | `92.92 ns` (❌ *2.22x slower*)    |
| **`into_bigint`** | `27.23 ns` (✅ **1.00x**) | `48.84 ns` (❌ *1.79x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

