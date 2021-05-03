# Analysis of missing response headers \label{sec:analysis}
<!-- Reviewed: 1x rewritten -->
In \ref{sec:measurement} it was determined that Clippy's lint list currently misses three HTTP response header fields to fulfill all requirements that have been defined in \ref{sec:requirements}. This chapter will inspect each of these fields individually by explaining the technical background, evaluating the relevance for Clippy's use case, and then suggest what each option should optimally be set to.

The observatory scan focuses on HTTP headers which are set by the server behind the domain. The scan was, therefore, conducted for the domain `rust-lang.github.io`. Clippy's lint list is indirectly included in this result as well as documentation from other repositories inside the rust-lang organization. Further investigation will continue to focus on the context of Clippy's lint list however changes to the server could, therefore, also indirectly improve other sites.

## HTTP Strict-Transport-Security
<!-- Reviewed: 1x rewritten -->
`Strict-Transport-Security` is an optional HTTP header field that instructs the client to only use an encrypted connection for further requests. The instruction extends to all resources that are referenced by the requested result. It is, therefore, necessary that the referenced hosts provide the option to download their resources over HTTPS [@ietf.rfc6797, p. 6ff].

This header protects the user from _passive network attacks_ where an attacker eavesdrop on the exchanged data. This can be used to collect personal information, passwords, or browsing habits. A connection that is not encrypted is also vulnerable to _active network attack_. With this, an attacker can impersonate the actual site or deliver a modified version altogether. An encrypted connection on the other hand can be used to request a certificate and validate that the content is delivered from the expected source. An additional advantage of this header is that it prevents accidental use of unencrypted connections by developers [@ietf.rfc6797, p. 6ff].

### Importance for Clippy
<!-- Reviewed: 1x rewritten -->
Clippy's lint list only displays publicly available information about lints in an easily accessible and searchable way. A passive network attack could, therefore, not collect any secret or personal information about the user. Except for the fact that they visited the domain at all. However, this would still be possible with the header as the connected IP is not affected by it. The biggest thread could actually be an active network attack that injects a donation button into the website as several developers have expressed interest to donate to the Rust Foundation in general. This button would then forward the user to another page of the attacker to donate. However, the chance of this is probably negotiable due to the low traffic that Clippy's lint list actually receives. Such an attack would, therefore, be targeted towards a specific user.

With all of this being said it has to be noted that all references to the website already include `https` at the start and a user has to deliberately enter the domain with http in front. Most browsers will then still recommend using the encrypted connection or at least add a _not encrypted_ notice next to the URL. All of this results in a very low risk. The header should still be set if the hosting provider provides a simple setting for this. Also, due to the fact that the targeted A+ rating would require this field.

### Configuration \label{sec:analysis.header.strict-transport-security.value}
<!-- Reviewed: 1x rewritten -->
The header can take up to three arguments that configure which domains are included in this instruction and a duration for how long an encrypted connection should be forced [@ietf.rfc6797, p. 14ff]. Both Mozilla and the Rust development documentation recommend setting the duration to two years in the header field. This is equivalent to the value `"max-age=63072000"` (\cite{mozilla.infosec.recommendations}, cite{rust-forge.static-websites}). This is, therefore, also the recommended value for Clippy's lint list.

## X-Frame-Options
<!-- Reviewed: 1x rewritten -->
The `X-Frame-Options` header was initially accepted by some browsers as an opt-in security measure to prevent clickjacking. In 2013 the header was formally specified by the _Internet Engineering Task Force_ (_IETF_) in RFC7014 [@ietf.rfc7034, p. 3].

HTML supports frame elements that allow a website to embed an external website into the user's view. This can be used to add a complementary view that is externally hosted or provides additional information. The parent document can for security reasons not directly interact with the framed content. Clickhijacking describes an attack where a frame is used to make a user unknowingly interact with framed content through clicks as these will be accepted by the frame as interactions. This interaction can then be used to trigger some behavior or gain access in some other way. Browsers have added support for the `X-Frame-Options` header which enables the provider to deny the display of content in frames [@ietf.rfc7034, p. 3ff]. Therefore, preventing clickhijacking all together.

