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
|        | `178.22 us` (✅ **1.00x**)        | `1.85 ms` (❌ *10.40x slower*)     |

### arithmetic_for_bls12_377

|                                       | `fr::bigint`            | `fq::bigint`                    | `g1projective`            | `g2projective`                 | `fq2`                             | `fq12`                            | `fq`                             | `fr`                              |
|:--------------------------------------|:------------------------|:--------------------------------|:--------------------------|:-------------------------------|:----------------------------------|:----------------------------------|:---------------------------------|:--------------------------------- |
| **`addition`**                        | `N/A`                   | `N/A`                           | `1.13 us` (✅ **1.00x**)   | `4.45 us` (❌ *3.93x slower*)   | `27.12 ns` (🚀 **41.83x faster**)  | `178.21 ns` (🚀 **6.37x faster**)  | `19.39 ns` (🚀 **58.52x faster**) | `8.28 ns` (🚀 **136.96x faster**)  |
| **`subtraction`**                     | `N/A`                   | `N/A`                           | `1.17 us` (✅ **1.00x**)   | `4.50 us` (❌ *3.84x slower*)   | `27.13 ns` (🚀 **43.19x faster**)  | `169.00 ns` (🚀 **6.93x faster**)  | `14.98 ns` (🚀 **78.19x faster**) | `8.61 ns` (🚀 **136.11x faster**)  |
| **`mixed_addition`**                  | `N/A`                   | `N/A`                           | `833.48 ns` (✅ **1.00x**) | `3.19 us` (❌ *3.82x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`mixed_subtraction`**               | `N/A`                   | `N/A`                           | `859.59 ns` (✅ **1.00x**) | `3.22 us` (❌ *3.75x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`double`**                          | `N/A`                   | `N/A`                           | `572.96 ns` (✅ **1.00x**) | `2.08 us` (❌ *3.62x slower*)   | `12.84 ns` (🚀 **44.61x faster**)  | `103.73 ns` (🚀 **5.52x faster**)  | `7.48 ns` (🚀 **76.56x faster**)  | `9.10 ns` (🚀 **62.94x faster**)   |
| **`scalar_multiplication`**           | `N/A`                   | `N/A`                           | `284.98 us` (✅ **1.00x**) | `1.06 ms` (❌ *3.73x slower*)   | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`negation`**                        | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `22.49 ns` (❌ *3.78x slower*)     | `100.71 ns` (❌ *16.93x slower*)   | `17.10 ns` (❌ *2.88x slower*)    | `5.95 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `267.97 ns` (❌ *7.14x slower*)    | `6.66 us` (❌ *177.57x slower*)    | `68.98 ns` (❌ *1.84x slower*)    | `37.51 ns` (✅ **1.00x**)          |
| **`square`**                          | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `247.89 ns` (❌ *7.79x slower*)    | `4.68 us` (❌ *147.22x slower*)    | `59.40 ns` (❌ *1.87x slower*)    | `31.82 ns` (✅ **1.00x**)          |
| **`inverse`**                         | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `13.96 us` (❌ *2.22x slower*)     | `25.31 us` (❌ *4.03x slower*)     | `13.63 us` (❌ *2.17x slower*)    | `6.28 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `570.01 ns` (❌ *10.74x slower*)   | `13.61 us` (❌ *256.41x slower*)   | `112.50 ns` (❌ *2.12x slower*)   | `53.06 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                   | `N/A`                           | `N/A`                     | `N/A`                          | `554.21 ns` (❌ *6.98x slower*)    | `13.52 us` (❌ *170.23x slower*)   | `156.19 ns` (❌ *1.97x slower*)   | `79.42 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `6.52 ns` (✅ **1.00x**) | `7.84 ns` (❌ *1.20x slower*)    | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`subtraction_with_borrow`**         | `7.84 ns` (✅ **1.00x**) | `10.30 ns` (❌ *1.31x slower*)   | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`multiplication_by_2`**             | `3.89 ns` (✅ **1.00x**) | `4.04 ns` (✅ **1.04x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |
| **`division_by_2`**                   | `3.74 ns` (✅ **1.00x**) | `3.75 ns` (✅ **1.00x slower**)  | `N/A`                     | `N/A`                          | `N/A`                             | `N/A`                             | `N/A`                            | `N/A`                             |

### serialization_for_bls12_377

|                                          | `g1projective`            | `g2projective`                   | `fr`                               | `fq`                               | `fq2`                               | `fq12`                            |
|:-----------------------------------------|:--------------------------|:---------------------------------|:-----------------------------------|:-----------------------------------|:------------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `148.05 ns` (✅ **1.00x**) | `212.68 ns` (❌ *1.44x slower*)   | `27.86 ns` (🚀 **5.31x faster**)    | `50.30 ns` (🚀 **2.94x faster**)    | `100.38 ns` (✅ **1.47x faster**)    | `628.24 ns` (❌ *4.24x slower*)    |
| **`serialize_uncompressed`**             | `197.47 ns` (✅ **1.00x**) | `317.76 ns` (❌ *1.61x slower*)   | `27.83 ns` (🚀 **7.10x faster**)    | `50.03 ns` (🚀 **3.95x faster**)    | `100.40 ns` (🚀 **1.97x faster**)    | `628.46 ns` (❌ *3.18x slower*)    |
| **`deserialize_compressed`**             | `281.25 us` (✅ **1.00x**) | `969.36 us` (❌ *3.45x slower*)   | `46.44 ns` (🚀 **6056.51x faster**) | `93.19 ns` (🚀 **3017.86x faster**) | `208.24 ns` (🚀 **1350.59x faster**) | `1.25 us` (🚀 **224.42x faster**)  |
| **`deserialize_compressed_unchecked`**   | `65.24 us` (✅ **1.00x**)  | `174.20 us` (❌ *2.67x slower*)   | `46.43 ns` (🚀 **1405.00x faster**) | `93.60 ns` (🚀 **697.04x faster**)  | `207.22 ns` (🚀 **314.83x faster**)  | `1.25 us` (🚀 **52.30x faster**)   |
| **`deserialize_uncompressed`**           | `216.00 us` (✅ **1.00x**) | `793.63 us` (❌ *3.67x slower*)   | `46.41 ns` (🚀 **4653.93x faster**) | `93.55 ns` (🚀 **2308.80x faster**) | `208.22 ns` (🚀 **1037.37x faster**) | `1.25 us` (🚀 **172.29x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `226.01 ns` (✅ **1.00x**) | `469.26 ns` (❌ *2.08x slower*)   | `46.40 ns` (🚀 **4.87x faster**)    | `93.57 ns` (🚀 **2.42x faster**)    | `207.24 ns` (✅ **1.09x faster**)    | `1.25 us` (❌ *5.52x slower*)      |

### msm_for_bls12_377

|        | `g1projective`          | `g2projective`                 |
|:-------|:------------------------|:------------------------------ |
|        | `2.25 s` (✅ **1.00x**)  | `7.98 s` (❌ *3.54x slower*)    |

### squareroot_for_bls12_377

|                          | `fr`                     | `fq`                            | `fq2`                             |
|:-------------------------|:-------------------------|:--------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `27.75 us` (✅ **1.00x**) | `65.05 us` (❌ *2.34x slower*)   | `173.40 us` (❌ *6.25x slower*)    |
| **`legendre_for_qr`**    | `9.57 us` (✅ **1.00x**)  | `29.27 us` (❌ *3.06x slower*)   | `29.43 us` (❌ *3.08x slower*)     |

### bitwise_operations_for_bls12_377

|                               | `fr::bigint`             | `fq::bigint`                      |
|:------------------------------|:-------------------------|:--------------------------------- |
| **`number_of_bits`**          | `3.98 ns` (✅ **1.00x**)  | `4.19 ns` (✅ **1.05x slower**)    |
| **`from_little-endian_bits`** | `61.03 ns` (✅ **1.00x**) | `108.77 ns` (❌ *1.78x slower*)    |
| **`from_big-endian_bits`**    | `61.01 ns` (✅ **1.00x**) | `108.68 ns` (❌ *1.78x slower*)    |
| **`comparison`**              | `4.08 ns` (✅ **1.00x**)  | `4.31 ns` (✅ **1.06x slower**)    |
| **`equality`**                | `4.48 ns` (✅ **1.00x**)  | `4.67 ns` (✅ **1.04x slower**)    |
| **`is_zero`**                 | `3.90 ns` (✅ **1.00x**)  | `4.00 ns` (✅ **1.03x slower**)    |

### conversions_for_bls12_377

|                   | `fr`                     | `fq`                             |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `35.98 ns` (✅ **1.00x**) | `78.44 ns` (❌ *2.18x slower*)    |
| **`into_bigint`** | `21.64 ns` (✅ **1.00x**) | `41.49 ns` (❌ *1.92x slower*)    |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

