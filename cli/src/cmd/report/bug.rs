use std::iter;
use crate::Config;

/// The issue template as a macro so then it can be used as a formatting string
/// separate from the rest of the code.
///
/// See `.github/ISSUE_TEMPLATE/bug_report.md` for the base issue template.
///
/// This *must* be small or else GitHub gives "414 Request-URI Too Large".
macro_rules! template {
    () => {
r#"<!--
Thank you for reporting a bug in Ocean!

Please fill in as much of the template below as possible. This helps us address
and hopefully fix what you're dealing with.
-->

# 🐛 Bug Report

<!--
Clearly and concisely describe what happened.
-->

## Steps to Reproduce

<!--
Succinctly explain the steps to reproduce this.
-->

## Components Affected

- CLI client
  - Version: {version}
  - Commit: {commit}

## Platforms Affected

- {os} ({arch})

## Detailed Description

<!--
Write a longer description here than "Steps to Reproduce".
-->

## Relevant Issues

<!--
Link relevant issues or remove this section.
-->"#
    };
}

/// Creates a URL suitable for opening a condensed variant of [`bug_report.md`]
/// with the user's Ocean and system info filled out.
///
/// [`bug_report.md`]: https://github.com/oceanpkg/ocean/blob/master/.github/ISSUE_TEMPLATE/bug_report.md
pub fn url(_config: &Config) -> String {
    let version  = env!("CARGO_PKG_VERSION");
    let revision = option_env!("OCEAN_GIT_REV").unwrap_or("Unknown");

    let target_os   = env!("OCEAN_TARGET_OS");
    let target_arch = env!("OCEAN_TARGET_ARCH");

    let body = format!(
        template!(),
        version = version,
        commit  = revision,
        os      = target_os,
        arch    = target_arch,
    );
    let base = "\
        https://github.com/oceanpkg/ocean/issues/new\
        ?assignees=nvzqz\
        &template=bug_report.md\
        &labels=kind%2Fbug\
        &title=%5BDescribe+the+problem+you+encountered%5D\
        &body=\
    ";
    iter::once(base)
        .chain(percent_encoding::utf8_percent_encode(
            &body,
            percent_encoding::NON_ALPHANUMERIC,
        ))
        .collect()
}
