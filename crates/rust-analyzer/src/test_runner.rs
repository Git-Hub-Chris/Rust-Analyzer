//! This module provides the functionality needed to run `cargo test` in a background
//! thread and report the result of each test in a channel.

use crossbeam_channel::Sender;
use paths::AbsPath;
use project_model::TargetKind;
use serde::Deserialize as _;
use serde_derive::Deserialize;
use toolchain::Tool;

use crate::{
    command::{CargoParser, CommandHandle},
    flycheck::CargoOptions,
};

#[derive(Debug, Deserialize)]
#[serde(tag = "event", rename_all = "camelCase")]
pub(crate) enum TestState {
    Started,
    Ok,
    Ignored,
    Failed {
        // the stdout field is not always present depending on cargo test flags
        #[serde(skip_serializing_if = "String::is_empty", default)]
        stdout: String,
    },
}

#[derive(Debug)]
pub(crate) struct CargoTestMessage {
    pub target: TestTarget,
    pub output: CargoTestOutput,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub(crate) enum CargoTestOutput {
    Test {
        name: String,
        #[serde(flatten)]
        state: TestState,
    },
    Suite,
    Finished,
    Custom {
        text: String,
    },
}

pub(crate) struct CargoTestOutputParser {
    pub target: TestTarget,
}

impl CargoTestOutputParser {
    pub(crate) fn new(test_target: &TestTarget) -> Self {
        Self { target: test_target.clone() }
    }
}

impl CargoParser<CargoTestMessage> for CargoTestOutputParser {
    fn from_line(&self, line: &str, _error: &mut String) -> Option<CargoTestMessage> {
        let mut deserializer = serde_json::Deserializer::from_str(line);
        deserializer.disable_recursion_limit();

        Some(CargoTestMessage {
            target: self.target.clone(),
            output: if let Ok(message) = CargoTestOutput::deserialize(&mut deserializer) {
                message
            } else {
                CargoTestOutput::Custom { text: line.to_owned() }
            },
        })
    }

    fn from_eof(&self) -> Option<CargoTestMessage> {
        Some(CargoTestMessage { target: self.target.clone(), output: CargoTestOutput::Finished })
    }
}

#[derive(Debug)]
pub(crate) struct CargoTestHandle {
    _handle: CommandHandle<CargoTestMessage>,
}

// Example of a cargo test command:
// cargo test --workspace --no-fail-fast -- -Z unstable-options --format=json
// or
// cargo test --package my-package --bin my_bin --no-fail-fast -- module::func -Z unstable-options --format=json

#[derive(Debug, Clone)]
pub(crate) enum TestTarget {
    Workspace,
    Package { package: String, target: String, kind: TargetKind },
}

impl TestTarget {
    pub(crate) fn target_name(&self) -> Option<&str> {
        match self {
            TestTarget::Workspace => None,
            TestTarget::Package { target, .. } => Some(target),
        }
    }
}

impl CargoTestHandle {
    pub(crate) fn new(
        path: Option<&str>,
        options: CargoOptions,
        root: &AbsPath,
        test_target: TestTarget,
        sender: Sender<CargoTestMessage>,
    ) -> std::io::Result<Self> {
        let mut cmd = toolchain::command(Tool::Cargo.path(), root);
        cmd.env("RUSTC_BOOTSTRAP", "1");
        cmd.arg("test");

        match &test_target {
            TestTarget::Package { package, target, kind } => {
                cmd.arg("--package");
                cmd.arg(package);
                match kind {
                    TargetKind::Bin => {
                        cmd.arg("--bin");
                        cmd.arg(target);
                    }
                    TargetKind::Test => {
                        cmd.arg("--test");
                        cmd.arg(target);
                    }
                    TargetKind::Bench => {
                        cmd.arg("--bench");
                        cmd.arg(target);
                    }
                    TargetKind::Example => {
                        cmd.arg("--example");
                        cmd.arg(target);
                    }
                    TargetKind::Lib { .. } => {
                        cmd.arg("--lib");
                        // no name required because there can only be one lib target
                    }
                    TargetKind::BuildScript | TargetKind::Other => {}
                }
            }
            TestTarget::Workspace => {
                cmd.arg("--workspace");
            }
        };

        // --no-fail-fast is needed to ensure that all requested tests will run
        cmd.arg("--no-fail-fast");
        cmd.arg("--manifest-path");
        cmd.arg(root.join("Cargo.toml"));
        options.apply_on_command(&mut cmd);
        cmd.arg("--");
        if let Some(path) = path {
            cmd.arg(path);
        }
        cmd.args(["-Z", "unstable-options"]);
        cmd.arg("--format=json");

        for extra_arg in options.extra_test_bin_args {
            cmd.arg(extra_arg);
        }

        Ok(Self {
            _handle: CommandHandle::spawn_with_parser(
                cmd,
                CargoTestOutputParser::new(&test_target),
                sender,
            )?,
        })
    }
}
