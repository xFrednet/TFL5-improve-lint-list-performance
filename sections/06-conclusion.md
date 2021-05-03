# Conclusion
<!-- Reviewed: 1x rewritten -->
This paper succeeded in reviewing the research question defined in \label{sec:into.question}. It determined that Clippy's lint list currently does not fulfill all requirements. The paper was then able to find a possible solution for these missing aspects.

## Summary
<!-- Reviewed: 1x rewritten + present -->
Section \ref{sec:into} introduces the rust-clippy project and the problem that same requirements put on the website might not be meet currently. This lead to the following research question: _Does Clippy's lint list fulfill all requirements and if not how can this be improved?_

To answer this question the paper first introduced Clippy's lint list in section \ref{sec:clippys-lint-list} and then summarized the technical requirements that the lint list of the rust-clippy project should fulfill.

The assignment then continued in section \ref{sec:measurement} with an evaluation to which extend the requirements are currently fulfilled. The results reveal that the user security is impacted by the three following missing HTTP headers: 

* Strict-Transport-Security
* X-Frame-Options
* X-Content-Type-Options

The following section \ref{sec:analysis} analyzed the security concern of these missing headers by explaining their behavior and reasoning behind them. This explanation was based on the formal specification and most relevant documentation. The paper than discussed the importance for Clippy's lint list and suggested a initial value that the specific field should be set to. The results of this evaluation have been summarized in table \ref{tab:solution.http-header.target-values}.

The \ref{sec:solutions} chapter then investigated how these values can be in the selected content host. This includes an investigation which settings are provided by the used hosting provider GitHub Pages. Following an evaluating of the HTML meta tag and the use a content delivery network to set these headers.

## Solution
<!-- Reviewed: 1x rewritten + present -->
Chapter \ref{sec:solutions} determines that GitHub Pages configurations are not sufficient enough to meet the set requirements. In \ref{sec:solutions.meta-tag} it is found that the HTML meta tag can be used to set two of the required headers in Firefox and Chromium. This solution has the drawbacks that it is not a universal solution and only HTML document specific. The last investigated solution is the use of a CDN which is able to add the headers under the condition of using a custom domain. The last solution adds complexity to the project and uses additional resources.

The paper therefor concludes with two possible actions that can be taken. It now has to be evaluated which one of these should be taken if any. This evaluation extends over the scope of this paper.

## Steps moving forward
<!-- Reviewed: 1x rewritten + present-->
Section \ref{sec:solutions} concludes that the missing headers can be set with the use of a content delivery network. The next step is now to investigate if the added complexity and additional use of resources worth it as the website is currently operating to no additional cost to the project. It also has to be taken into consideration that GitHub Pages is still a recommended hosting provider by the Rust Infrastructure Team.

If the decision is made to continue the use and deployment using GitHub Pages then it might be worth to investigate the html meta tag a bit more. The use of the meta tag can address the two main security concerns in regard of the `Strict-Transport-Security` and `X-Content-Type-Options` response header fields. Before using this feature it should to be investigated if this functionality is supported by all major browsers as it is not part of the living standard of HTML.