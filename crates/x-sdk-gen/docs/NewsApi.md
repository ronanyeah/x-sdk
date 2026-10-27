# \NewsApi

All URIs are relative to *https://api.x.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_news**](NewsApi.md#get_news) | **GET** /2/news/{id} | Get news stories by ID
[**search_news**](NewsApi.md#search_news) | **GET** /2/news/search | Search News



## get_news

> models::GetNewsResponse get_news(id, news_fields)
Get news stories by ID

Retrieves news story by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**news_fields** | Option<[**Vec<models::NewsField>**](Models__NewsField.md)> | A comma separated list of News fields to display. |  |

### Return type

[**models::GetNewsResponse**](GetNewsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_news

> models::SearchNewsResponse search_news(query, max_results, max_age_hours, news_fields)
Search News

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** |  | [required] |
**max_results** | Option<**i32**> |  |  |[default to 10]
**max_age_hours** | Option<**i32**> |  |  |[default to 168]
**news_fields** | Option<[**Vec<models::NewsField>**](Models__NewsField.md)> | A comma separated list of News fields to display. |  |

### Return type

[**models::SearchNewsResponse**](SearchNewsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

