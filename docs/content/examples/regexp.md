## Description
Example regular expressions can be added to paths and payloads.

The expressions here are how they must be given in the configuration. This means
backslashes must escaped.
Example
In the daemon configuration it must written like this. In the ui it will be written \?

## Method expressions
|Path expression in daemon|Description|
|^/test$|Looks for an exact match for /test|
|^/test\\?opt\\=1$|Looks for example path /test?opt=1|
|^/test/[0-9]*/list$|Looks for example path /test/900/list|

## Body expressions
|Body expression json|Description|
|\\\"test\\\"\\:\\s?\\\"900\\\"|Testing for json element test has value 900|





