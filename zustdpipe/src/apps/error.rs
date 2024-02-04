use std::borrow::Cow;
use std::fmt;

/// Error in case the definition of an App is invalid

#[derive(Debug)]
pub enum AppDefinitionError {
    Serdeyaml(serde_yaml::Error),
}

impl AppDefinitionError {
    pub(crate) fn as_str(&self) -> Cow<String> {
        use AppDefinitionError::*;
        // Strictly alphabetical, please.  (Sadly rustfmt cannot do this yet.)
        match &*self {
            Serdeyaml(err) => Cow::Owned(format!(
                "Invalid App definition. Error in Yaml file: {}",
                err.to_string()
            )),
        }
    }
}

/// Display a proper error message for invalid app definiton
impl fmt::Display for AppDefinitionError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.as_str().into_owned().as_str())
    }
}
