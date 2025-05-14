## Description
This creates a simple http server on port 8080.
```"httpPort": 8080,```

It responds on method GET, all input bodies and on path / .
```
"pathExpression": "/",
"bodyExpression": null,
"method": "GET",
```

It will route to an apache webserver on port 80
```
"route": {
    "configuration": {
    "url": "http://localhost:80",
    "http1Only": false,
    "acceptInvalidCerts": false,
    "acceptInvalidHostnames": false,
    }
}
```

## Running and testing the setup
```
apinae --file <FILE> --id 1747239324846
```

```
curl -i http://localhost:8080
```

returns
```
HTTP/1.1 200 OK
content-length: 481
date: Wed, 14 May 2025 20:27:47 GMT
content-type: text/html;charset=ISO-8859-1
server: Apache/2.4.63 (Unix)
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
              "pathExpression": "/**",
              "bodyExpression": null,
              "method": "GET",
              "endpointType": {
                "route": {
                  "configuration": {
                    "url": "http://localhost:80",
                    "http1Only": false,
                    "acceptInvalidCerts": false,
                    "acceptInvalidHostnames": false
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