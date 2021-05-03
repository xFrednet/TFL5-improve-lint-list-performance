# Fulfillment of requirements \label{sec:measurement}
<!-- Reviewed: 1x rewritten -->
The previous chapter has summarized the requirements that are put on Clippy's lint list and provided additional information and reasoning behind them. This section will investigate to which extent these are currently fulfilled. The goal is to identify key areas that could be improved.

## Repository requirements
<!-- Reviewed: 1x newly written. -->
This first part evaluates the project and repository related requirements for the rust-clippy project. Clippy is as mentioned in \ref{sec:into} the official linter for the Rust language and deployed as part of the Rust installation. This makes Clippy an affiliated project to the Rust organization. The project repository is a part of the rust-lang organization an GitHub and therefor also satisfies the requirement of being managed by the Rust Infrastructure Team. Clippy therefor meets the project related requirements.

## Hosting requirements
<!-- Reviewed: 1x newly written. -->
This section assesses the requirements connected to website hosting for Clippy. The project uses _continues integration_ to deploy the lint list with every merged pull request [@github.clippy.ci]. This ensures that the documentation is always up to date with current development. The content is hosted on GitHub Pages, this is a vetted and even suggested website host by the Rust Infrastructure Team. Clippy's lint list therefor fulfills all requirements in relation to deployment and content hosting.

## Mozilla Observatory rating
<!-- Reviewed: 1x rewritten -->
_Mozilla Observatory_ is a collection of tools that can analyze a website to determine which available security measures have been utilized by it [@github.observatory.readme]. The scan is focussed on opt-in security options that are set in the HTTP response header, these will then instruct the client to enforce them [@rust-forge.static-websites]. The Rust development documentation links to a free [online interface](https://observatory.mozilla.org/) for the Mozilla Observatory that is provided by the Mozilla Foundation free of charge. The requirements in \ref{sec:requirements} state that a website should archive the grade A+.

### Scoring
<!-- Reviewed: 1x slightly changed -->
The result of the analyses is summarized in a single score with a corresponding grade. The score is calculated using a baseline. Each checked criteria can add bonus points or subtracted a penalty. This implementation is used to give different weight to specific configurations. The significance of these modifiers are based on how important the analyzed aspect for security. Scores can range from a minimum of 0 to a maximum of 135, the score of 100 already indicates that the website is configured correctly a higher score can be archived by gaining bonus points. A score of 100 and above corresponds to the grade _A+_ [@github.observatory.scoring].

The observatory documentation notes that all websites are graded equally, this means certain graded configurations might be unimportant for the specific use case [@mozilla.observatory.faq].

### Measurement
<!-- Reviewed: 1x slightly changed -->
Scanning Clippy's lint list results in an overall grade of _C_ with a score of 55/100. It is to note that the analysis cut of the path to the lint list and graded the domain itself. The results are therefor for the URL \texttt{\url{rust-lang.github.io}} in general. The score was calculated using the baseline of 100 points and subtracting a penalty of 45 points. This sanction is the result of three failed tests that are shown in table \ref{tab:scan.rust-lang.github.io.2021-04-24}.

\input{sections/03-measurement/observatory-scan-rust-lang.github.io-2021-04-24.tex}

The original scan output with all test results is included in attachment number \ref{att:scan.rust-lang.github.io.2021-04-24.json}.

## Summary
<!-- Reviewed: 1x newly written. -->
The requirement evaluation has shown that Clippy's lint list fulfills all requirements except for archiving the A+ grade by the Mozilla Observatory rating engine. The actual grade is a C which is a result of three missing entries in the HTTP response header. The missing values are displayed in table \ref{tab:scan.rust-lang.github.io.2021-04-24}.
