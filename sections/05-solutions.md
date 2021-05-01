# Setting HTTP header fields \label{sec:solutions}
The analysis has shown that the aspects to improve can be split into two parts. First the addition of HTTP headers for security and secondly the optimization of the website content for faster loading times. These two will be investigated individually.

The measurement in chapter \ref{sec:measurement} has shown that three HTTP headers have not been set. The section \ref{sec:analysis} determined that they should be configured as defined in Table \ref{tab:solution.http-header.target-values}.

\input{sections/05-solutions/01-header-values-table.tex}

This chapter will investigate how these values can be set for Clippy's lint list.

## GitHub Pages configuration 
The GitHub Pages documentation does not contain any information if and how HTTP header can be set. There has been requests to support user defined HTTP headers in several places by the GitHub community. All of them have concluded with the answer that this is currently not possible (\cite{github.no-set-header.ref1}, \cite{github.no-set-header.ref2}, \cite{github.no-set-header.ref3}).

Searching in the documentation for the header functionality reveals that GutHub Pages provides an option called "Enforce HTTPS" [@github.docs.enforce-https]. This option can be enabled for each hosted site, under the condition that the original `github.io` domain is used [@github.docs.enforce-https]. Putting this setting to the test under a personal fork of the rust-clippy project reveals that the effect is limited. Requesting the project domain over HTTP results in a _301 Moved Permanently_ responds that forwards the browser to the same domain using HTTPS. The Strict-Transport-Security header which could enforce this behavior by the client is not set. The responds for the test page is included in attachment \ref{att:http-response-enforce-https}. This forward message only works for the root project url, other resources and direct HTML pages can still be loaded without an encrypted connection. Clippy uses paths to display version specific documentation. This setting is therefor not helpful in enforcing HTTPS security for Clippy.

The GutHub Pages documentation currently does not contain any information regarding the other header options.

## HTML meta tag
A discussion on the topic of setting HTTP header fields in GutHub Pages included suggestions to use a meta tag inside the main html file header [@github.no-set-header.ref1]. The meta tag is part of the living HTML standard defined by the _Web Hypertext Application Technology Working Group_ (_WHATWG_). It can be used to add supplementary information for the client. The tag can contain a `http-equiv` attribute with a linked `content` attribute that can define values that would usually be set in the HTTP response header. The standard currently defines a set of fields that can be set with the meta attribute. The in focussed fields defined in Table \ref{tab:solution.http-header.target-values} are not listed in the living standard [@whatwg.html-living-standard]. However, clients can still deviate from this standard or support additional functionality that has not yet been specified. 

Putting the meta tag to the test reveals that both Firefox and Chromium accept values for Strict-Transport-Security and X-Content-Type-Options. Assigning a value to X-Frame-Options produces a warning in the Chromium console with the message that this option is not supported and should be set as a HTTP header. After setting the meta tags both browsers take care to enforce HTTPS connections for all requested resources. Accessing a HTTP connection produces in both cases an error message indicating that mixed content is not allowed and the request has been blocked.

The meta tag can therefor be used to define Strict-Transport-Security and X-Content-Type-Options fields for individual websites and with that increase security. The max-time defined in the Strict-Transport-Security header also ensures that future accesses to the website will use HTTPS. This solution still has four drawbacks:

* The header to enforce HTTPS is only set during the loading of the page. An attacker could therefor still modify the page and remove the tag if they catch the initial request where HTTPS is not yet enforced.
* The meta tag to set these headers is not yet fully specified and can still change. The fact that Chromium and Firefox both accept these headers is an additional functionality.
* The X-Frame-Options header can not be set via a meta tag. This header is as discussed in \ref{sec:analysis.header.x-frame-options.value} the least important for now but still relevant when it comes to the Mozilla Observatory rating.
* These meta tags are defined in HTML files, it will therefor not increase the Mozilla Observatory scoring and they have to be added to each project in each html file to ensure that they are enforces in the project.

## Content Delivery Network
The Rust Infrastructure Team has noticed that some hosting providers have limitations when it comes to available configuration. The team has therefor setup a _CloudFront_ account for rust projects [@rust-forge.static-websites]. CloudFront is a _content delivery network_ (_CDN_) provided by Amazon. It can be used to deliver content on a global scale by acting as an intermediary agent. Each content request is wired over the network, the network then saves the data in caches to speedup future access [@amazon.cloundfront.readme]. Using a content delivery network enables the definition of custom behavior this can for example be used to define additional HTTP headers [@rust-forge.static-websites].

CloudFront is already used to provide the website for the Rust project _rustup_ at [rustup.rs](https://rustup.rs/). That website archives the highest grade of A+ when evaluated with Mozilla Observatory. The rating is included in attachment number \ref{att:scan.rustup.rs.2021-05-01.json}. However, the distribution over CloudFront requires the use of a domain as the direct access to GitHub Pages can not be intercepted via a CDN.

Using CloudFront, a different hosting or content provider would, as seen in the rustup website example, improve the website rating to a possible grade of A+ if configured correctly. A valid configuration is also provided in the Rust development documentation [@rust-forge.static-websites]. This is therefor a valid solution. However, using an additional new service like CloudFront would also add some additional complexity and use up resources of the Rust project in general. These are two disadvantages that have to be put into consideration when deciding for or against the usage.
