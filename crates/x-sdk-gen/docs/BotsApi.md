# \BotsApi

All URIs are relative to *https://api.x.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_bot**](BotsApi.md#create_bot) | **POST** /2/bots | Create a bot
[**delete_bot**](BotsApi.md#delete_bot) | **DELETE** /2/bots/{id} | Delete Bot
[**get_bots**](BotsApi.md#get_bots) | **GET** /2/bots | Get Bots
[**revoke_bot_token**](BotsApi.md#revoke_bot_token) | **DELETE** /2/bots/{id}/token | Revoke Bot Token
[**rotate_bot_token**](BotsApi.md#rotate_bot_token) | **POST** /2/bots/{id}/token | Rotate Bot Token
[**update_bot**](BotsApi.md#update_bot) | **PUT** /2/bots/{id} | Update Bot



## create_bot

> models::CreateBotResponse create_bot(create_bot_request)
Create a bot

Creates a bot account in the calling app's project and returns its bearer token. Idempotent on handle: repeating the request for a handle that already names one of the project's bots returns that bot with a freshly minted token, revoking the previous one — safe to retry when a response was lost, since the replaced token was never seen.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_bot_request** | [**CreateBotRequest**](CreateBotRequest.md) |  | [required] |

### Return type

[**models::CreateBotResponse**](CreateBotResponse.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_bot

> models::DeleteBotResponse delete_bot(id)
Delete Bot

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::DeleteBotResponse**](DeleteBotResponse.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bots

> models::GetBotsResponse get_bots(user_fields, expansions, post_fields)
Get Bots

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |
**expansions** | Option<[**Vec<models::UserExpansion>**](Models__UserExpansion.md)> | A comma separated list of fields to expand. |  |
**post_fields** | Option<[**Vec<models::PostField>**](Models__PostField.md)> | A comma separated list of Post fields to display. |  |

### Return type

[**models::GetBotsResponse**](GetBotsResponse.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_bot_token

> models::RevokeBotTokenResponse revoke_bot_token(id)
Revoke Bot Token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::RevokeBotTokenResponse**](RevokeBotTokenResponse.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rotate_bot_token

> models::RotateBotTokenResponse rotate_bot_token(id, rotate_bot_token_request)
Rotate Bot Token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**rotate_bot_token_request** | [**RotateBotTokenRequest**](RotateBotTokenRequest.md) |  | [required] |

### Return type

[**models::RotateBotTokenResponse**](RotateBotTokenResponse.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_bot

> models::UpdateBotResponse update_bot(id, update_bot_request)
Update Bot

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**update_bot_request** | [**UpdateBotRequest**](UpdateBotRequest.md) |  | [required] |

### Return type

[**models::UpdateBotResponse**](UpdateBotResponse.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

