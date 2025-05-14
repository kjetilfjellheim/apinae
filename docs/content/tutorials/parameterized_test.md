## Description
This tutorial shows how to use parameters in the test. Parameters are used to create a single response which changes
based on the parameters given when starting the setup.

## Setup
Server setup.

```
        {
          "id": "1747239330993",
          "name": "Http server",
          "httpPort": 8080,
          "endpoints": [
            {
              "id": "1747255333650",
              "pathExpression": "/**",
              "bodyExpression": null,
              "method": "GET",
              "endpointType": {
                "mock": {
                  "configuration": {
                    "response": "{\n  \"errorcode\": \"${errorcode}\",\n  \"errortext\": \"${errortext}\"\n}",
                    "status": "${statuscode}",
                    "headers": {
                      "Content-Type": "application/json"
                    },
                    "delay": 0
                  }
                }
              }
            }
          ],
          "httpsConfig": null
        }
```

There are now three parameters defined in the setup.
- ${errorcode}
- ${errortext}
- ${statuscode}

These will be replaced with the values given as input arguments.

## Parameters
These parameters are required, it not given no replacements will be done. They must have the same names as given for the replacement texts ${<PARAMETER_NAME>}.

```
"params": [
   "errortext",
   "statuscode",
   "errorcode"
],
```

## Starting the daemon

Starting the daemon without arguments will cause the daemon to fail.
```
apinae --file <FILE> --id 1747239324846
```
```
Error: CouldNotFind("Missing parameter: errorcode")
```

To get the daemon to run key values must be given.
```
apinae --file /home/kjetil/Documents/test.apinae --id 1747239324846 \
--param errorcode=10 --param errortext="Not found" --param statuscode=404 
```

The daemon now starts. Running the request below
```
curl -i http://localhost:8080
```

the following is returned.
```
HTTP/1.1 404 Not Found
content-length: 51
content-type: application/json
date: Wed, 14 May 2025 20:59:03 GMT

{
  "errorcode": "10",
  "errortext": "Not found"
}
```

## Conclusion
Parameterized tests allows us to reuse the same setup for multiple scenarios.