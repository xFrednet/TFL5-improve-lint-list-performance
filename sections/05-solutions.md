# Solutions for unfulfilled specifications
The analysis has shown that the aspects to improve can be split into two parts. First the addition of HTTP headers for security and secondly the optimization of the website content for faster loading times. These two will be investigated individually.

## HTTP Header fields
The analysis has shown that three HTTP headers have not been set and determined that they should be configured as defined in Table \ref{tab:solution.http-header.target-values}.

\input{sections/05-solutions/01-header-values-table.tex}

### GitHub Pages configuration 
The GitHub Pages documentation does not contain any information if and how HTTP header can be set. There has been requests to support user defined HTTP headers in several places by the GitHub community. All of them have concluded with the answer that this is currently not possible (\cite{github.no-set-header.ref1}, \cite{github.no-set-header.ref2}, \cite{github.no-set-header.ref3}).

Searching in the documentation for the header functionality reveals that GutHub Pages provides an option called "Enforce HTTPS" [@github.docs.enforce-https]. This option can be enabled for each hosted site, under the condition that the original `github.io` domain is used [@github.docs.enforce-https]. Putting this setting to the test under a personal fork of the rust-clippy project reveals that the effect is limited. Requesting the project domain over HTTP results in a _301 Moved Permanently_ responds that forwards the browser to the same domain using HTTPS. The Strict-Transport-Security header which could enforce this behavior by the client is not set. The responds for the test page is included in attachment \ref{att:http-response-enforce-https}. This forward message only works for the root project url, other resources and direct HTML pages can still be loaded without an encrypted connection. Clippy uses paths to display version specific documentation. This setting is therefor not helpful in enforcing HTTPS security for Clippy.

The documentation currently does not contain any information regarding the other header options.

### HTML meta tag
These can also be set using the HTML meta tag. Better then nothing but still problematic as an attacker could remove them with a man in the middle attack

* All header fields can be looked at at once
* Reading GH page documentation

### Access over Proxy
* Maybe cloud front (But GH still links the environments directly)
* Possible, but extra service that has to be maintained. 
* GH directly links to the github.io page therefor not all solved
* Old references will continue to link to io domain

## Slow loading times
Well optimize with the use of static content