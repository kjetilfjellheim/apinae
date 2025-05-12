---
weight: 40
---

## Root
| Property | Type | Required | Description | 
| --- | --- | --- | --- |
| name | string | true | Name of configuration. |
| description | string | true | Description of the configuration set. |
| setups | array | true | Array of setup objects. |

### Setup
| Property | Type | Required | Description | 
| --- | --- | --- | --- |
| id | string | true | Unique identifier of the setup. |
| name | string | true | Name of setup. This is just for your conveniance. |
| description | string | true | Description of the setup. |
| servers | array | true | Array of https servers started for this setup. |
| listeners | array | true | Array of tcp listeners started for this setup. |
| params | array(string) | true |Array of named parameters used in this setup. |
| predefinedParams | array | true | Array of predefined parameter sets. |

## Http server
| Property | Type | Required | Description | 
| --- | --- | --- | --- |
| id | string | true | Unique identifier of the http server. |
| name | string | true | Name of http server. This is just for your conveniance. |
| httpPort | int | false | Optional http port. If used then a simple http server is started. |
| httpsConfig | object | false | Optional https configuration. If defined a https server is started. |
| endpoints | array | false | Array of endpoint configurations. |

## Https configuration for http server
| Property | Type | Required | Description | 
| --- | --- | --- | --- |
| id | string | true | Unique identifier of the http server. |
| serverCertificate | string | true | Server certificate pem file. |
| privateKey | string | true | Servers private key in pem format. |
| httpsPort | int | true | Port used for https server |
| clientCertificate | string | false | Optional ca certificate from the client. |
| supportedTlsVersions | array | false | Supported tls versions supported. |

## Endpoints
Endpoints can either be routed or mocked.
### Mock 
| Property | Type | Required | Description | 
| --- | --- | --- | --- |
| response | string | false | Response data. |
| status | int | true | Response statuscode. |
| headers | hashmap | true | Headers written in the response. |
| delay | int | true | Delay in ms between request read and response written. |
### Route 
| Property | Type | Required | Description | 
| --- | --- | --- | --- |
| url | string | true | Url to route to. |
| proxyUrl | int | false | Optional proxy url.. |
| http1Only | hashmap | true | Only support http1. |
| acceptInvalidCerts | int | true | Should invalid server certificates be accepted. |
| acceptInvalidHostnames | int | true | If https should invalid hostnames be accepted. |
| minTlsVersion | int | true | Supported TLS version. Either 1.2 or 1.3. |
| maxTlsVersion | int | true | Supported TLS version. Either 1.2 or 1.3. |
| readTimeout | int | true | Read timeout |
| connectTimeout | int | true | Connection timeout |
| delayBefore | int | false | Delay in ms before request is sent. |
| delayAfter | int | false | Delay in ms after response is received. |

## Tcp listener
| Property | Type | Required | Description | 
| --- | --- | --- | --- |
| id | string | true | Unique identifier of the tcp listener. |
| file | string | false | File used for response. Required if it's binary data. |
| data | string | false | Return data in text format. |
| delayWriteMs | int | false | Delay in ms from request received to response written. |
| port | int | true | Port used for listener. |
| supportedTlsVersions | array | false | Supported tls versions supported. |
| accept | bool | true | Should connection be accepted. |
| closeConnection | string | true | Should connection be closed before read, after read, after write and never. |

## Predefined params
| Property | Type | Required | Description | 
| --- | --- | --- | --- |
| name | string | true | Name of the predefined set. |
| values | HashMap(String,String) | true | Values for the parameters. |