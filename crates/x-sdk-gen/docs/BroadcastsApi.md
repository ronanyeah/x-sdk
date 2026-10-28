# \BroadcastsApi

All URIs are relative to *https://api.x.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_chat_moderator**](BroadcastsApi.md#add_chat_moderator) | **POST** /2/broadcasts/chat/moderators | Add a chat moderator
[**create_scheduled_broadcast**](BroadcastsApi.md#create_scheduled_broadcast) | **POST** /2/broadcasts/scheduled | Create a scheduled broadcast
[**create_source**](BroadcastsApi.md#create_source) | **POST** /2/sources | Create a stream source
[**create_user_source**](BroadcastsApi.md#create_user_source) | **POST** /2/users/{id}/sources | Create a stream source for a user
[**delete_broadcast_chat_message**](BroadcastsApi.md#delete_broadcast_chat_message) | **DELETE** /2/broadcasts/{id}/chat/{message_id} | Remove a chat message from a live broadcast
[**delete_scheduled_broadcast**](BroadcastsApi.md#delete_scheduled_broadcast) | **DELETE** /2/broadcasts/scheduled/{id} | Delete a scheduled broadcast
[**delete_source**](BroadcastsApi.md#delete_source) | **DELETE** /2/sources/{id} | Delete a stream source
[**delete_user_source**](BroadcastsApi.md#delete_user_source) | **DELETE** /2/users/{id}/sources/{source_id} | Delete a user's stream source
[**get_broadcast**](BroadcastsApi.md#get_broadcast) | **GET** /2/broadcasts/{id} | Get a broadcast
[**get_broadcast_chat**](BroadcastsApi.md#get_broadcast_chat) | **GET** /2/broadcasts/{id}/chat | Get broadcast chat history
[**get_chat_moderators**](BroadcastsApi.md#get_chat_moderators) | **GET** /2/broadcasts/chat/moderators | Get chat moderators
[**get_chat_muted_users**](BroadcastsApi.md#get_chat_muted_users) | **GET** /2/broadcasts/chat/mutes | Get muted chat users
[**get_scheduled_broadcast**](BroadcastsApi.md#get_scheduled_broadcast) | **GET** /2/broadcasts/scheduled/{id} | Get a scheduled broadcast
[**get_source**](BroadcastsApi.md#get_source) | **GET** /2/sources/{id} | Get a stream source
[**get_user_source**](BroadcastsApi.md#get_user_source) | **GET** /2/users/{id}/sources/{source_id} | Get a user's stream source
[**go_live_scheduled_broadcast**](BroadcastsApi.md#go_live_scheduled_broadcast) | **POST** /2/broadcasts/scheduled/{id}/live | Go live on a scheduled broadcast
[**list_broadcasts**](BroadcastsApi.md#list_broadcasts) | **GET** /2/broadcasts | List broadcasts
[**list_scheduled_broadcasts**](BroadcastsApi.md#list_scheduled_broadcasts) | **GET** /2/broadcasts/scheduled | List scheduled broadcasts
[**list_sources**](BroadcastsApi.md#list_sources) | **GET** /2/sources | List stream sources
[**list_user_sources**](BroadcastsApi.md#list_user_sources) | **GET** /2/users/{id}/sources | List a user's stream sources
[**mute_broadcast_chat_user**](BroadcastsApi.md#mute_broadcast_chat_user) | **POST** /2/broadcasts/{id}/chat/mutes | Mute or time out a user in a broadcast chat
[**mute_chat_user**](BroadcastsApi.md#mute_chat_user) | **POST** /2/broadcasts/chat/mutes | Mute or time out a user in chat
[**remove_chat_moderator**](BroadcastsApi.md#remove_chat_moderator) | **DELETE** /2/broadcasts/chat/moderators/{user_id} | Remove a chat moderator
[**send_broadcast_chat**](BroadcastsApi.md#send_broadcast_chat) | **POST** /2/broadcasts/{id}/chat | Send a chat message to a live broadcast
[**unmute_broadcast_chat_user**](BroadcastsApi.md#unmute_broadcast_chat_user) | **DELETE** /2/broadcasts/{id}/chat/mutes/{user_id} | Unmute a user in a broadcast chat
[**unmute_chat_user**](BroadcastsApi.md#unmute_chat_user) | **DELETE** /2/broadcasts/chat/mutes/{user_id} | Unmute a user in chat
[**update_scheduled_broadcast**](BroadcastsApi.md#update_scheduled_broadcast) | **PUT** /2/broadcasts/scheduled/{id} | Update a scheduled broadcast
[**update_source**](BroadcastsApi.md#update_source) | **PUT** /2/sources/{id} | Rename a stream source
[**update_user_source**](BroadcastsApi.md#update_user_source) | **PUT** /2/users/{id}/sources/{source_id} | Rename a user's stream source



## add_chat_moderator

> models::AddChatModeratorResponse add_chat_moderator(add_chat_moderator_request)
Add a chat moderator

Adds a moderator to the authenticated broadcaster's persistent list and returns the updated moderator user IDs. The list applies to all current and future broadcast chats.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**add_chat_moderator_request** | [**AddChatModeratorRequest**](AddChatModeratorRequest.md) |  | [required] |

### Return type

[**models::AddChatModeratorResponse**](AddChatModeratorResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_scheduled_broadcast

> models::CreateScheduledBroadcastResponse create_scheduled_broadcast(create_scheduled_broadcast_request)
Create a scheduled broadcast

Schedules a broadcast for the authenticated user. A `source_id` (ingest) is required at create time.

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


## create_source

> models::CreateSourceResponse create_source(create_source_request)
Create a stream source

Creates a persistent RTMP/RTMPS ingest point for the authenticated user. Push an encoder to `rtmps_url` with the source's stream key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_source_request** | [**CreateSourceRequest**](CreateSourceRequest.md) |  | [required] |

### Return type

[**models::CreateSourceResponse**](CreateSourceResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_source

> models::CreateUserSourceResponse create_user_source(id, create_user_source_request)
Create a stream source for a user

Creates a persistent RTMP/RTMPS ingest point for the authenticated user. `id` must be the authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**create_user_source_request** | [**CreateUserSourceRequest**](CreateUserSourceRequest.md) |  | [required] |

### Return type

[**models::CreateUserSourceResponse**](CreateUserSourceResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_broadcast_chat_message

> models::DeleteBroadcastChatMessageResponse delete_broadcast_chat_message(id, message_id)
Remove a chat message from a live broadcast

Removes a specific chat message from a running broadcast where the authenticated user is the host or a moderator.

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

Deletes a scheduled broadcast by its alphanumeric broadcast id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**roll_forward** | Option<**bool**> | Deprecated: recurring schedules are being retired and `true` will be rejected. |  |

### Return type

[**models::DeleteScheduledBroadcastResponse**](DeleteScheduledBroadcastResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_source

> models::DeleteSourceResponse delete_source(id)
Delete a stream source

Deletes one of the authenticated user's stream sources.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::DeleteSourceResponse**](DeleteSourceResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_source

> models::DeleteUserSourceResponse delete_user_source(id, source_id)
Delete a user's stream source

Deletes one of the authenticated user's stream sources. `id` must be the authenticated user. Success is an empty object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**source_id** | **String** |  | [required] |

### Return type

[**models::DeleteUserSourceResponse**](DeleteUserSourceResponse.md)

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


## get_chat_moderators

> models::GetChatModeratorsResponse get_chat_moderators(user_fields, expansions, post_fields)
Get chat moderators

Returns the authenticated broadcaster's persistent moderators as User resources. The list applies to all of their current and future broadcast chats.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |
**expansions** | Option<[**Vec<models::UserExpansion>**](Models__UserExpansion.md)> | A comma separated list of fields to expand. |  |
**post_fields** | Option<[**Vec<models::PostField>**](Models__PostField.md)> | A comma separated list of Post fields to display. |  |

### Return type

[**models::GetChatModeratorsResponse**](GetChatModeratorsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chat_muted_users

> models::GetChatMutedUsersResponse get_chat_muted_users(max_results, pagination_token, broadcast_chat_mute_fields, expansions, user_fields)
Get muted chat users

Returns the users muted in the authenticated broadcaster's chat, with the expiry time of timed mutes. Mutes apply to all of their broadcasts. Expired timed mutes are omitted. Use `expansions=user_id` to include the muted users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**max_results** | Option<**i32**> |  |  |
**pagination_token** | Option<**String**> |  |  |
**broadcast_chat_mute_fields** | Option<[**Vec<models::BroadcastChatMuteField>**](Models__BroadcastChatMuteField.md)> | A comma separated list of BroadcastChatMute fields to display. |  |
**expansions** | Option<[**Vec<models::BroadcastChatMuteExpansion>**](Models__BroadcastChatMuteExpansion.md)> | A comma separated list of fields to expand. |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |

### Return type

[**models::GetChatMutedUsersResponse**](GetChatMutedUsersResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scheduled_broadcast

> models::GetScheduledBroadcastResponse get_scheduled_broadcast(id)
Get a scheduled broadcast

Returns a single scheduled broadcast by its alphanumeric broadcast id.

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


## get_source

> models::GetSourceResponse get_source(id)
Get a stream source

Returns one of the authenticated user's stream sources.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::GetSourceResponse**](GetSourceResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_source

> models::GetUserSourceResponse get_user_source(id, source_id)
Get a user's stream source

Returns one of the authenticated user's stream sources. `id` must be the authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**source_id** | **String** |  | [required] |

### Return type

[**models::GetUserSourceResponse**](GetUserSourceResponse.md)

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


## list_sources

> models::ListSourcesResponse list_sources()
List stream sources

Returns the authenticated user's RTMP/RTMPS stream sources. A source's `id` is the `source_id` a scheduled broadcast binds to.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListSourcesResponse**](ListSourcesResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_user_sources

> models::ListUserSourcesResponse list_user_sources(id)
List a user's stream sources

Returns the authenticated user's RTMP/RTMPS stream sources (empty array if none). `id` must be the authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::ListUserSourcesResponse**](ListUserSourcesResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mute_broadcast_chat_user

> models::MuteBroadcastChatUserResponse mute_broadcast_chat_user(id, mute_broadcast_chat_user_request)
Mute or time out a user in a broadcast chat

Prevents a user from posting further messages in a running broadcast chat. When `end_at_ms` is provided, the mute expires at that time. The authenticated user must be the broadcast host or a moderator.

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


## mute_chat_user

> models::MuteChatUserResponse mute_chat_user(mute_chat_user_request)
Mute or time out a user in chat

Prevents a user from posting messages in all of a broadcaster's chats. Without `broadcast_id`, the user is muted in the authenticated user's own chats. Moderators pass `broadcast_id` to mute the user in that broadcast host's chats. When `end_at_ms` is provided, the mute expires at that time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**mute_chat_user_request** | [**MuteChatUserRequest**](MuteChatUserRequest.md) |  | [required] |

### Return type

[**models::MuteChatUserResponse**](MuteChatUserResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_chat_moderator

> models::RemoveChatModeratorResponse remove_chat_moderator(user_id)
Remove a chat moderator

Removes a moderator from the authenticated broadcaster's persistent list and returns the updated moderator user IDs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | A 64-bit signed integer. | [required] |

### Return type

[**models::RemoveChatModeratorResponse**](RemoveChatModeratorResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
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

Allows a muted user to post messages in a running broadcast chat again. The authenticated user must be the broadcast host or a moderator.

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


## unmute_chat_user

> models::UnmuteChatUserResponse unmute_chat_user(user_id, broadcast_id)
Unmute a user in chat

Allows a muted user to post messages in a broadcaster's chats again. Without `broadcast_id`, the user is unmuted in the authenticated user's own chats. Moderators pass `broadcast_id` to unmute the user in that broadcast host's chats.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | A 64-bit signed integer. | [required] |
**broadcast_id** | Option<**String**> | A broadcast whose host's chats the user is unmuted in. |  |

### Return type

[**models::UnmuteChatUserResponse**](UnmuteChatUserResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_scheduled_broadcast

> models::UpdateScheduledBroadcastResponse update_scheduled_broadcast(id, update_scheduled_broadcast_request)
Update a scheduled broadcast

Updates a scheduled broadcast. Path `:id` is the broadcast id; the body must include `scheduled_broadcast_id`. Re-send every field that should be kept.

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


## update_source

> models::UpdateSourceResponse update_source(id, update_source_request)
Rename a stream source

Renames one of the authenticated user's stream sources.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**update_source_request** | [**UpdateSourceRequest**](UpdateSourceRequest.md) |  | [required] |

### Return type

[**models::UpdateSourceResponse**](UpdateSourceResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user_source

> models::UpdateUserSourceResponse update_user_source(id, source_id, update_user_source_request)
Rename a user's stream source

Renames one of the authenticated user's stream sources. `id` must be the authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**source_id** | **String** |  | [required] |
**update_user_source_request** | [**UpdateUserSourceRequest**](UpdateUserSourceRequest.md) |  | [required] |

### Return type

[**models::UpdateUserSourceResponse**](UpdateUserSourceResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

