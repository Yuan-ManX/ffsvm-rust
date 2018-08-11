use std::{
    num::{ParseFloatError, ParseIntError},
    option::NoneError,
};

use pest::Error;

/// Possible error types when classifying with a [SVM].
#[derive(Debug)]
pub enum SVMError {
    /// This can be emitted when creating a [SVM] from a [ModelFile]. For models generated by
    /// libSVM's `svm-train`, the most common reason this occurs is skipping attributes.
    /// All attributes must be in sequential order 0, 1, 2, ..., n. If they are not, this
    /// error will be emitted. For more details see the documentation provided in [ModelFile].
    AttributesUnordered {
        /// The index process that was not a direct successor of the previous index. Can be used for
        /// easier debugging the model file.
        index: u32,

        /// The value of the given index. Can be used for debugging in conjunction with `index`.
        value: f32,

        /// The last index processed. If everything were alright, then `index` should equal
        /// `last_index + 1`.
        last_index: u32,
    },

    /// This error can be emitted by [Predict::predict_probability()] in case the model loaded by
    /// [ModelFile] was not trained with probability estimates (`svm-train -b 1`).
    NoProbabilities,

    /// Can be emitted by [Predict::predict_probability()] when predicting probabilities
    /// and the internal iteration limit was exceeded.
    IterationsExceeded,

    /// If the model does not have a `gamma` set this error may be raised.
    NoGamma,

    /// If the model does not have a `coef0` set this error may be raised.
    NoCoef0,

    /// If the model does not have a `degree` set this error may be raised.
    NoDegree,

    /// Wrapper for [ModelError] when unifiying error handling.
    ParsingError(String),
}

// impl<'a, T> From<Error<'a, T>> for SVMError {
//     fn from(_: Error<'a, T>) -> Self {
//         SVMError::ParsingError
//     }
// }

impl<'a> From<pest::Error<'a, crate::parser::Rule>> for SVMError {
    fn from(e: pest::Error<'a, crate::parser::Rule>) -> Self { SVMError::ParsingError(format!("{}", e)) }
}

impl From<NoneError> for SVMError {
    fn from(_: NoneError) -> Self { SVMError::ParsingError("NoneError".to_owned()) }
}

impl From<ParseFloatError> for SVMError {
    fn from(_e: ParseFloatError) -> Self { SVMError::ParsingError("ParseFloatError".to_owned()) }
}

impl From<ParseIntError> for SVMError {
    fn from(_: ParseIntError) -> Self { SVMError::ParsingError("ParseIntError".to_owned()) }
}
