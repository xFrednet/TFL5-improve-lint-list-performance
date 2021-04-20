# TFL improvements to Clippys website
My university requires me to hand in several papers which describe how I applied some knowledge in a project. In this fifth paper I'll focus on [Clippy's lint list](https://rust-lang.github.io/rust-clippy/master/index.html) with the question: _How can the internet presentation of the lint list for the rust-clippy project be improved?_

The secondary goal of this repo is to train my English writing skills since I've never written a complete scientific paper in English. Wish me luck I guess^^.

Latest handing date: 2021-05-04

## Typos
Spelling is sadly not my strong suite, I'll go through the paper at the end to correct mistakes and reformulate some content.

## TODOs (Those that are not included in the source files them self)
* TODO xFrednet 2021-04-15: Maybe add an abstract section to the start.

## Building
Use the following comments to build the actual document:

```sh
$ pdflatex --shell-escape main.tex
$ biber main
$ pdflatex --shell-escape main.tex
```