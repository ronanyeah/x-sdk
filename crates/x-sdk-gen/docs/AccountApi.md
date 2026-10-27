# \AccountApi

All URIs are relative to *https://api.x.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ensure_account**](AccountApi.md#ensure_account) | **POST** /2/account | Ensure developer account
[**get_developer_account**](AccountApi.md#get_developer_account) | **GET** /2/account | Get developer account



## ensure_account

> models::EnsureAccountResponse ensure_account(ensure_account_request)
Ensure developer account

Creates an X Developer Platform account for the authenticated user if they do not already have one, and returns it. Idempotent: when the user already has an account it is returned unchanged with `created` false.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ensure_account_request** | [**EnsureAccountRequest**](EnsureAccountRequest.md) |  | [required] |

### Return type

[**models::EnsureAccountResponse**](EnsureAccountResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_developer_account

> models::GetDeveloperAccountResponse get_developer_account()
Get developer account

Returns the authenticated user's X Developer Platform account. A user without an account gets a resource-not-found error entry.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetDeveloperAccountResponse**](GetDeveloperAccountResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

