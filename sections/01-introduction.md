# Introduction

Rust is a programming language that focusses on performance, security an reliability. The _Rust Project_ is open source and dual-licensed under the Apache 2.0 and MIT license [@github.rust.license]. Rust 1.0 the first stable version was announced in May 2015 [@rust-blog.rust1-0]. This release also marked the start of the _commitment to stability_ which promises stability on future Rust stable releases [@rust-blog.stability-promise]. This new commitment also introduced a 6-week release cycle as well as development channels for language users and early adapters [@rust-blog.stability-promise]. The latest stable compiler version 1.51.0 has been released on 25 of March 2021 [@rust-blog.rust1-51]. Developers and teams within the project put high effort into open communication. This focussed is formalized in the official _Code of conduct_ [@rust-lang.coc]. The language with its connected tools has attracted over 5900 individual contributes as of writing this [@rust-lang.thanks].

The Rust project develops several tools besides the compiler itself. These tools are seen as a vital part in automating parts of the development process and collaboration among teams. _Clippy_ the official linter of the Rust project is being developed in the _rust-clippy_ repository. The linter contains over 450 lints which span from complexity and style lints over to restriction lints which might be required by certificates [@github.clippy.readme]. Since 2018 Clippy is distributed as a component with the compiler itself [@github.rustup.1461].

* Developed by mozilla
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