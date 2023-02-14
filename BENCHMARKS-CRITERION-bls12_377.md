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
|        | `194.99 us` (✅ **1.00x**)        | `2.04 ms` (❌ *10.44x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                            | `fq12`                            | `fq`                              | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.25 us` (✅ **1.00x**)   | `4.80 us` (❌ *3.84x slower*)   | `23.27 ns` (🚀 **53.75x faster**) | `181.78 ns` (🚀 **6.88x faster**)  | `12.52 ns` (🚀 **99.88x faster**)  | `8.71 ns` (🚀 **143.58x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.30 us` (✅ **1.00x**)   | `4.86 us` (❌ *3.74x slower*)   | `23.25 ns` (🚀 **55.82x faster**) | `159.43 ns` (🚀 **8.14x faster**)  | `12.74 ns` (🚀 **101.89x faster**) | `8.80 ns` (🚀 **147.46x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `904.19 ns` (✅ **1.00x**) | `3.44 us` (❌ *3.80x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `941.08 ns` (✅ **1.00x**) | `3.47 us` (❌ *3.69x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `590.80 ns` (✅ **1.00x**) | `2.27 us` (❌ *3.84x slower*)   | `12.27 ns` (🚀 **48.14x faster**) | `67.39 ns` (🚀 **8.77x faster**)   | `7.14 ns` (🚀 **82.77x faster**)   | `5.92 ns` (🚀 **99.83x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `325.68 us` (✅ **1.00x**) | `1.16 ms` (❌ *3.58x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `22.84 ns` (❌ *3.71x slower*)    | `94.89 ns` (❌ *15.40x slower*)    | `18.77 ns` (❌ *3.05x slower*)     | `6.16 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `270.60 ns` (❌ *6.27x slower*)   | `7.11 us` (❌ *164.58x slower*)    | `76.09 ns` (❌ *1.76x slower*)     | `43.18 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `243.07 ns` (❌ *6.68x slower*)   | `5.01 us` (❌ *137.60x slower*)    | `66.39 ns` (❌ *1.82x slower*)     | `36.41 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `15.12 us` (❌ *2.14x slower*)    | `27.46 us` (❌ *3.89x slower*)     | `14.80 us` (❌ *2.09x slower*)     | `7.07 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `587.19 ns` (❌ *9.53x slower*)   | `14.57 us` (❌ *236.56x slower*)   | `117.62 ns` (❌ *1.91x slower*)    | `61.61 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `577.44 ns` (❌ *6.44x slower*)   | `14.50 us` (❌ *161.87x slower*)   | `163.49 ns` (❌ *1.82x slower*)    | `89.61 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**) | `8.62 ns` (❌ *1.13x slower*)    | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.71 ns` (✅ **1.00x**) | `10.32 ns` (❌ *1.18x slower*)   | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.87 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`division_by_2`**                   | `4.55 ns` (✅ **1.00x**) | `4.55 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `158.52 ns` (✅ **1.00x**) | `219.68 ns` (❌ *1.39x slower*)   | `30.65 ns` (🚀 **5.17x faster**)    | `56.67 ns` (🚀 **2.80x faster**)    | `110.38 ns` (✅ **1.44x faster**)    | `698.76 ns` (❌ *4.41x slower*)    |
| **`serialize_uncompressed`**             | `215.32 ns` (✅ **1.00x**) | `333.22 ns` (❌ *1.55x slower*)   | `31.13 ns` (🚀 **6.92x faster**)    | `55.74 ns` (🚀 **3.86x faster**)    | `110.03 ns` (🚀 **1.96x faster**)    | `698.84 ns` (❌ *3.25x slower*)    |
| **`deserialize_compressed`**             | `315.17 us` (✅ **1.00x**) | `1.06 ms` (❌ *3.37x slower*)     | `52.55 ns` (🚀 **5997.66x faster**) | `92.27 ns` (🚀 **3415.76x faster**) | `210.42 ns` (🚀 **1497.83x faster**) | `1.28 us` (🚀 **246.06x faster**)  |
| **`deserialize_compressed_unchecked`**   | `68.28 us` (✅ **1.00x**)  | `184.52 us` (❌ *2.70x slower*)   | `52.49 ns` (🚀 **1300.86x faster**) | `92.30 ns` (🚀 **739.77x faster**)  | `210.29 ns` (🚀 **324.71x faster**)  | `1.28 us` (🚀 **53.37x faster**)   |
| **`deserialize_uncompressed`**           | `247.27 us` (✅ **1.00x**) | `875.55 us` (❌ *3.54x slower*)   | `52.40 ns` (🚀 **4719.10x faster**) | `92.15 ns` (🚀 **2683.42x faster**) | `209.75 ns` (🚀 **1178.90x faster**) | `1.27 us` (🚀 **194.40x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `222.43 ns` (✅ **1.00x**) | `471.54 ns` (❌ *2.12x slower*)   | `52.39 ns` (🚀 **4.25x faster**)    | `92.14 ns` (🚀 **2.41x faster**)    | `209.62 ns` (✅ **1.06x faster**)    | `1.28 us` (❌ *5.75x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.38 s` (✅ **1.00x**)  | `8.32 s` (❌ *3.50x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.13 us` (✅ **1.00x**) | `67.88 us` (❌ *2.18x slower*)   | `183.07 us` (❌ *5.88x slower*)    |
| **`legendre_for_qr`**    | `10.95 us` (✅ **1.00x**) | `32.03 us` (❌ *2.93x slower*)   | `32.07 us` (❌ *2.93x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.84 ns` (✅ **1.00x**)  | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.23 ns` (✅ **1.00x**) | `89.52 ns` (❌ *1.86x slower*)    |
| **`from_big-endian_bits`**    | `48.12 ns` (✅ **1.00x**) | `90.71 ns` (❌ *1.89x slower*)    |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)  | `5.13 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.37 ns` (✅ **1.00x**)  | `5.67 ns` (✅ **1.06x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `5.21 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.79 ns` (✅ **1.00x**) | `75.19 ns` (❌ *1.84x slower*)    |
| **`into_bigint`** | `22.79 ns` (✅ **1.00x**) | `47.08 ns` (❌ *2.07x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

