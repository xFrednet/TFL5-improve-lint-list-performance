# Fulfillment of requirements
This chapter will measure the current fulfillment for the previously in \ref{sec:requirements} defined requirements. These results will then be used to identify the key aspects that need improvement. Additionally they will be used for comparison when testing suggestions.

## Mozilla Observatory
_Mozilla Observatory_ is a collection of tools that can analyze a website to determine which available security measures have been utilized by it [@github.observatory.readme]. These security is focussed on values that can be set in the HTTP header to indicate that opt-in security options should be enabled by the browser [@TODO]. The Rust development documentation links to a free [online interface](https://observatory.mozilla.org/) for the Mozilla Observatory that is provided by the Mozilla Foundation free of charge.

### Scoring
The result of the analyzes is summarized in a single score with a corresponding grade. The score is calculated using a baseline each checked criteria can add bonus points or subtracted penalty. This implementation is used to give different weight to specific configurations. The significance of these modifiers are based on how important the analyzed aspect for security. Scores can range from a minimum of 0 to a maximum of 135. A score of 100 and above corresponds to the grade _A+_ [@github.observatory.scoring].

The observatory documentation notes that all websites are graded equally, this means certain graded configurations might be unimportant for the specific use case [@mozilla.observatory.faq]. 

### Measurement


## Running benchmarks
1. Simple _Mozilla Observatory_
2. Load times -> difficult
      * Comparing only on one device as this load time is significant and we want significant improvements
      * rustfmt's website shows that fast loading times are possible

## Summary of benchmarks

Hello, was geht?