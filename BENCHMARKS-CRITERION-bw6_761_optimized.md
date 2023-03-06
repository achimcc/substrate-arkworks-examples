# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [sample_bw6_761_optimized](#sample_bw6_761_optimized)
    - [arithmetic_for_bw6_761_optimized](#arithmetic_for_bw6_761_optimized)
    - [serialization_for_bw6_761_optimized](#serialization_for_bw6_761_optimized)
    - [msm_for_bw6_761_optimized](#msm_for_bw6_761_optimized)
    - [squareroot_for_bw6_761_optimized](#squareroot_for_bw6_761_optimized)
    - [bitwise_operations_for_bw6_761_optimized](#bitwise_operations_for_bw6_761_optimized)
    - [conversions_for_bw6_761_optimized](#conversions_for_bw6_761_optimized)
    - [pairing_for_bw6_761optimized](#pairing_for_bw6_761optimized)

## Benchmark Results

### sample_bw6_761_optimized

|        | `g1projectivebw6_761_elements`          | `g2projectivebw6_761_elements`           |
|:-------|:----------------------------------------|:---------------------------------------- |
|        | `2.02 ms` (✅ **1.00x**)                 | `2.01 ms` (✅ **1.00x faster**)           |

### arithmetic_for_bw6_761_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`           | `g1projectivebw6_761`          | `g2projectivebw6_761`          | `fq3optimized`                   | `fq6optimized`                    | `fqoptimized`                     | `froptimized`                      |
|:--------------------------------------|:-------------------------------|:--------------------------------|:-------------------------------|:-------------------------------|:---------------------------------|:----------------------------------|:----------------------------------|:---------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                           | `4.80 us` (✅ **1.00x**)        | `4.81 us` (✅ **1.00x slower**) | `79.15 ns` (🚀 **60.70x faster**) | `161.56 ns` (🚀 **29.74x faster**) | `27.80 ns` (🚀 **172.84x faster**) | `12.49 ns` (🚀 **384.52x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                           | `4.87 us` (✅ **1.00x**)        | `4.88 us` (✅ **1.00x slower**) | `79.40 ns` (🚀 **61.38x faster**) | `155.16 ns` (🚀 **31.41x faster**) | `26.11 ns` (🚀 **186.65x faster**) | `13.15 ns` (🚀 **370.68x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                           | `3.36 us` (✅ **1.00x**)        | `3.37 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                           | `3.40 us` (✅ **1.00x**)        | `3.40 us` (✅ **1.00x slower**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`double`**                          | `N/A`                          | `N/A`                           | `2.22 us` (✅ **1.00x**)        | `2.23 us` (✅ **1.00x slower**) | `54.77 ns` (🚀 **40.60x faster**) | `122.48 ns` (🚀 **18.15x faster**) | `19.56 ns` (🚀 **113.66x faster**) | `7.14 ns` (🚀 **311.29x faster**)   |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                           | `1.81 ms` (✅ **1.00x**)        | `1.80 ms` (✅ **1.00x faster**) | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`negation`**                        | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `73.23 ns` (❌ *3.89x slower*)    | `119.56 ns` (❌ *6.35x slower*)    | `22.85 ns` (❌ *1.21x slower*)     | `18.83 ns` (✅ **1.00x**)           |
| **`multiplication`**                  | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `2.49 us` (❌ *32.72x slower*)    | `7.91 us` (❌ *104.14x slower*)    | `306.74 ns` (❌ *4.04x slower*)    | `76.00 ns` (✅ **1.00x**)           |
| **`square`**                          | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `1.78 us` (❌ *26.81x slower*)    | `5.54 us` (❌ *83.29x slower*)     | `246.57 ns` (❌ *3.71x slower*)    | `66.50 ns` (✅ **1.00x**)           |
| **`inverse`**                         | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `51.79 us` (❌ *3.60x slower*)    | `60.88 us` (❌ *4.23x slower*)     | `47.69 us` (❌ *3.32x slower*)     | `14.38 us` (✅ **1.00x**)           |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `5.13 us` (❌ *43.67x slower*)    | `16.16 us` (❌ *137.66x slower*)   | `418.67 ns` (❌ *3.57x slower*)    | `117.37 ns` (✅ **1.00x**)          |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                           | `N/A`                          | `N/A`                          | `5.07 us` (❌ *30.95x slower*)    | `16.07 us` (❌ *98.12x slower*)    | `650.70 ns` (❌ *3.97x slower*)    | `163.83 ns` (✅ **1.00x**)          |
| **`addition_with_carry`**             | `8.64 ns` (✅ **1.00x**)        | `17.17 ns` (❌ *1.99x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`subtraction_with_borrow`**         | `10.35 ns` (✅ **1.00x**)       | `21.90 ns` (❌ *2.12x slower*)   | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`multiplication_by_2`**             | `4.87 ns` (✅ **1.00x**)        | `4.95 ns` (✅ **1.02x slower**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |
| **`division_by_2`**                   | `4.54 ns` (✅ **1.00x**)        | `4.53 ns` (✅ **1.00x faster**)  | `N/A`                          | `N/A`                          | `N/A`                            | `N/A`                             | `N/A`                             | `N/A`                              |

### serialization_for_bw6_761_optimized

|                                          | `g1projectivebw6_761`          | `g2projectivebw6_761`            | `froptimized`                       | `fqoptimized`                       | `fq3optimized`                    | `fq6optimized`                    |
|:-----------------------------------------|:-------------------------------|:---------------------------------|:------------------------------------|:------------------------------------|:----------------------------------|:--------------------------------- |
| **`serialize_compressed`**               | `515.37 ns` (✅ **1.00x**)      | `515.30 ns` (✅ **1.00x faster**) | `56.02 ns` (🚀 **9.20x faster**)     | `168.79 ns` (🚀 **3.05x faster**)    | `513.66 ns` (✅ **1.00x faster**)  | `1.05 us` (❌ *2.04x slower*)      |
| **`serialize_uncompressed`**             | `694.44 ns` (✅ **1.00x**)      | `690.79 ns` (✅ **1.01x faster**) | `55.66 ns` (🚀 **12.48x faster**)    | `169.17 ns` (🚀 **4.10x faster**)    | `513.37 ns` (✅ **1.35x faster**)  | `1.05 us` (❌ *1.51x slower*)      |
| **`deserialize_compressed`**             | `1.59 ms` (✅ **1.00x**)        | `1.59 ms` (✅ **1.00x slower**)   | `93.40 ns` (🚀 **17049.08x faster**) | `340.98 ns` (🚀 **4669.92x faster**) | `1.05 us` (🚀 **1518.03x faster**) | `2.09 us` (🚀 **761.00x faster**)  |
| **`deserialize_compressed_unchecked`**   | `292.03 us` (✅ **1.00x**)      | `291.96 us` (✅ **1.00x faster**) | `93.14 ns` (🚀 **3135.21x faster**)  | `341.01 ns` (🚀 **856.36x faster**)  | `1.05 us` (🚀 **278.39x faster**)  | `2.09 us` (🚀 **139.77x faster**)  |
| **`deserialize_uncompressed`**           | `1.30 ms` (✅ **1.00x**)        | `1.30 ms` (✅ **1.00x faster**)   | `93.33 ns` (🚀 **13931.99x faster**) | `340.80 ns` (🚀 **3815.37x faster**) | `1.05 us` (🚀 **1239.53x faster**) | `2.09 us` (🚀 **622.45x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `740.83 ns` (✅ **1.00x**)      | `742.01 ns` (✅ **1.00x slower**) | `93.53 ns` (🚀 **7.92x faster**)     | `340.81 ns` (🚀 **2.17x faster**)    | `1.05 us` (❌ *1.42x slower*)      | `2.09 us` (❌ *2.82x slower*)      |

### msm_for_bw6_761_optimized

|        | `g1projectivebw6_761`          | `g2projectivebw6_761`           |
|:-------|:-------------------------------|:------------------------------- |
|        | `12.36 s` (✅ **1.00x**)        | `12.40 s` (✅ **1.00x slower**)  |

### squareroot_for_bw6_761_optimized

|                          | `froptimized`            | `fqoptimized`                    | `fq3optimized`                    |
|:-------------------------|:-------------------------|:---------------------------------|:--------------------------------- |
| **`square_root_for_qr`** | `68.07 us` (✅ **1.00x**) | `290.38 us` (❌ *4.27x slower*)   | `6.97 ms` (❌ *102.35x slower*)    |
| **`legendre_for_qr`**    | `32.15 us` (✅ **1.00x**) | `290.62 us` (❌ *9.04x slower*)   | `296.94 us` (❌ *9.24x slower*)    |

### bitwise_operations_for_bw6_761_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`             |
|:------------------------------|:-------------------------------|:--------------------------------- |
| **`number_of_bits`**          | `5.03 ns` (✅ **1.00x**)        | `5.13 ns` (✅ **1.02x slower**)    |
| **`from_little-endian_bits`** | `83.60 ns` (✅ **1.00x**)       | `176.45 ns` (❌ *2.11x slower*)    |
| **`from_big-endian_bits`**    | `86.43 ns` (✅ **1.00x**)       | `167.67 ns` (❌ *1.94x slower*)    |
| **`comparison`**              | `5.08 ns` (✅ **1.00x**)        | `5.09 ns` (✅ **1.00x slower**)    |
| **`equality`**                | `5.65 ns` (✅ **1.00x**)        | `5.76 ns` (✅ **1.02x slower**)    |
| **`is_zero`**                 | `5.21 ns` (✅ **1.00x**)        | `5.35 ns` (✅ **1.03x slower**)    |

### conversions_for_bw6_761_optimized

|                   | `froptimized`            | `fqoptimized`                     |
|:------------------|:-------------------------|:--------------------------------- |
| **`from_bigint`** | `75.37 ns` (✅ **1.00x**) | `309.07 ns` (❌ *4.10x slower*)    |
| **`into_bigint`** | `47.04 ns` (✅ **1.00x**) | `159.04 ns` (❌ *3.38x slower*)    |

### pairing_for_bw6_761optimized

|        | `g1_preparation_for_bw6_761optimized`          | `g2_preparation_for_bw6_761optimized`          | `miller_loop_for_bw6_761optimized`          | `final_exponentiation_for_bw6_761optimized`          | `full_pairing_for_bw6_761optimized`           |
|:-------|:-----------------------------------------------|:-----------------------------------------------|:--------------------------------------------|:-----------------------------------------------------|:--------------------------------------------- |
|        | `18.54 ns` (✅ **1.00x**)                       | `12.59 ns` (✅ **1.47x faster**)                | `4.52 ms` (❌ *243538.50x slower*)           | `4.21 ms` (❌ *227099.23x slower*)                    | `8.74 ms` (❌ *471119.12x slower*)             |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

