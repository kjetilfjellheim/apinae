## Description
This creates a simple https server on port 443.
```"httpsPort": 443,```

Generate certificate and private key.
```openssl req -x509 -nodes -new -newkey rsa:2048 -keyout ./private-key.pem -out ./certificate.pem```

Add the certificates to the https config.
```
  "serverCertificate": "./certificate.pem",
  "privateKey": "./private-key.pem",  
```

Set supported TLS versions.
```
  "supportedTlsVersions": [
    "TLSv1_3"
  ]
```

Add an endpoint so we can test the server.
```
  "endpoints": [
    {
      "id": "1747771761199",
      "pathExpression": "/",
      "bodyExpression": null,
      "method": "GET",
      "endpointType": {
        "mock": {
          "configuration": {
            "response": "OK",
            "status": "200",
            "headers": {
              "Content-Type": "text/plain"
            },
            "delay": 0
          }
        }
      }
    }
  ],
```

## Running and testing the setup
Run the test.
```sudo apinae --file <FILE> --id <ID>```

Make the request with curl
```curl --tlsv1.3 --tls-max 1.3 -i --insecure https://localhost```

It will respond with the following.
```
HTTP/2 200 
content-length: 2
content-type: text/plain
date: Tue, 20 May 2025 20:51:53 GMT
```

## Full configuration
```
{
  "name": "Untitled",
  "description": "",
  "setups": [
    {
      "id": "1747771464706",
      "name": "Test",
      "description": "",
      "servers": [
        {
          "id": "1747771487580",
          "name": "Test",
          "httpPort": null,
          "endpoints": [
            {
              "id": "1747771761199",
              "pathExpression": "/",
              "bodyExpression": null,
              "method": "GET",
              "endpointType": {
                "mock": {
                  "configuration": {
                    "response": "OK",
                    "status": "200",
                    "headers": {
                      "Content-Type": "text/plain"
                    },
                    "delay": 0
                  }
                }
              }
            }
          ],
          "httpsConfig": {
            "serverCertificate": "./certificate.pem",
            "privateKey": "./private-key.pem",
            "httpsPort": 443,
            "clientCertificate": null,
            "supportedTlsVersions": [
              "TLSv1_3"
            ]
          }
        }
      ],
      "listeners": [],
      "params": null,
      "predefinedParams": null
    }
  ]
}
```