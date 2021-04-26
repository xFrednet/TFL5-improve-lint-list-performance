# Analysis of benchmark results
This chapter will inspect each identified technical problem from section \ref{sec:measurement}. The inspection will first explain the technical background behind the problem and then identify the optimal configuration.

## HTTP Strict Transport Security (HSTS)
HTTP Strict Transport Security (HSTS) is a optional HTTP header field that requests the client accessing the HTTP API to only use encrypted connection for further requests. The request to use and encrypted connection extends to all resources that are referenced by the requested result. It is therefor necessary that these resources also provide the option to connect via HTTPS.

The specification references three threads that can be prevented using this header:

1. Using an unencryped connection allows attackers to eavesdrop on the exchanged data. This is a _passive network attack_ and can be used to collect personal information, passwords or browsing habits.
2. A HTTPS connection requires a certificate that has to be signed by a certification authority. This certificate intern lists the owner or organization. This can be used to validate that the displayed content really originates from the expected source and with that prevent attackers from creating a fake website copy to steal otherwise secure information.
3. Forcing the use of HTTPS additionally ensures that mistakes like referencing ressources via HTTP links will be corrected by the requesting client

Importance for clippy? We are Open source... But yes you can open http://rust-lang.github.io/rust-clippy/master/index.html without being forwarded. So mostly not modify



Ja ja toller text [@ietf.rfc6797]

## X-Frame-Options (XFO)
Some other explanation [@ietf.rfc7034]

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
