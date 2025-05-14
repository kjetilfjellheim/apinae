## Description
Receive timeouts are a normal situation which needs to be tested. Apinae supports this with the delay parameter.

## Setup

The important element is the delay. This is the delay the application waits before sending the response in milliseconds.

```
{
    "id": "1747255333650",
    "pathExpression": "/**",
    "bodyExpression": null,
    "method": "GET",
    "endpointType": {
    "mock": {
        "configuration": {
        "response": "{\n  \"errorcode\": \"10\",\n  \"errortext\": \"Not found\"\n}",
        "status": "404",
        "headers": {
            "Content-Type": "application/json"
        },
        "delay": 30000
        }
    }
    }
}
```

## Testing the setups
Start the application normally.

```
apinae --file <FILE> --id 1747239324846
```

Test the timeout by running
```
curl --max-time 5 -i http://localhost:8080
```
which results in
```
curl: (28) Operation timed out after 5002 milliseconds with 0 bytes received
```
