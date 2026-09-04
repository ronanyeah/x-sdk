# \WebhooksApi

All URIs are relative to *https://api.x.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_webhook_replay_job**](WebhooksApi.md#create_webhook_replay_job) | **POST** /2/webhooks/replay | Create replay job for webhook
[**create_webhooks**](WebhooksApi.md#create_webhooks) | **POST** /2/webhooks | Create webhook
[**create_webhooks_stream_link**](WebhooksApi.md#create_webhooks_stream_link) | **POST** /2/tweets/search/webhooks/{webhook_id} | Create stream link
[**delete_webhooks**](WebhooksApi.md#delete_webhooks) | **DELETE** /2/webhooks/{webhook_id} | Delete webhook
[**delete_webhooks_stream_link**](WebhooksApi.md#delete_webhooks_stream_link) | **DELETE** /2/tweets/search/webhooks/{webhook_id} | Delete stream link
[**get_webhooks**](WebhooksApi.md#get_webhooks) | **GET** /2/webhooks | Get webhook
[**get_webhooks_stream_links**](WebhooksApi.md#get_webhooks_stream_links) | **GET** /2/tweets/search/webhooks | Get stream links
[**validate_webhooks**](WebhooksApi.md#validate_webhooks) | **PUT** /2/webhooks/{webhook_id} | Validate webhook



## create_webhook_replay_job

> models::CreateWebhookReplayJobResponse create_webhook_replay_job(create_webhook_replay_job_request)
Create replay job for webhook

Creates a replay job to retrieve events from up to the past 24 hours for all events delivered or attempted to be delivered to the webhook.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_webhook_replay_job_request** | [**CreateWebhookReplayJobRequest**](CreateWebhookReplayJobRequest.md) |  | [required] |

### Return type

[**models::CreateWebhookReplayJobResponse**](CreateWebhookReplayJobResponse.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_webhooks

> models::CreateWebhooksResponse create_webhooks(create_webhooks_request)
Create webhook

Creates a new webhook configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_webhooks_request** | [**CreateWebhooksRequest**](CreateWebhooksRequest.md) |  | [required] |

### Return type

[**models::CreateWebhooksResponse**](CreateWebhooksResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_webhooks_stream_link

> models::CreateWebhooksStreamLinkResponse create_webhooks_stream_link(webhook_id)
Create stream link

Creates a link to deliver FilteredStream events to the given webhook.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** |  | [required] |

### Return type

[**models::CreateWebhooksStreamLinkResponse**](CreateWebhooksStreamLinkResponse.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_webhooks

> models::DeleteWebhooksResponse delete_webhooks(webhook_id)
Delete webhook

Deletes an existing webhook configuration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** |  | [required] |

### Return type

[**models::DeleteWebhooksResponse**](DeleteWebhooksResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_webhooks_stream_link

> models::DeleteWebhooksStreamLinkResponse delete_webhooks_stream_link(webhook_id)
Delete stream link

Deletes the link delivering FilteredStream events to the given webhook.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** |  | [required] |

### Return type

[**models::DeleteWebhooksStreamLinkResponse**](DeleteWebhooksStreamLinkResponse.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webhooks

> models::GetWebhooksResponse get_webhooks(webhook_config_fields)
Get webhook

Get a list of webhook configs associated with a client app.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_config_fields** | Option<[**Vec<models::WebhookConfigField>**](Models__WebhookConfigField.md)> | A comma separated list of WebhookConfig fields to display. |  |

### Return type

[**models::GetWebhooksResponse**](GetWebhooksResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_webhooks_stream_links

> models::GetWebhooksStreamLinksResponse get_webhooks_stream_links()
Get stream links

Get a list of webhook links associated with a filtered stream ruleset.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetWebhooksStreamLinksResponse**](GetWebhooksStreamLinksResponse.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_webhooks

> models::ValidateWebhooksResponse validate_webhooks(webhook_id)
Validate webhook

Triggers a CRC check for a given webhook.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_id** | **String** |  | [required] |

### Return type

[**models::ValidateWebhooksResponse**](ValidateWebhooksResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

