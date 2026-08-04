# \ConnectionsApi

All URIs are relative to *https://api.x.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_all_connections**](ConnectionsApi.md#delete_all_connections) | **DELETE** /2/connections/all | Terminate all connections
[**delete_connections_by_endpoint**](ConnectionsApi.md#delete_connections_by_endpoint) | **DELETE** /2/connections/{endpoint_id} | Terminate connections by endpoint
[**delete_connections_by_uuids**](ConnectionsApi.md#delete_connections_by_uuids) | **DELETE** /2/connections | Terminate multiple connections
[**get_connection_history**](ConnectionsApi.md#get_connection_history) | **GET** /2/connections | Get Connection History



## delete_all_connections

> models::DeleteAllConnectionsResponse delete_all_connections()
Terminate all connections

Terminates all active streaming connections for the authenticated application.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::DeleteAllConnectionsResponse**](DeleteAllConnectionsResponse.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_connections_by_endpoint

> models::DeleteConnectionsByEndpointResponse delete_connections_by_endpoint(endpoint_id)
Terminate connections by endpoint

Terminates all streaming connections for a specific endpoint ID for the authenticated application.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**endpoint_id** | **String** |  | [required] |

### Return type

[**models::DeleteConnectionsByEndpointResponse**](DeleteConnectionsByEndpointResponse.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_connections_by_uuids

> models::DeleteConnectionsByUuidsResponse delete_connections_by_uuids(delete_connections_by_uuids_request)
Terminate multiple connections

Terminates multiple streaming connections by their UUIDs for the authenticated application.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_connections_by_uuids_request** | [**DeleteConnectionsByUuidsRequest**](DeleteConnectionsByUuidsRequest.md) |  | [required] |

### Return type

[**models::DeleteConnectionsByUuidsResponse**](DeleteConnectionsByUuidsResponse.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_connection_history

> models::GetConnectionHistoryResponse get_connection_history(status, endpoints, max_results, pagination_token, connection_fields)
Get Connection History

Returns active and historical streaming connections with disconnect reasons for the authenticated application.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | Option<**String**> |  |  |[default to active]
**endpoints** | Option<[**Vec<String>**](String.md)> |  |  |
**max_results** | Option<**i32**> |  |  |[default to 10]
**pagination_token** | Option<**String**> |  |  |
**connection_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Connection fields to display. |  |

### Return type

[**models::GetConnectionHistoryResponse**](GetConnectionHistoryResponse.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

