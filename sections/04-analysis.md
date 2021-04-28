# Analysis of benchmark results
This chapter will inspect each identified technical problem from section \ref{sec:measurement}. The inspection will first explain the technical background behind the problem and then identify the optimal configuration.

The observatory scan focuses on HTTP header which are set by the server behind the domain. The scan was therefor conducted for the domain `rust-lang.github.io`. Clippy's lint list is indirectly included in this result as well as documentation from repositories by the _Rust Organization_. Further investigation will continue to focus on the context of Clippy's lint list however improvements to the server would directly improve other websites.

* TODO xFrednet 2021-04-29: Move section about clippy hosting to specification
* TODO xFrednet 2021-04-29: HTTP explanation/into header stuff

## HTTP Strict Transport Security (HSTS)
HTTP Strict Transport Security (HSTS) is a optional HTTP header field that requests the client accessing the HTTP API to only use encrypted connection for further requests. The request to use and encrypted connection extends to all resources that are referenced by the requested result. It is therefor necessary that these resources also provide the option to connect via HTTPS [@ietf.rfc6797, p. 6ff].

### Risks
The specification references three threads that can be prevented using this header [@ietf.rfc6797, p. 6ff]:

1. Using an unencryped connection allows attackers to eavesdrop on the exchanged data. This is a _passive network attack_ and can be used to collect personal information, passwords or browsing habits.
2. A HTTPS connection requires a certificate that has to be signed by a certification authority. This certificate intern lists the owner or organization. This can be used to validate that the displayed content really originates from the expected source and with that prevent attackers from creating a fake website copy to steal otherwise secure information.
3. Forcing the use of HTTPS additionally ensures that mistakes like referencing ressources via HTTP links will be corrected by the requesting client

### Importance for Clippy
Clippy's lint lint only displays publicly available information about lints in a easy accessible and searchable way. A passive network attack could therefor not collect any secret of personal information about the user. Except the fact that they visited the domain at all. However, this would however also be possible with the header as the connected IP is not effected by it. This also extends to the third thread of accidentally not requesting unencrypted resources, this can currently still happen but would not be detrimental.

The second thread of modification of the website is the relevant thread in this case. An attacker could for instance inject a donation button as several developers have expressed interest to donate to the Rust Foundation itself. This button would then forward the user to another page of the attacker to donate.

With all of this being said it has to be noted that all references to the website already include `https` at the start and a user has to deliberately enter the domain with http in front. Most browsers will then still recommend to use the encrypted connection or at least add a _not encrypted_ notice next to the URL. All of this results in a very low risk. The header should still be set if the hosting provider provides a simple setting for this. Also due to the fact that the targeted A+ rating would require this field.

<!-- TODO xFrednet 2021-04-27: Define which value the header should be set to -->

## X-Frame-Options (XFO)
This header was initially implemented by browsers as a non-standard HTTP header field as a new security measure to prevent the thread clickjacking. In 2013 the header was formalized by the _Internet Engineering Task Force_ (_IETF_) in RFC7014. Clickjacking describes is the act of hijacking clicks of the user, this can be done by embedding a website that should be hijacks as a frame and than getting the user to unknowingly interact with that site. The XFO header field allows a host to specify that delivered content must not be displayed in a frame [@ietf.rfc7034, p. 3].

The option can be set to three mutually exclusive values [@ietf.rfc7034, p. 4]:

* _DENY_: Indicates that the content should not be displayed in any frame.
* _SAMEORIGIN_: Allows the display of the content inside a frame as long as it originated from the same origin as the frame.
* _ALLOW-FROM_: This prohibits the display of the content with the exception of the origins that are defined after the "ALLOW-FROM" value.

### Importance for Clippy
Not important at all, clippy only contains open source information and no active login etc. therefor not important. Even limiting as the list could not be embedded. 

BUT this could still be good to set in case that there will be some additions in the future (even if none are planned). Therefor if used the set to _SAMEORIGIN_.

## X-Content-Type-Options
What content to we want? All the content who do we trust: [@mozilla.developer.content-type-opt] and who was the weird inventor? Yes: [@microsoft.docs.ie8-security]

## Slow loading times

Some form of art
<!--
## Technical background

Give hosting background IE the website is deployed using GH Pages etc...

## Technical problems
* Explaining the grade C from _Mozilla Observatory_
* This should definitely include scientific sources to make this a valid paper
    * The examiner noted that the paper outline seems interesting but that I need to take care to include scientific sources
* Explanation why the listed security risks are security risks

## Slow loading times (Browser debug tools)
Mention works for:
* rustfmt's website shows that fast loading times are possible -> analysis

## Running benchmarks

Hello

-->
