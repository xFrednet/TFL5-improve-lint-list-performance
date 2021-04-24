# Requirements \label{sec:requirements}
The goal of this work is to improve the impression of Clippy's lint list. This section of the document will set a list of requirements to focus on in further research for this paper. Requirements are usually split up into the following two groups [@book.Sommerville2010, p. 83ff]:

* *Functional requirements* describe direct functionality and behavior that a system should provide. They can also be defined as negations, stating that a certain behavior should not happen. These requirements are usually documented in an abstract way to enable system users to understand them.
* *Non-functional requirements* are focussed on the characteristics of the system itself, an example might be the requirement to have a reliable and maintainable system. These requirements can include constraints that the system might have to take care of.

## Implemented functionality
The topic of this assignment focusses on the perception and impression of Clippy's lint list as an entire system. The research question therefor puts a focus on non-functional requirements. The website itself is based on functional requirements and these should remain fulfilled even after the suggested changes. It is therefor important to note them in some way or form while not taking focus of the key point of this paper. All requirements that have been implemented previously will therefor be summarized in the following functional requirement: _The functionality of the website should not be impacted by the implementation of new measures to improve the impression or usage._ This requirement covers functionality like the search feature, filter options and theming. The implementation of all of these has been completed at the point of writing this. A more extensive specification of the underlying requirements can be retrieved from the rust-clippy issue tracker.

## Non-functional requirements (Maybe: Requirements by the Infra team)
The Rust Infrastructure team has created a set of guidelines that static websites affiliated with the Rust project should fulfill. Clippy is an official Rust project and the website itself presents static content, the guidelines therefor apply to the website. 

These guidelines contain a _formal specification_ that states: _"The website must reach an A+ grade on the Mozilla Observatory."_ [@rust-forge.static-websites]. This specification is based on the requirement of security. The A+ grade ensures that all important headers have been set and that enhanced users privacy features should be enabled by the browser [@rust-forge.static-websites]. A secure website contributes to a trustful relation ship between the user and Clippy's lint list. It additionally improves the ranking in most search engines and therefor helps users to faster find the documentation they require. <!-- TODO xFrednet 2021--04-24: quelle? -->

The guidelines from the infrastructure team additionally contain some functional requirements that are not directly connected to the research question, they are also already fulfilled by the current setup. These are therefor included in the defined functional requirement. 

## Performance
The second major non-functional requirement this assignment will look at is performance.

* Search performance
* Load performance 

<!-- 
TODO xFrednet 2021-04-24: mention that GitHub pages are officially recommended as a hosting provider 
https://forge.rust-lang.org/infra/guidelines/static-websites.html
-->

<!--
## Functional requirements
* List formal requirements
* Say: These formal requirements have also been defined as formal specifications:
* *Formal specifications*: Specifications that are unambiguous and formally defined. Formal specifications have the advantage that the precise description and limited scope reduce misunderstandings. These specifications are usually a translation of user requirements which are often expressed in natural language. This translation forces a detailed analysis of the requirements which often catches errors in them, that were previously hidden by ambiguous language. The fulfillment of these specifications can also be verified using manual or tool-supported methods [@book.Sommerville2010, p. 334].
    * Technical security measured by Mozilla Observatory

## Non-functional requirements
* We need SPEED
    * Why are these informal? This paper want's to focus and work on the technical background not on fining the most optimal solution

## Specifications
Or summary what ever works better

-->

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