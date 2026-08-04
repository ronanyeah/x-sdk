# \CommunitiesApi

All URIs are relative to *https://api.x.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_communities_by_id**](CommunitiesApi.md#get_communities_by_id) | **GET** /2/communities/{id} | Get Communities by ID
[**search_communities**](CommunitiesApi.md#search_communities) | **GET** /2/communities/search | Search Communities



## get_communities_by_id

> models::GetCommunitiesByIdResponse get_communities_by_id(id, community_fields)
Get Communities by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**community_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Community fields to display. |  |

### Return type

[**models::GetCommunitiesByIdResponse**](GetCommunitiesByIdResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_communities

> models::SearchCommunitiesResponse search_communities(query, max_results, next_token, pagination_token, community_fields)
Search Communities

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** |  | [required] |
**max_results** | Option<**i32**> |  |  |[default to 10]
**next_token** | Option<**String**> |  |  |
**pagination_token** | Option<**String**> |  |  |
**community_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Community fields to display. |  |

### Return type

[**models::SearchCommunitiesResponse**](SearchCommunitiesResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