### Importance for Clippy \label{sec:analysis.header.x-frame-options.importance}
<!-- Reviewed: 1x slight adjustments -->
Clickhijacking is used to make a victim interacts with a different website to use the privileges or data that the user has saved on that site. Clippy's lint list provides the same data to everyone and the only user specific data is the selected color theme. An attacker has, therefore, nothing to gain with this attack. Adding the header would actually reduce flexibility from external users to embed the lint list in their own interface, even if the project at this point does not know of a website doing so.

However, Clippy's lint list is just one site that's hosted under the domain, it should be investigated if other sites contain sensitive data that would require the header. This paper will still look into setting the header as it is required to receive the grade A+ by Mozilla Observatory.

### Configuration \label{sec:analysis.header.x-frame-options.value}
<!-- Reviewed: 1x slight adjustments -->
The option can be set to three mutually exclusive values [@ietf.rfc7034, p. 4]:

* _DENY_: Indicates that the content should not be displayed in any frame.
* _SAMEORIGIN_: Allows the display of the content inside a frame as long as it originated from the same origin as the frame.
* _ALLOW-FROM_: This prohibits the display of the content except for the origins that are defined after the "ALLOW-FROM" value.

The Rust development documentation provides an example configuration that uses `DENY` [@rust-forge.static-websites]. `DENY` is the most restrictive setting but can easily be adapted to allow framing if requested. The author for this reason suggests the initial value of `DENY` as well.

## X-Content-Type-Options
<!-- Reviewed: 1x slight adjustments -->
In 2008 the `X-Content-Type-Options` HTTP header was initially implemented by Microsoft in Internet Explorer 8 to prevent attacks that abuse _MIME-sniffing_ [@microsoft.docs.ie8-security-4]. HTTP response header includes a `content-type` field that indicates the type of content that is being delivered, these types are called _MIME types_. Most browsers have a mechanic called _MIME-sniffing_ to determine what MIME type the received resource is in. This functionality is used for backwards compatibility with legacy servers that serve all content with the `text/plain` content type. MIME-Sniffing can determine that received data is in a different data type than specified and display it in the newly determined way. This would for instance render an HTML document that is delivered with the `text/plain` content type if the text contains HTML elements [@microsoft.docs.ie8-security-5].

The feature has however introduced some security concerns for content hosts. Attackers could create content, like images, that contain HTML text with scrips. The sniffing functionality could then falsely determine during the inspection that the received resource is an HTML document and execute the contained script instead of showing an image [@microsoft.docs.ie8-security-5]. This led to the introduction of the `X-Content-Type-Options` field that can be used to disable sniffing and with that enforce the use of the specified content type [@microsoft.docs.ie8-security-4].

### Importance for Clippy's lint list
<!-- Reviewed: 1x slight adjustments -->
This field can actually be of high importance to the project. Clippy, like all Rust projects, has a review policy that only allows the merge of changes if they have been reviewed by a project member. This type of attack especially focuses on hiding the malicious code inside other resources, like an image. This could, therefore, also easily be overlooked during the review process. Additionally due to the fact that the project maintainers mainly focus on Rust and not the website.

This header requires that the `content-type` header is set correctly for content that is being delivered by the host. GitHub Pages doesn't support the manual specification of the content type. It instead uses an open-source database to determine the correct MIME type based on the file extension [@github.docs.about-pages]. Clippy's lint list is composed out of a `.html` and a `.json` file which both are delivered with the correct content type as can be seen in attachment \ref{att:http-response-header-html} and \ref{att:http-response-header-json}. The security measure option can, therefore, be enabled without side effects.

### Configuration \label{sec:analysis.header.x-content-type-options.value}
<!-- Reviewed: 1x newly written -->
The `X-Content-Type-Options` response header can only be set to `nosniff` which disables the sniffing feature altogether. This option is also supported by all major browsers [@mozilla.developer.content-type-opt]. This will, therefore, also be the suggested value to the field.

## Summary
<!-- Reviewed: 1x newly written -->
This chapter reviewed the three missing header fields that are required to gain the grade A+ by Mozilla Observatory. It was chosen a suggested value for each field that will be used for further reference in this paper. These values were chosen based on the technical background and importance for Clippy. The results are summarizes in table \ref{tab:solution.http-header.target-values}.

\input{sections/04-analysis/01-header-values-table.tex}
