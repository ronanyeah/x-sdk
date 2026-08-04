# \TrendsApi

All URIs are relative to *https://api.x.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_trends_by_woeid**](TrendsApi.md#get_trends_by_woeid) | **GET** /2/trends/by/woeid/{woeid} | Get Trends by Woeid
[**get_trends_personalized_trends**](TrendsApi.md#get_trends_personalized_trends) | **GET** /2/users/personalized_trends | Get Trends Personalized Trends



## get_trends_by_woeid

> models::GetTrendsByWoeidResponse get_trends_by_woeid(woeid, max_trends, trend_fields)
Get Trends by Woeid

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**woeid** | **i32** |  | [required] |
**max_trends** | Option<**i32**> |  |  |[default to 20]
**trend_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Trend fields to display. |  |

### Return type

[**models::GetTrendsByWoeidResponse**](GetTrendsByWoeidResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_trends_personalized_trends

> models::GetTrendsPersonalizedTrendsResponse get_trends_personalized_trends(personalized_trend_fields)
Get Trends Personalized Trends

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**personalized_trend_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of PersonalizedTrend fields to display. |  |

### Return type

[**models::GetTrendsPersonalizedTrendsResponse**](GetTrendsPersonalizedTrendsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

