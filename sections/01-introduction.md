# Introduction \label{sec:into}
Rust is a programming language that focuses on performance, security, and reliability. The compiler is open source and dual-licensed under the Apache 2.0 and MIT license [@github.rust.license]. Rust 1.0 the first stable version was announced in May 2015 [@rust-blog.rust1-0]. This release also marked the start of the _commitment to stability_ which promises stability on future Rust stable releases [@rust-blog.stability-promise]. This new commitment also introduced a 6-week release cycle as well as development channels for language users and early adopters [@rust-blog.stability-promise]. The latest stable compiler version 1.51.0 has been released on 25 of March 2021 [@rust-blog.rust1-51]. Developers and teams within the project put high effort into open communication. This focusis formalized in the official _Code of conduct_ [@rust-lang.coc]. The language with its connected tools has attracted over 5900 individual contributors as of writing this [@rust-lang.thanks].

The Rust project consists out of several tools besides the compiler itself. These tools are seen as a vital part in automating parts of the development process and collaboration among teams. _Clippy_ is the official linter for Rust and is being developed in the _rust-clippy_ repository. The linter contains over 450 lints which span from complexity and style lints over to restriction lints which might be required by certificates [@github.clippy.readme]. Clippy is written in Rust itself and interfaces with the compiler directly. This direct connection enables the use of the existing lexer, parser, and connected diagnostic tools and ensures that the project stays up to date with the latest compiler changes. Since 2018 Clippy has been distributed as a component of the Rust installation itself [@github.rustup.1461].

## Problem \label{sec:into.problem}
<!-- Reviewed: 1x rewritten -->
Clippy maintains a website that contains documentation about all implemented lints. This list has the title _ALL the Clippy Lints_ and will be referred to as _Clippy's lint list_ or simply _lint list_ in this paper. Diagnostic messages of the tool provide a suggestion and usually a small explanation with a reference to the website for detailed lint documentation with examples. This makes Clippy's lint list the second point of contact for new users with the project itself. The lint list is also the only internet presentation of Clippy besides the GitHub repository inside the Rust organization.

Offering online documentation gives a central point of reference that can be linked to and used in discussions. However, it also brings some responsibility when it comes to security and functionality. The _Rust Infrastructure Team_, a team inside the project with members that organize and manage the entire infrastructure, has therefore defined some guidelines for static websites [@rust-forge.static-websites]. Clippy's lint list is static and should therefore follow these rules. A small review of these requirements has shown that not all of them might be fulfilled when it comes to security. Not having them fulfilled might give a bad impression for new users and reduce the search engine rating.

A secondary problem is the initial load time of the lint list which is noticeably slower than most other websites in the Rust ecosystem. This aspect also influences the user experience and search engine rating. However, this will not be evaluated as part of this paper due to the fact that there have been some recent discussions on the topic inside the community to change the display of content completely which would void all research on this topic.

## Research question \label{sec:into.question}
<!-- Reviewed: 1x rewritten -->
The described problem in \ref{sec:into.problem} leads to the following research question: _Does Clippy's lint list fulfill all requirements and if not how can this be improved?_

## Goal
<!-- Reviewed: 1x rewritten -->
The primary goal of this paper is to review which requirements are currently not met and possibly find a solution to fulfill them. These solutions should ideally be simple to implement in the form of a pull request in the GitHub repository or as a suggestion on how to change the settings of the hosting provider.

## Approach
<!-- Reviewed: 1x rewritten -->
<!-- TODO xFrednet 2021-05-02: Maybe rewrite to present -->
The start of this paper will provide some context about the Clippy's lint list and the current hosting provider. It will then collect the requirements defined for static websites, like Clippy's lint list, inside the Rust ecosystem.

The next chapter will then measure the current fulfillment of these collected requirements to deduct which topics should be further investigated. The following section will analyze the measurements and explain the technical importance behind them as well as evaluate the importance for Clippy.

Based on this work the author will try to find or develop solutions for unfulfilled requirements. This section might include some practical tests to see if certain changes have the desired effects.

The assignment will conclude with a summary of the investigated topics and suggestions for further work that can be done on the topic.
