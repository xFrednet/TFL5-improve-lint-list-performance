# Fulfillment of requirements \label{sec:measurement}
This chapter will measure the current fulfillment for the previously in \ref{sec:requirements} defined requirements. These results will then be used to identify the key aspects that need improvement. Additionally they will be used for comparison when testing suggestions.

## Mozilla Observatory
_Mozilla Observatory_ is a collection of tools that can analyze a website to determine which available security measures have been utilized by it [@github.observatory.readme]. These security is focussed on values that can be set in the HTTP header to indicate that opt-in security options should be enabled by the browser [@TODO]. The Rust development documentation links to a free [online interface](https://observatory.mozilla.org/) for the Mozilla Observatory that is provided by the Mozilla Foundation free of charge.

### Scoring
The result of the analyzes is summarized in a single score with a corresponding grade. The score is calculated using a baseline each checked criteria can add bonus points or subtracted penalty. This implementation is used to give different weight to specific configurations. The significance of these modifiers are based on how important the analyzed aspect for security. Scores can range from a minimum of 0 to a maximum of 135, the score of 100 already indicates that the website is configured correctly a higher score can be archived by archiving bonuses. A score of 100 and above corresponds to the grade _A+_ [@github.observatory.scoring].

The observatory documentation notes that all websites are graded equally, this means certain graded configurations might be unimportant for the specific use case [@mozilla.observatory.faq].

### Measurement
Scanning Clippy's lint list results in an overall grade of _C_ with a score of 55/100. It is to note that the analysis cut of the path to the lint list and graded the host itself. The results are therefor for _rust-lang.github.io_ in general. The score was calculated using the baseline of 100 points and subtracting a penalty of 45 points. This sanction is the result of three failed tests that are shown in table \ref{tab:scan.rust-lang.github.io.2021-04-24}.

\input{sections/03-measurement/observatory-scan-rust-lang.github.io-2021-04-24.tex}

The original scan output with all test results is included in attachment number \ref{att:scan.rust-lang.github.io.2021-04-24.json}.

## Initial page load time
What is exactly being measured?

* From opening the lint list
* To seeing the list (Loading... is not valid)

### Note problems
Influenced by almost everything

Currently interested in significant speedups this means that small deviations are not detrimental.

### Setup
Describe setup maybe using debugger, tool or log statements. Take average of X times

### Measurement
Add a nice table with an average and so on

## Summary of benchmarks
Select * from above where x.interesting is true
