use std::collections::HashMap;

use crate::tools::Runner;
use crate::utils::{self, CommandOutput};

use anyhow::{anyhow, Context, Result};
use camino::{Utf8Path, Utf8PathBuf};

pub(crate) struct MakeRunner {
    runner: Runner,
    rover_exe: Utf8PathBuf,
}

impl MakeRunner {
    pub(crate) fn new(verbose: bool, rover_exe: Utf8PathBuf) -> Result<Self> {
        let runner = Runner::new("make", verbose)?;

        Ok(MakeRunner { runner, rover_exe })
    }

    pub(crate) fn test_supergraph_demo(&self, base_dir: &Utf8Path) -> Result<()> {
        let mut env = HashMap::new();
        env.insert("ROVER_BIN".to_string(), self.rover_exe.to_string());
        let output = self.runner.exec(&["ci"], base_dir, Some(&env))?;
        assert_demo_includes(&output)
            .with_context(|| "There were problems with the output of 'make ci'.")?;
        utils::info("successfully ran supergraph-demo with a local binary.");
        Ok(())
    }
}

fn assert_demo_includes(output: &CommandOutput) -> Result<()> {
    let necessary_stdout = vec!["🚀 Graph Router ready at http://localhost:4000/"];
    let necessary_stderr = vec!["allProducts", "Removing network"];

    let mut missing_strings = Vec::with_capacity(necessary_stderr.len() + necessary_stdout.len());
    for necessary_string in necessary_stdout {
        if !output.stdout.contains(necessary_string) {
            missing_strings.push(necessary_string);
        }
    }
    for necessary_string in necessary_stderr {
        if !output.stderr.contains(necessary_string) {
            missing_strings.push(necessary_string);
        }
    }

    if missing_strings.is_empty() {
        Ok(())
    } else {
        Err(anyhow!(
            "The output from 'make` is missing the following strings: {:?}",
            missing_strings
        ))
    }
}
