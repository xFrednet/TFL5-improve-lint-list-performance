# Clippy's lint list \label{sec:clippys-lint-list}
<!-- Reviewed: 1x newly written. -->
Chapter \ref{sec:into} gives an introduction of Clippy and the lint list which is being maintained by the contributors of the rust-clippy project. This section will provide relevant background information about the website and it hosting. It will then summarize the relevant requirements and conclude with an overview which aspects will be further investigated.

## Hosting
<!-- Reviewed: 1x newly written. -->
Clippy's lint list is a static website centered around a HTML file which displays a json document with lint documentation and metadata. It additionally references resources by other projects but these two are the only once that are directly hosted in project. The website is automatically updated and deployed with every merged pull request.

The rust-clippy project has selected _GitHub Pages_ as a hosting provider. GitHub Pages provides a simple way to host project websites directly from the repository itself. GitHub additionally provides a project domain which is made up of the name of the organization or username and a path to the project [@github.docs.about-pages]. For Clippy this domain is \texttt{\url{https://rust-lang.github.io/rust-clippy/}}. The use of this hosting adds no additional cost if the user or organization has a payed product plan, like _GitHub Pro_ or _GitHub Team_ [@github.docs.gh-products]. The later applies to the Rust Organization. GitHub Pages has soft limits when it comes to bandwidth usage, page site and amount of page updates per hour. The documentation also states that the hosting should not be used directly for commercial purposes or sensitive and personal data [@github.docs.about-pages].

## Requirements \label{sec:requirements}
<!-- Reviewed: 1x rewritten -->
The research question specified in \ref{sec:into.question} focusses on requirements that are put on the lint list as a static website that is provided as a part of the Rust community. This part of the paper outlines these requirements. 

### Website functionality
<!-- Reviewed: 1x rewritten -->
The website has _functional requirements_ which describe direct functionality and behavior that the website should provide [@book.Sommerville2010, p. 83ff]. An example is the implemented search and filter feature. These requirements will not be listed as part of this work as they are not necessarily needed to fulfil the technical requirements defined in the Rust development documentation. However, it is noteworthy that this functionality should not be impacted by suggestions in this paper. This paragraph will be referenced again if a suggested solution could impact them. A list of requested and implemented functionality can be retrieved from the rust-clippy issue tracker.

### Requirements by the Rust Infrastructure Team
<!-- Reviewed: 1x rewritten -->
The Rust Infrastructure team has created a set of guidelines that static websites affiliated with the Rust project should fulfill to be hosted and managed by them. Clippy is an official Rust project and the website itself presents static content, the guidelines therefor apply to the website. The requirements are as follows [@rust-forge.static-websites]:

* "The website must be managed by a Rust team, or be officially affiliated with the project."
    * This point excludes community projects due to finite resources of the infrastructure team.
* "The website’s content and build tooling must be hosted on a GitHub repository in either the [rust-lang](https://github.com/rust-lang) or [rust-lang-nursery](https://github.com/rust-lang-nursery) organizations."
    * The teams wants to be able to rebuild the website at any time. They therefor require it to be hosted in a GitHub repository that is also managed by them.
* "The website must be built and deployed with a CI service."
* "The website must reach an A+ grade on the [Mozilla Observatory](https://observatory.mozilla.org/)."
    * This requirement focusses on user security as it ensures that multiple security features are enabled for the website. The referenced tool analyzes security features than can toggled through header fields in the HTTP response by the hosting provider. The target grade indicates that the website is configured correctly. 
* "The website must be hosted on platforms vetted by the infra team."
    * The documentation recommends the usage of _GitHub Pages_ or _Amazon AWS_ in combination with _CloudFront_ as a _content delivery network_. Other providers can be suggested and requested as long as they are deemed to be secure and reliable.


<!--
This section can be expanded if more text is needed.

You might read that sentence and ask: WHAT?

So, let me rage a bit. My university had the _brilliant_ idea to create these assignments which are
not even recognized by other universities but are needed for their specific master. This sounds
stupid enough but hey, that means they are optional or only required for master students right?
No, that's not how management works with them...

While raging I have to say that it does have some purpose. We learn how to write scientific
papers before our actual theses. That point given, why do I need to write six of them???
The last papers took a lot of work specially for someone who kind of struggles with writing and
there was only positive feedback why do I need to write even more?

Okay, let's end the rage here. The simple answer is that these assignments have a page requirement
of 10 pages +- 10%. This means that I might have to waste my time and your time just to reach the
required page count even if all important aspects have been said...

I'm just a bit frustrated by people and systems who waste my lifetime. That's the most valuable
resource I have.
-->