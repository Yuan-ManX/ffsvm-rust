[![Latest Version]][crates.io]
[![Travis-CI Status]][travis]
[![docs]][docs.rs]
![MIT]

# In One Sentence

You trained a non-sparse RBF-C-SVM using [libSVM](https://github.com/cjlin1/libsvm), now you want the highest possible performance during (real-time) classification, like games or VR.



# Highlights

* can load trained [libSVM](https://github.com/cjlin1/libsvm) models (currently  RBF-SVM without sparse attributes)
* optimized for [SIMD](https://github.com/rust-lang/rfcs/pull/2366) and can be mixed seamlessly with [Rayon](https://github.com/rayon-rs/rayon).
* allocation-free during classification
* written in 100% Rust, but can be loaded from any language (via FFI)
* 2.5x - 14x faster than libSVM
* Free of `unsafe` code ;)


# Principal Usage

Train with [libSVM](https://github.com/cjlin1/libsvm) (e.g., using the tool `svm-train`), then classify with `ffsvm-rust`.

From Rust:

```rust
// Load model file / SVM.
let model_str: &str = include_str!("model.libsvm");
let model = ModelFile::try_from(model_str)?;
let svm = RbfSVM::try_from(&model)?;

// Produce problem we want to classify.
let mut problem = Problem::from(&svm);

// Set features
problem.features_mut().clone_from_slice(&[
    0.3093766, 0.0, 0.0, 0.0, 0.0, 0.1764706, 0.0, 0.0, 1.0, 0.1137485,
]);

// Can be trivially parallelized (e.g., with Rayon) ...
svm.predict_value(&mut problem);

// Results should match libSVM
assert_eq!(42, problem.label);
```

From C / FFI:

Please see [FFSVM-FFI](https://github.com/ralfbiedert/ffsvm-ffi)


# Status

* **Aug 5, 2018**: Still in alpha, but finally on crates.io.
* **May 27, 2018**: We're in alpha. Successfully used internally on Windows, Mac, Android and Linux
on various machines and devices. Once SIMD stabilizes and we can cross-compile to WASM
we'll move to beta.
* **December 16, 2017**: We're in pre-alpha. It will probably not even work on your machine.


# Performance

![performance](docs/performance_relative.v3.png)

Classification time vs. libSVM.

![performance](docs/performance_history.v4.png)

Performance milestones during development.

[See here for details.](https://github.com/ralfbiedert/ffsvm-rust/blob/master/docs/performance.adoc)



# FAQ

[See here for details.](https://github.com/ralfbiedert/ffsvm-rust/blob/master/docs/FAQ.adoc)


[travis]: https://travis-ci.org/ralfbiedert/ffsvm-rust
[Travis-CI Status]: https://travis-ci.org/ralfbiedert/ffsvm-rust.svg?branch=master
[Latest Version]: https://img.shields.io/crates/v/ffsvm.svg
[crates.io]: https://crates.io/crates/ffsvm
[MIT]: https://img.shields.io/badge/license-MIT-blue.svg
[docs]: https://docs.rs/ffsvm/badge.svg
[docs.rs]: https://docs.rs/ffsvm/
