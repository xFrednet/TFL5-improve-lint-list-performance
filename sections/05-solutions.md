# Solutions for unfulfilled specifications
The analysis has shown that the aspects to improve can be split into two parts. First the addition of HTTP headers for security and secondly the optimization of the website content for faster loading times. These two will be investigated individually.

## HTTP header fields
The analysis has shown that three HTTP headers have not been set and determined that they should be configured as defined in Table \ref{tab:solution.http-header.target-values}.

\input{sections/05-solutions/01-header-values-table.tex}

### GitHub Pages configuration 
The GitHub Pages documentation does not contain any information if and how HTTP header can be set. There has been requests to support user defined HTTP headers in several places by the GitHub community. All of them have concluded with the answer that this is currently not possible (\cite{github.no-set-header.ref1}, \cite{github.no-set-header.ref2}, \cite{github.no-set-header.ref3}).

Searching in the documentation for the header functionality reveals that GutHub Pages provides an option called "Enforce HTTPS" [@github.docs.enforce-https]. This option can be enabled for each hosted site, under the condition that the original `github.io` domain is used [@github.docs.enforce-https]. Putting this setting to the test under a personal fork of the rust-clippy project reveals that the effect is limited. Requesting the project domain over HTTP results in a _301 Moved Permanently_ responds that forwards the browser to the same domain using HTTPS. The Strict-Transport-Security header which could enforce this behavior by the client is not set. The responds for the test page is included in attachment \ref{att:http-response-enforce-https}. This forward message only works for the root project url, other resources and direct HTML pages can still be loaded without an encrypted connection. Clippy uses paths to display version specific documentation. This setting is therefor not helpful in enforcing HTTPS security for Clippy.

The GutHub Pages documentation currently does not contain any information regarding the other header options.

### HTML meta tag
A discussion on the topic of setting HTTP header fields in GutHub Pages included suggestions to use a meta tag inside the main html file header [@github.no-set-header.ref1]. The meta tag is part of the living HTML standard defined by the _Web Hypertext Application Technology Working Group_ (_WHATWG_). It can be used to add supplementary information for the client. The tag can contain a `http-equiv` attribute with a linked `content` attribute that can define values that would usually be set in the HTTP response header. The standard currently defines a set of fields that can be set with the meta attribute. The in focussed fields defined in Table \ref{tab:solution.http-header.target-values} are not listed in the living standard [@whatwg.html-living-standard]. However, clients can still deviate from this standard or support additional functionality that has not yet been specified. 

Putting the meta tag to the test reveals that both Firefox and Chromium accept values for Strict-Transport-Security and X-Content-Type-Options. Assigning a value to X-Frame-Options produces a warning in the Chromium console with the message that this option is not supported and should be set as a HTTP header. After setting the meta tags both browsers take care to enforce HTTPS connections for all requested resources. Accessing a HTTP connection produces in both cases an error message indicating that mixed content is not allowed and the request has been blocked.

The meta tag can therefor be used to define Strict-Transport-Security and X-Content-Type-Options fields for individual websites and with that increase security. The max-time defined in the Strict-Transport-Security header also ensures that future accesses to the website will use HTTPS. This solution still has four drawbacks:

* The header to enforce HTTPS is only set during the loading of the page. An attacker could therefor still modify the page and remove the tag if they catch the initial request where HTTPS is not yet enforced.
* The meta tag to set these headers is not yet fully specified and can still change. The fact that Chromium and Firefox both accept these headers is an additional functionality.
* The X-Frame-Options header can not be set via a meta tag. This header is as discussed in \ref{sec:analysis.header.x-frame-options.value} the least important for now but still relevant when it comes to the Mozilla Observatory rating.
* These meta tags are defined in HTML files, it will therefor not increase the Mozilla Observatory scoring and they have to be added to each project in each html file to ensure that they are enforces in the project.

### Access over Proxy
* Maybe cloud front (But GH still links the environments directly)
* Possible, but extra service that has to be maintained. 
* GH directly links to the github.io page therefor not all solved
* Old references will continue to link to io domain

## Slow loading times
Well optimize with the use of static content