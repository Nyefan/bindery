use std::fmt::Display;

#[derive(Debug)]
pub enum BinderyError {
    GPUIError(gpui::private::anyhow::Error),
}

impl Display for BinderyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GPUIError(err) => write!(f, "{}", err),
        }
    }
}

impl From<gpui::private::anyhow::Error> for BinderyError {
    fn from(err: gpui::private::anyhow::Error) -> Self {
        Self::GPUIError(err)
    }
}
