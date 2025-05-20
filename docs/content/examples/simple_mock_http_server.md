## Description
This creates a simple http server on port 8080.
```"httpPort": 8080,```

It responds on method GET, all input bodies and on path / .
"pathExpression": "/",
"bodyExpression": null,
"method": "GET",

It will respond with the following body
```
{
  "id": "1",
  "description": "Description text"
}
```
and
header ```Content-Type: application/json```

It will respond with no wait because of
```"delay": 0```

## Running and testing the setup
```apinae --file <FILE> --id 1747239324846```

```curl -i http://localhost:8080 ```

returns
```
HTTP/1.1 200 OK
content-length: 52
content-type: application/json
date: Wed, 14 May 2025 16:43:34 GMT

{
  "id": "1",
  "description": "Description text"
}
```


## Full configuration
```
{
  "name": "Untitled",
  "description": "",
  "setups": [
    {
      "id": "1747239324846",
      "name": "Untitled",
      "description": "",
      "servers": [
        {
          "id": "1747239330993",
          "name": "Http server",
          "httpPort": 8080,
          "endpoints": [
            {
              "id": "1747239489034",
              "pathExpression": "/",
              "bodyExpression": null,
              "method": "GET",
              "endpointType": {
                "mock": {
                  "configuration": {
                    "response": "{\n  \"id\": \"1\",\n  \"description\": \"Description text\"\n}",
                    "status": "200",
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
      ],
      "listeners": [],
      "params": null,
      "predefinedParams": null
    }
  ]
}
```