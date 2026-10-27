# \ActivityApi

All URIs are relative to *https://api.x.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_activity_subscription**](ActivityApi.md#create_activity_subscription) | **POST** /2/activity/subscriptions | Create X activity subscription
[**delete_activity_subscription**](ActivityApi.md#delete_activity_subscription) | **DELETE** /2/activity/subscriptions/{subscription_id} | Deletes X activity subscription
[**delete_activity_subscriptions_by_ids**](ActivityApi.md#delete_activity_subscriptions_by_ids) | **DELETE** /2/activity/subscriptions | Delete X activity subscriptions by IDs
[**get_activity_subscriptions**](ActivityApi.md#get_activity_subscriptions) | **GET** /2/activity/subscriptions | Get X activity subscriptions
[**update_activity_subscription**](ActivityApi.md#update_activity_subscription) | **PUT** /2/activity/subscriptions/{subscription_id} | Update X activity subscription



## create_activity_subscription

> models::CreateActivitySubscriptionResponse create_activity_subscription(create_activity_subscription_request)
Create X activity subscription

Creates a subscription for an X activity event. OAuth2 user-context tokens must hold the scope matching the requested event_type: dm.read for chat.* and dm.* events, like.read for like.* events, mute.read for mute.* events, block.read for block.* events, follows.read for subscriptions.* events, and tweet.read for all other event types. Mute and block subscriptions are actor-only: filter.user_id must identify the authenticated user and direction is not supported. subscriptions.* events require a user-context token and filter.user_id must identify the authenticated user. Optional filter.qualifiers further narrow supported chat, follow, unfollow, like, and subscriptions events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_activity_subscription_request** | [**CreateActivitySubscriptionRequest**](CreateActivitySubscriptionRequest.md) |  | [required] |

### Return type

[**models::CreateActivitySubscriptionResponse**](CreateActivitySubscriptionResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_activity_subscription

> models::DeleteActivitySubscriptionResponse delete_activity_subscription(subscription_id)
Deletes X activity subscription

Deletes a subscription for an X activity event

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **String** |  | [required] |

### Return type

[**models::DeleteActivitySubscriptionResponse**](DeleteActivitySubscriptionResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_activity_subscriptions_by_ids

> models::DeleteActivitySubscriptionsByIdsResponse delete_activity_subscriptions_by_ids(ids)
Delete X activity subscriptions by IDs

Deletes multiple subscriptions for X activity events by their IDs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) |  | [required] |

### Return type

[**models::DeleteActivitySubscriptionsByIdsResponse**](DeleteActivitySubscriptionsByIdsResponse.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_activity_subscriptions

> models::GetActivitySubscriptionsResponse get_activity_subscriptions(max_results, pagination_token, user_id)
Get X activity subscriptions

Get a list of active subscriptions for XAA. Optional user_id narrows the list to subscriptions whose filter.user_id matches; with a user-context token it must identify the authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**max_results** | Option<**i32**> |  |  |[default to 1000]
**pagination_token** | Option<**String**> | A base32hex-encoded pagination token that decodes to 9 bytes. |  |
**user_id** | Option<**String**> | Only return subscriptions whose filter.user_id matches. |  |

### Return type

[**models::GetActivitySubscriptionsResponse**](GetActivitySubscriptionsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_activity_subscription

> models::UpdateActivitySubscriptionResponse update_activity_subscription(subscription_id, update_activity_subscription_request)
Update X activity subscription

Updates a subscription for an X activity event

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_id** | **String** |  | [required] |
**update_activity_subscription_request** | [**UpdateActivitySubscriptionRequest**](UpdateActivitySubscriptionRequest.md) |  | [required] |

### Return type

[**models::UpdateActivitySubscriptionResponse**](UpdateActivitySubscriptionResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

