use std::iter;
use crate::Config;

/// See `.github/ISSUE_TEMPLATE/feature_request.md` for the base issue template.
///
/// This *must* be small or else GitHub gives "414 Request-URI Too Large".
const TEMPLATE: &str = r#"<!--
Thank you for submitting a feature request for Ocean!
-->

# 💡 Feature Request

<!--
Explain in one paragraph what the feature is.
-->

I have a dream that Ocean will be able to ...

## Components

<!--
Please specify the ocean components the new feature should be added to.
Otherwise, delete this section.

If this feature is platform-specific, state the platforms and why the feature
may not work on other platforms.
-->

This would work on Ocean's ...

## Motivation

<!--
Why would we do this? Describe reasons to use this feature.

Describe the use case(s) or other motivation for the new feature.
-->

We should do this because ...

## Proposal

<!--
Describe how this feature might be implemented, and why. Add any considered
drawbacks.
-->

This can be achieved by ...

## Prior Art

<!--
Discuss prior art, both the good and the bad, in relation to this proposal. A
few examples of what this can include are:
- For CLI suggestions: What other programs have this command/flag and what are
  the semantics?
- For design suggestions: Where has this design language been used before?
-->

If you take a look at ..., you'll see that it has ...

## Alternatives

<!--
Are there other ways to solve this problem that you've considered? What are
their potential drawbacks? Why was the proposed solution chosen over these
alternatives?
-->

We could also take the approach explained at ...
"#;

/// Creates a URL suitable for opening a condensed variant of [`bug_report.md`]
/// with the user's Ocean and system info filled out.
///
/// [`bug_report.md`]: https://github.com/oceanpkg/ocean/blob/master/.github/ISSUE_TEMPLATE/bug_report.md
pub fn url(_config: &Config) -> String {
    let body = TEMPLATE;
    let base = "\
        https://github.com/oceanpkg/ocean/issues/new\
        ?assignees=nvzqz\
        &template=feature_request.md\
        &labels=kind%2Ffeature\
        &title=%5BDescribe+the+feature+you%27re+proposing%5D\
        &body=\
    ";
    iter::once(base)
        .chain(percent_encoding::utf8_percent_encode(
            &body,
            percent_encoding::NON_ALPHANUMERIC,
        ))
        .collect()
}
