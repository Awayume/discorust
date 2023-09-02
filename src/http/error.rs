// SPDX-FileCopyrightText: 2023 Awayume <dev@awayume.jp>
// SPDX-License-Identifier: Apache-2.0

use std::error::Error;
use std::fmt::{Display, Formatter, Result};


#[derive(Debug)]
pub struct HttpError {
    message: String,
    #[allow(dead_code)]
    code: Option<u16>,
    source: Option<Box<dyn Error>>,
}

impl HttpError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            code: None,
            source: None,
        }
    }

    pub fn from(message: &str, source: Box<dyn Error>) -> Self {
        Self {
            message: message.to_string(),
            code: None,
            source: Some(source),
        }
    }
}

impl Error for HttpError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref())
    }
}

impl Display for HttpError {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        write!(fmt, "HttpError: {}", self.message)?;

        if let Some(ref source) = self.source {
            write!(fmt, "\n Caused by: {}", source)?;
        }

        Ok(())
    }
}


#[derive(Debug)]
pub struct DiscordLoginError {
    message: String,
    source: Option<Box<dyn Error>>,
}

impl DiscordLoginError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            source: None,
        }
    }

    pub fn from(message: &str, source: Box<dyn Error>) -> Self {
        Self {
            message: message.to_string(),
            source: Some(source),
        }
    }
}

impl Error for DiscordLoginError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref())
    }
}

impl Display for DiscordLoginError {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        write!(fmt, "DiscordLoginError: {}", self.message)?;

        if let Some(ref source) = self.source {
            write!(fmt, "\n Caused by: {}", source)?;
        }

        Ok(())
    }
}
