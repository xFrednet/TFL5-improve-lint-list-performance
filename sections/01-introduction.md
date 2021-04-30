# Introduction

Rust is a programming language that focusses on performance, security an reliability. The _Rust Project_ is open source and dual-licensed under the Apache 2.0 and MIT license [@github.rust.license]. Rust 1.0 the first stable version was announced in May 2015 [@rust-blog.rust1-0]. This release also marked the start of the _commitment to stability_ which promises stability on future Rust stable releases [@rust-blog.stability-promise]. This new commitment also introduced a 6-week release cycle as well as development channels for language users and early adapters [@rust-blog.stability-promise]. The latest stable compiler version 1.51.0 has been released on 25 of March 2021 [@rust-blog.rust1-51]. Developers and teams within the project put high effort into open communication. This focussed is formalized in the official _Code of conduct_ [@rust-lang.coc]. The language with its connected tools has attracted over 5900 individual contributes as of writing this [@rust-lang.thanks].

The Rust project develops several tools besides the compiler itself. These tools are seen as a vital part in automating parts of the development process and collaboration among teams. _Clippy_ is the official linter for Rust and is being developed in the _rust-clippy_ repository. The linter contains over 450 lints which span from complexity and style lints over to restriction lints which might be required by certificates [@github.clippy.readme]. Clippy is written in Rust itself and interfaces with the compiler directly. This direct connection enables the use of the existing lexer, parser and connected diagnostic tools and ensures that the project stays up to date with the latest compiler changes. Since 2018 Clippy is distributed as a component of the Rust installation itself [@github.rustup.1461].

## Problem \label{sec:into.problem}
Clippy, like the compiler, puts great effort into giving helpful diagnostic messages. These messages consist out of a message, a direct quotation of the suboptimal code fragment and a direct suggestion how the code quality can be improved. The first message of a lint additionally includes a note why which lint is enabled in this location and a reference to the lint documentation in Clippy's lint list. This makes Clippy's lint list the second point of contact for new users with the project itself. The initial load time of the lint list is noticeably slower than most other websites in the Rust eco system. 

Another pain point of the website are some technical deficiencies. The development documentation of the Rust project includes some technical guidelines that the current lint list doesn't fully fulfill.

## Research question
The described problem in \ref{sec:into.problem} leads to the following research question: _How can the internet presentation of the lint list for the rust-clippy project be improved?_

## Overall goal
The primary goal of this paper is to review the previously mentioned problems and to find solutions for them.

## Approach
The start of this paper will collect the formal and informal specifications that Clippy's lint list should optimally fulfill. The next part will analyze the current state of the website. This chapter will conclude with a list of aspects that should be improved as well as a selection which this research will focus on.

The previous preparation leads to the solution finding process. This process will start with a technical explanation of the aspect to determine why this specification is important. The next step tries to find a solution for the found issues under the given circumstances. Additionally, this section will include a practical test if the suggested change would improved the focussed problem.

This paper will end with a summary of the reviewed aspects and a conclusion as well as a suggestion what further work can be done on this topic.

<!--

* Developed by mozilla
* Used by Google, Microsoft, Linux Kernel, Amazon
* Rust foundation
* Opensource
* MIT licence
* Clippy official linter
* Lint description splitup
* Clippy is released as part of Rust
* No new lints policy https://rust-lang.github.io/rfcs/2476-clippy-uno.html?highlight=Clippy

Some text

* What is rust
    * Language
    * Rust foundation
* What is clippy (Well rust-clippy until defined that it will be called Clippy)
    * Background
    * Clippy's lint lint (Only what it does)
* Problems / Motivation behind this work
    * Long initial loading time
    * Mozilla Observatory low score
* Main question: _How can the internet presentation of the lint list for the rust-clippy project be improved?_
* How will this paper try to solve the main question
    * Define specifications
    * Look at current fulfillment
    * Explain the technical background
    * Try to find a solution (Or contact GH support)

Some text

-->