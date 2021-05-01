# Conclusion

## Summary
This paper as summarized the technical requirements that the lint list of the rust-clippy project should fulfill in section \ref{sec:requirements}. The main requirement put on the website is the goal of archiving A+ rating by the rating engine Mozilla Observatory.

The assignment then continued in section \ref{sec:measurement} with an evaluation to which extend the requirement is currently fulfilled. This evaluation is done by using the previously mentioned tool. The results reveal that the evaluation and user security is impacted by the three following missing HTTP headers: 

* Strict-Transport-Security
* X-Frame-Options
* X-Content-Type-Options

The following section \ref{sec:analysis} analyzed the security concern of the missing headers by explaining their behavior and reasoning behind them. This explanation was based on the formal specification and most relevant documentation. The paper than discussed the importance for Clippy's lint list and suggested a initial value that the specific field should be set to. The results of this evaluation have been summarized in table \ref{tab:solution.http-header.target-values}.

The \ref{sec:solutions} chapter then investigated how these header values could be set for the website. This included an investigation which settings are provided by the used hosting provider GitHub Pages. This is followed by evaluating the HTML meta tag as the author was unable to find a solution using the hosting providers settings. It was determined that the meta tag could be used to specify values for Strict-Transport-Security and X-Content-Type-Options in the Firefox and Chromium browser even if this behavior is not part of the living standard. The last solution looked at the possibility to use a content delivery network to set these headers. It was determined that this last solution would work but add complexity to the hosting process and consume some additional resources. 

## Steps moving forward
The section \ref{sec:solutions} has concluded that the missing headers can be set with the use of a content delivery network. The next step is now to investigate if the added complexity and additional use of resources this would take are worth it as the website is currently operating to no additional cost to the project. Especially due to the fact that GitHub Pages is still a recommended hosting provider by the Rust Infrastructure Team.  

If the decision is made to continue the use and deployment using GitHub pages then it might be worth to investigate the html meta tag a bit more. The use of the meta tag can address the two main security concerns in regard of the  the Strict-Transport-Security and X-Content-Type-Options header fields. Before using this feature it haas to be investigated if this usage is supported by all major browsers as it's not part of the living standard of HTML.