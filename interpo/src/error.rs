use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct InterpolationError {
    message: String,
}

impl InterpolationError {
    pub fn with_message(message: impl Into<String>) -> Self {
        InterpolationError {
            message: message.into(),
        }
    }
}

impl Display for InterpolationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Interpolation error: {}", self.message)
    }
}

impl Error for InterpolationError {}
