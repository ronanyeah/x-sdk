# \UsageApi

All URIs are relative to *https://api.x.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_usage**](UsageApi.md#get_usage) | **GET** /2/usage/tweets | Get Usage



## get_usage

> models::GetUsageResponse get_usage(days, usage_fields)
Get Usage

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**days** | Option<**i32**> |  |  |[default to 7]
**usage_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Usage fields to display. |  |

### Return type

[**models::GetUsageResponse**](GetUsageResponse.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

