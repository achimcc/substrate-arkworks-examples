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
|        | `193.32 us` (✅ **1.00x**)        | `2.03 ms` (❌ *10.49x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                            | `fq12`                            | `fq`                              | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.26 us` (✅ **1.00x**)   | `4.78 us` (❌ *3.79x slower*)   | `23.17 ns` (🚀 **54.36x faster**) | `179.72 ns` (🚀 **7.01x faster**)  | `12.49 ns` (🚀 **100.85x faster**) | `8.80 ns` (🚀 **143.18x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.29 us` (✅ **1.00x**)   | `4.83 us` (❌ *3.76x slower*)   | `23.21 ns` (🚀 **55.45x faster**) | `162.96 ns` (🚀 **7.90x faster**)  | `12.76 ns` (🚀 **100.89x faster**) | `8.81 ns` (🚀 **146.06x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `895.14 ns` (✅ **1.00x**) | `3.42 us` (❌ *3.82x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `915.76 ns` (✅ **1.00x**) | `3.45 us` (❌ *3.77x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `607.11 ns` (✅ **1.00x**) | `2.27 us` (❌ *3.73x slower*)   | `12.28 ns` (🚀 **49.44x faster**) | `67.47 ns` (🚀 **9.00x faster**)   | `7.16 ns` (🚀 **84.73x faster**)   | `5.87 ns` (🚀 **103.38x faster**)  |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `316.40 us` (✅ **1.00x**) | `1.16 ms` (❌ *3.66x slower*)   | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `23.07 ns` (❌ *3.76x slower*)    | `95.48 ns` (❌ *15.56x slower*)    | `18.79 ns` (❌ *3.06x slower*)     | `6.14 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `270.28 ns` (❌ *6.25x slower*)   | `7.10 us` (❌ *164.21x slower*)    | `74.60 ns` (❌ *1.72x slower*)     | `43.25 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `250.14 ns` (❌ *6.89x slower*)   | `5.01 us` (❌ *138.00x slower*)    | `66.55 ns` (❌ *1.83x slower*)     | `36.29 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `15.12 us` (❌ *2.15x slower*)    | `27.44 us` (❌ *3.90x slower*)     | `14.80 us` (❌ *2.10x slower*)     | `7.04 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `588.48 ns` (❌ *9.59x slower*)   | `14.55 us` (❌ *237.09x slower*)   | `118.18 ns` (❌ *1.93x slower*)    | `61.37 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `575.91 ns` (❌ *6.46x slower*)   | `14.47 us` (❌ *162.25x slower*)   | `163.52 ns` (❌ *1.83x slower*)    | `89.15 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `7.62 ns` (✅ **1.00x**) | `8.64 ns` (❌ *1.13x slower*)    | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`subtraction_with_borrow`**         | `8.66 ns` (✅ **1.00x**) | `10.31 ns` (❌ *1.19x slower*)   | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**) | `4.87 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |
| **`division_by_2`**                   | `4.53 ns` (✅ **1.00x**) | `4.55 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                             |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `157.28 ns` (✅ **1.00x**) | `220.49 ns` (❌ *1.40x slower*)   | `32.48 ns` (🚀 **4.84x faster**)    | `58.03 ns` (🚀 **2.71x faster**)    | `109.73 ns` (✅ **1.43x faster**)    | `703.89 ns` (❌ *4.48x slower*)    |
| **`serialize_uncompressed`**             | `211.74 ns` (✅ **1.00x**) | `332.78 ns` (❌ *1.57x slower*)   | `31.52 ns` (🚀 **6.72x faster**)    | `55.83 ns` (🚀 **3.79x faster**)    | `110.09 ns` (🚀 **1.92x faster**)    | `708.00 ns` (❌ *3.34x slower*)    |
| **`deserialize_compressed`**             | `311.28 us` (✅ **1.00x**) | `1.06 ms` (❌ *3.40x slower*)     | `52.54 ns` (🚀 **5924.82x faster**) | `92.03 ns` (🚀 **3382.45x faster**) | `208.97 ns` (🚀 **1489.56x faster**) | `1.28 us` (🚀 **242.34x faster**)  |
| **`deserialize_compressed_unchecked`**   | `67.65 us` (✅ **1.00x**)  | `182.87 us` (❌ *2.70x slower*)   | `52.57 ns` (🚀 **1286.84x faster**) | `92.10 ns` (🚀 **734.56x faster**)  | `208.95 ns` (🚀 **323.78x faster**)  | `1.29 us` (🚀 **52.53x faster**)   |
| **`deserialize_uncompressed`**           | `243.72 us` (✅ **1.00x**) | `871.81 us` (❌ *3.58x slower*)   | `52.49 ns` (🚀 **4643.48x faster**) | `92.05 ns` (🚀 **2647.71x faster**) | `208.74 ns` (🚀 **1167.59x faster**) | `1.29 us` (🚀 **189.20x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `226.79 ns` (✅ **1.00x**) | `471.21 ns` (❌ *2.08x slower*)   | `52.48 ns` (🚀 **4.32x faster**)    | `92.00 ns` (🚀 **2.47x faster**)    | `208.71 ns` (✅ **1.09x faster**)    | `1.29 us` (❌ *5.68x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.34 s` (✅ **1.00x**)  | `8.30 s` (❌ *3.55x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `31.07 us` (✅ **1.00x**) | `67.23 us` (❌ *2.16x slower*)   | `181.94 us` (❌ *5.86x slower*)    |
| **`legendre_for_qr`**    | `10.89 us` (✅ **1.00x**) | `31.33 us` (❌ *2.88x slower*)   | `31.34 us` (❌ *2.88x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                     |
|:------------------------------|:-------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.84 ns` (✅ **1.00x**)  | `5.02 ns` (✅ **1.04x slower**)   |
| **`from_little-endian_bits`** | `48.87 ns` (✅ **1.00x**) | `90.36 ns` (❌ *1.85x slower*)    |
| **`from_big-endian_bits`**    | `48.86 ns` (✅ **1.00x**) | `90.46 ns` (❌ *1.85x slower*)    |
| **`comparison`**              | `4.87 ns` (✅ **1.00x**)  | `5.12 ns` (✅ **1.05x slower**)   |
| **`equality`**                | `5.46 ns` (✅ **1.00x**)  | `5.67 ns` (✅ **1.04x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)  | `5.22 ns` (✅ **1.06x slower**)   |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.77 ns` (✅ **1.00x**) | `74.85 ns` (❌ *1.84x slower*)    |
| **`into_bigint`** | `23.78 ns` (✅ **1.00x**) | `47.01 ns` (❌ *1.98x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

