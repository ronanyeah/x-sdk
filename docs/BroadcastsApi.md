# \BroadcastsApi

All URIs are relative to *https://api.x.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_scheduled_broadcast**](BroadcastsApi.md#create_scheduled_broadcast) | **POST** /2/broadcasts/scheduled | Create a scheduled broadcast
[**delete_broadcast_chat_message**](BroadcastsApi.md#delete_broadcast_chat_message) | **DELETE** /2/broadcasts/{id}/chat/{message_id} | Remove a chat message from a live broadcast
[**delete_scheduled_broadcast**](BroadcastsApi.md#delete_scheduled_broadcast) | **DELETE** /2/broadcasts/scheduled/{id} | Delete a scheduled broadcast
[**get_broadcast**](BroadcastsApi.md#get_broadcast) | **GET** /2/broadcasts/{id} | Get a broadcast
[**get_broadcast_chat**](BroadcastsApi.md#get_broadcast_chat) | **GET** /2/broadcasts/{id}/chat | Get broadcast chat history
[**get_scheduled_broadcast**](BroadcastsApi.md#get_scheduled_broadcast) | **GET** /2/broadcasts/scheduled/{id} | Get a scheduled broadcast
[**go_live_scheduled_broadcast**](BroadcastsApi.md#go_live_scheduled_broadcast) | **POST** /2/broadcasts/scheduled/{id}/live | Go live on a scheduled broadcast
[**list_broadcasts**](BroadcastsApi.md#list_broadcasts) | **GET** /2/broadcasts | List broadcasts
[**list_scheduled_broadcasts**](BroadcastsApi.md#list_scheduled_broadcasts) | **GET** /2/broadcasts/scheduled | List scheduled broadcasts
[**mute_broadcast_chat_user**](BroadcastsApi.md#mute_broadcast_chat_user) | **POST** /2/broadcasts/{id}/chat/mutes | Mute or time out a user in a broadcast chat
[**send_broadcast_chat**](BroadcastsApi.md#send_broadcast_chat) | **POST** /2/broadcasts/{id}/chat | Send a chat message to a live broadcast
[**unmute_broadcast_chat_user**](BroadcastsApi.md#unmute_broadcast_chat_user) | **DELETE** /2/broadcasts/{id}/chat/mutes/{user_id} | Unmute a user in a broadcast chat
[**update_scheduled_broadcast**](BroadcastsApi.md#update_scheduled_broadcast) | **PUT** /2/broadcasts/scheduled/{id} | Update a scheduled broadcast



## create_scheduled_broadcast

> models::CreateScheduledBroadcastResponse create_scheduled_broadcast(create_scheduled_broadcast_request)
Create a scheduled broadcast

Schedules a one-off or recurring broadcast for the authenticated user. A `source_id` (ingest) is required at create time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_scheduled_broadcast_request** | [**CreateScheduledBroadcastRequest**](CreateScheduledBroadcastRequest.md) |  | [required] |

### Return type

[**models::CreateScheduledBroadcastResponse**](CreateScheduledBroadcastResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_broadcast_chat_message

> models::DeleteBroadcastChatMessageResponse delete_broadcast_chat_message(id, message_id)
Remove a chat message from a live broadcast

Removes a specific chat message from a running broadcast owned by the authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**message_id** | **String** |  | [required] |

### Return type

[**models::DeleteBroadcastChatMessageResponse**](DeleteBroadcastChatMessageResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_scheduled_broadcast

> models::DeleteScheduledBroadcastResponse delete_scheduled_broadcast(id, roll_forward)
Delete a scheduled broadcast

Deletes a scheduled broadcast by its alphanumeric UBS broadcast id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**roll_forward** | Option<**bool**> |  |  |

### Return type

[**models::DeleteScheduledBroadcastResponse**](DeleteScheduledBroadcastResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_broadcast

> models::GetBroadcastResponse get_broadcast(id, broadcast_fields)
Get a broadcast

Returns a broadcast owned by the authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**broadcast_fields** | Option<[**Vec<models::BroadcastField>**](Models__BroadcastField.md)> | A comma separated list of Broadcast fields to display. |  |

### Return type

[**models::GetBroadcastResponse**](GetBroadcastResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_broadcast_chat

> models::GetBroadcastChatResponse get_broadcast_chat(id, max_results, pagination_token, broadcast_chat_message_fields, expansions, user_fields)
Get broadcast chat history

Returns newest-first chat messages for a broadcast owned by the authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**max_results** | Option<**i32**> |  |  |[default to 100]
**pagination_token** | Option<**String**> | A 64-bit signed integer. |  |
**broadcast_chat_message_fields** | Option<[**Vec<models::BroadcastChatMessageField>**](Models__BroadcastChatMessageField.md)> | A comma separated list of BroadcastChatMessage fields to display. |  |
**expansions** | Option<[**Vec<models::BroadcastChatMessageExpansion>**](Models__BroadcastChatMessageExpansion.md)> | A comma separated list of fields to expand. |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |

### Return type

[**models::GetBroadcastChatResponse**](GetBroadcastChatResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scheduled_broadcast

> models::GetScheduledBroadcastResponse get_scheduled_broadcast(id)
Get a scheduled broadcast

Returns a single scheduled broadcast by its alphanumeric UBS broadcast id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::GetScheduledBroadcastResponse**](GetScheduledBroadcastResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## go_live_scheduled_broadcast

> models::GoLiveScheduledBroadcastResponse go_live_scheduled_broadcast(id)
Go live on a scheduled broadcast

Publishes a schedule that was created or updated with `manual_publish: true`. Without that flag the coordinator auto-publishes at start and this call is rejected.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::GoLiveScheduledBroadcastResponse**](GoLiveScheduledBroadcastResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_broadcasts

> models::ListBroadcastsResponse list_broadcasts(ids, max_results, pagination_token, broadcast_fields)
List broadcasts

Returns the authenticated user's live-video broadcasts, or selected owned broadcasts when ids are provided.  At most one of `ids`, `max_results` may be provided. At most one of `ids`, `pagination_token` may be provided.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<[**Vec<String>**](String.md)> | Comma-separated broadcast IDs to retrieve instead of listing broadcasts. |  |
**max_results** | Option<**i32**> |  |  |
**pagination_token** | Option<**String**> |  |  |
**broadcast_fields** | Option<[**Vec<models::BroadcastField>**](Models__BroadcastField.md)> | A comma separated list of Broadcast fields to display. |  |

### Return type

[**models::ListBroadcastsResponse**](ListBroadcastsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_scheduled_broadcasts

> models::ListScheduledBroadcastsResponse list_scheduled_broadcasts(max_results, oldest_start_time, newest_start_time, pagination_token)
List scheduled broadcasts

Returns scheduled broadcasts owned by the authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**max_results** | Option<**i32**> |  |  |
**oldest_start_time** | Option<**String**> |  |  |
**newest_start_time** | Option<**String**> |  |  |
**pagination_token** | Option<**String**> |  |  |

### Return type

[**models::ListScheduledBroadcastsResponse**](ListScheduledBroadcastsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mute_broadcast_chat_user

> models::MuteBroadcastChatUserResponse mute_broadcast_chat_user(id, mute_broadcast_chat_user_request)
Mute or time out a user in a broadcast chat

Prevents a user from posting further messages in a running broadcast chat. When `end_at_ms` is provided, the mute expires at that time. The authenticated user must own the broadcast.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**mute_broadcast_chat_user_request** | [**MuteBroadcastChatUserRequest**](MuteBroadcastChatUserRequest.md) |  | [required] |

### Return type

[**models::MuteBroadcastChatUserResponse**](MuteBroadcastChatUserResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_broadcast_chat

> models::SendBroadcastChatResponse send_broadcast_chat(id, send_broadcast_chat_request)
Send a chat message to a live broadcast

Posts a chat message to a running broadcast, attributed to the authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**send_broadcast_chat_request** | [**SendBroadcastChatRequest**](SendBroadcastChatRequest.md) |  | [required] |

### Return type

[**models::SendBroadcastChatResponse**](SendBroadcastChatResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unmute_broadcast_chat_user

> models::UnmuteBroadcastChatUserResponse unmute_broadcast_chat_user(id, user_id)
Unmute a user in a broadcast chat

Allows a muted user to post messages in a running broadcast chat again. The authenticated user must own the broadcast.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**user_id** | **String** | A 64-bit signed integer. | [required] |

### Return type

[**models::UnmuteBroadcastChatUserResponse**](UnmuteBroadcastChatUserResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_scheduled_broadcast

> models::UpdateScheduledBroadcastResponse update_scheduled_broadcast(id, update_scheduled_broadcast_request)
Update a scheduled broadcast

Fully replaces schedule fields for a broadcast. Path `:id` is the UBS broadcast id; the body must include `scheduled_broadcast_id` and re-send any fields that should be kept.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**update_scheduled_broadcast_request** | [**UpdateScheduledBroadcastRequest**](UpdateScheduledBroadcastRequest.md) |  | [required] |

### Return type

[**models::UpdateScheduledBroadcastResponse**](UpdateScheduledBroadcastResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

