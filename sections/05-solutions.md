# Solutions for unfulfilled specifications
The analysis has shown that the aspects to improve can be split into two parts. First the addition of HTTP headers for security and secondly the optimization of the website content for faster loading times. These two will be investigated individually.

## HTTP Header fields
The analysis has shown that three HTTP headers have not been set and determined that they should be configured as defined in Table \ref{tab:solution.http-header.target-values}.

\input{sections/05-solutions/01-header-values-table.tex}

### GitHub pages configuration 
Currently not possible. See https://stackoverflow.com/questions/14798589/github-pages-http-headers . Find and link the issue about the header values.

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