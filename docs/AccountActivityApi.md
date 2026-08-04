# \AccountActivityApi

All URIs are relative to *https://api.x.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_account_activity_subscription**](AccountActivityApi.md#create_account_activity_subscription) | **POST** /2/account_activity/webhooks/{webhook_id}/subscriptions/all | Create subscription
[**delete_account_activity_subscription**](AccountActivityApi.md#delete_account_activity_subscription) | **DELETE** /2/account_activity/webhooks/{webhook_id}/subscriptions/{user_id}/all | Delete subscription
[**get_account_activity_subscription_count**](AccountActivityApi.md#get_account_activity_subscription_count) | **GET** /2/account_activity/subscriptions/count | Get Account Activity Subscription Count
[**get_account_activity_subscriptions**](AccountActivityApi.md#get_account_activity_subscriptions) | **GET** /2/account_activity/webhooks/{webhook_id}/subscriptions/all/list | Get Account Activity Subscriptions
[**validate_account_activity_subscription**](AccountActivityApi.md#validate_account_activity_subscription) | **GET** /2/account_activity/webhooks/{webhook_id}/subscriptions/all | Validate Account Activity Subscription



## create_account_activity_subscription

> models::CreateAccountActivitySubscriptionResponse create_account_activity_subscription(webhook_id, body)
Create subscription

Creates an Account Activity subscription for the user and the given webhook.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** |  | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**models::CreateAccountActivitySubscriptionResponse**](CreateAccountActivitySubscriptionResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_account_activity_subscription

> models::DeleteAccountActivitySubscriptionResponse delete_account_activity_subscription(webhook_id, user_id)
Delete subscription

Deletes an Account Activity subscription for the given webhook and user ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |

### Return type

[**models::DeleteAccountActivitySubscriptionResponse**](DeleteAccountActivitySubscriptionResponse.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_activity_subscription_count

> models::GetAccountActivitySubscriptionCountResponse get_account_activity_subscription_count()
Get Account Activity Subscription Count

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetAccountActivitySubscriptionCountResponse**](GetAccountActivitySubscriptionCountResponse.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_activity_subscriptions

> models::GetAccountActivitySubscriptionsResponse get_account_activity_subscriptions(webhook_id)
Get Account Activity Subscriptions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** |  | [required] |

### Return type

[**models::GetAccountActivitySubscriptionsResponse**](GetAccountActivitySubscriptionsResponse.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_account_activity_subscription

> models::ValidateAccountActivitySubscriptionResponse validate_account_activity_subscription(webhook_id)
Validate Account Activity Subscription

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** |  | [required] |

### Return type

[**models::ValidateAccountActivitySubscriptionResponse**](ValidateAccountActivitySubscriptionResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

