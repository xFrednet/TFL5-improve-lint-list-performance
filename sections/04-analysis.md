# Analysis of benchmark results \label{sec:analysis}
This chapter will inspect each identified technical problem from section \ref{sec:measurement}. The inspection will first explain the technical background behind the problem and then identify the optimal configuration.

The observatory scan focuses on HTTP header which are set by the server behind the domain. The scan was therefor conducted for the domain `rust-lang.github.io`. Clippy's lint list is indirectly included in this result as well as documentation from repositories by the _Rust Organization_. Further investigation will continue to focus on the context of Clippy's lint list however improvements to the server would directly improve other websites.

* TODO xFrednet 2021-04-29: Move section about clippy hosting to specification
* TODO xFrednet 2021-04-29: HTTP explanation/into header stuff

## HTTP Strict-Transport-Security (HSTS) \label{sec:analysis.header.strict-transport-security.value}
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

## X-Frame-Options (XFO) \label{sec:analysis.header.x-frame-options.value}
This header was initially implemented by browsers as a non-standard HTTP header field as a new security measure to prevent the thread clickjacking. In 2013 the header was formalized by the _Internet Engineering Task Force_ (_IETF_) in RFC7014. Clickjacking describes is the act of hijacking clicks of the user, this can be done by embedding a website that should be hijacks as a frame and than getting the user to unknowingly interact with that site. The XFO header field allows a host to specify that delivered content must not be displayed in a frame [@ietf.rfc7034, p. 3].

### Variations
The option can be set to three mutually exclusive values [@ietf.rfc7034, p. 4]:

* _DENY_: Indicates that the content should not be displayed in any frame.
* _SAMEORIGIN_: Allows the display of the content inside a frame as long as it originated from the same origin as the frame.
* _ALLOW-FROM_: This prohibits the display of the content with the exception of the origins that are defined after the "ALLOW-FROM" value.

### Importance for Clippy's lint list
Clickhijacking is used to make a victim interacts with a different website to use the privileges or data that the user has saved on that site. Clippy's lint list provides the same data to everyone and the only user specific data is the selected color theme. An attacker has therefor nothing to gain with this attack. Adding the header would actually reduce flexibility from external users to embed the lint list in their own interface, even if the project at this point doesn't know of website doing so.

However, Clippy's lint list is just one site that's hosted under the domain, it should be investigated if other sites contain sensitive data that would require the header. This paper will still look into setting the header as it is required so receive a A+ grade by Mozilla Observatory. The goal will therefor be to set the header to _DENY_ this can later be expanded to _SAMEORIGIN_ or _ALLOW-FROM_ if required.

## X-Content-Type-Options \label{sec:analysis.header.x-content-type-options.value}
In 2008 the _X-Content-Type-Options_ HTTP header was initially implemented by Microsoft in Internet Explorer 8 to prevent attacks that abuse _MIME-sniffing_ for attacks [@microsoft.docs.ie8-security-4]. HTTP includes a content-type header that indicate the type of content that is being delivered, these types are called _MIME types_. Most browsers have a mechanic called _MIME-sniffing_ to determine what MIME type the received resource is in. This functionality is used for backwards compatibility with for example legacy servers that serve all content with the `text/plain` content type. MIME-Sniffing can determine that received data is in a different data type than specified and display it in the determined way. This would for instance render a HTML document that is send with the `text/plain` content-type if the text contains HTML elements [@microsoft.docs.ie8-security-5].

The feature has however introduced some security concerns for content hosts. Attackers could create content like images that contain HTML text with scrips. The sniffing functionality could then falsely determine during the inspection that the received resource is a HTML document and then execute the contained script instead of showing an image [@microsoft.docs.ie8-security-5]. This lead to the introduction of the X-Content-Type-Options field that can be used to prevent such content sniffing [@microsoft.docs.ie8-security-4].

The header can only be set to `nosniff` which disables the sniffing feature. It is supported by all major browsers [@mozilla.developer.content-type-opt].

### Importance for Clippy's lint list
This field can actually be of high importance to the project. Clippy like all Rust projects has a review policy that only allows the merge of changes if they have been reviewed by a project member. This type of attack especially focusses on hiding the malicious code inside an image, this could therefor also easily be overlooked during the review process. Additionally due to the fact that the project maintainers mainly focus on Rust and not the website.

This header requires that the `content-type` header is set correctly for content that is being delivered by the host. GitHub pages doesn't support the manual specification of the content type it instead uses a open source database to determine the correct MIME type based on the file extension [@github.docs.about-pages]. Clippy's lint list is composed out of a _html_ and a _json_ which both are delivered withe the correct content type as can be seen in attachment \ref{att:http-response-header-html} and \ref{att:http-response-header-json}. The nosniff option can therefor be enabled without side effects.

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
