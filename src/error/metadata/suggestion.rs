use std::fmt::{self, Display};

use ansi_term::Colour::{Cyan, Yellow};

use crate::env::RoverEnvKey;

/// `Suggestion` contains possible suggestions for remedying specific errors.
#[derive(Debug)]
pub enum Suggestion {
    SubmitIssue,
    RerunWithSensitive,
    SetConfigHome,
    MigrateConfigHomeOrCreateConfig,
    CreateConfig,
    ListProfiles,
}

impl Display for Suggestion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let suggestion = match self {
            Suggestion::SubmitIssue => {
                format!("This error was unexpected! Please submit an issue with any relevant details about what you were trying to do: {}", Cyan.normal().paint("https://github.com/apollographql/rover/issues/new"))
            }
            Suggestion::RerunWithSensitive => {
                format!(
                    "Try re-running this command with the {} flag",
                    Yellow.normal().paint("'--sensitive'")
                )
            }
            Suggestion::SetConfigHome => {
                format!(
                    "You can override this path by setting the {} environment variable.",
                    Yellow.normal().paint(RoverEnvKey::ConfigHome.to_string())
                )
            }
            Suggestion::MigrateConfigHomeOrCreateConfig => {
                format!("If you've recently changed the {} environment variable, you may need to migrate your old configuration directory to the new path. Otherwise, try setting up a new configuration profile by running {}.",
                Yellow.normal().paint(RoverEnvKey::ConfigHome.to_string()),
                Yellow.normal().paint("`rover config auth`"))
            }
            Suggestion::CreateConfig => {
                format!(
                    "Try setting up a configuration profile by running {}",
                    Yellow.normal().paint("`rover config auth`")
                )
            }
            Suggestion::ListProfiles => {
                format!(
                    "Try running {} to see the possible values for the {} argument.",
                    Yellow.normal().paint("`rover config list`"),
                    Yellow.normal().paint("'--profile'")
                )
            }
        };
        write!(formatter, "{}", &suggestion)
    }
}