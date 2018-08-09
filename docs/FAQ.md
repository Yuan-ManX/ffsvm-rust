
# FAQ




## General

### Why `ffsvm-rust`? What is the problem with `libsvm`?

First, in many cases there is nothing wrong with  [libsvm](https://github.com/cjlin1/libsvm). If time is not an issue, `libsvm` is probably the better, more flexible, choice.

However, when using `libsvm` in real-time applications (games!), a number of problems become noticable:

* it does lots of small allocation per classification call
* data is stored all over the place (e.g., pointers to pointers to a single value)
* there is no bulk classification

`ffsvm-rust` tries to address that by:

* being zero-allocation during classification
* packing all data SIMD-friendly, and using SIMD intrinsics whereever reasonable
* offering a bulk classifification call
* making use of all available CPU cores for classification
* being designed and measured, from day 1, for speed


However, `libsvm` still has nice, portable tools for training and grid search. The ultimate plan for `ffsvm-rust` is not to replace these, but to use their output.


## Usage

### How can I use a trained `libsvm` model?

In order to use a trained `libsvm` model, it needs to fulfill these requirements:

* `svm_type` must be `c_svc`
* `kernel_type` must be `rbf`
* All attributes must start with index `0`, there must be no sparse attributes.




## Development

### How do I enable AVX2 support?

If using the Fish shell run:

`set -g -x RUSTFLAGS "-C target-feature=+avx2"`

Also make sure to modify `utils.rs` and set `SIMD_F32_WIDTH` and `SIMD_F64_WIDTH`.


### Is going full `f32` worth it, and what about classification accuracy?

From using `Instruments` and looking at the performance results it seems most time is spent in the kernel. The kernel uses already `f32` and the numbers don't seem to deviate much from `libsvm`.

Changing the "lower parts" (computing decision values) to `f32` does not seem to give much performance (compare commit `e656296`), but decision values start to notably differ from `libsvm`.