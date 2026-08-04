# \ChatApi

All URIs are relative to *https://api.x.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_chat_group_members**](ChatApi.md#add_chat_group_members) | **POST** /2/chat/conversations/{id}/members | Add members to a Chat group conversation
[**add_conversation_keys**](ChatApi.md#add_conversation_keys) | **POST** /2/chat/conversations/{id}/keys | Add Conversation Keys
[**add_user_public_key**](ChatApi.md#add_user_public_key) | **POST** /2/users/{id}/public_keys | Add public key
[**chat_media_download**](ChatApi.md#chat_media_download) | **GET** /2/chat/media/{id}/{media_hash_key} | Download Chat Media
[**chat_media_upload_append**](ChatApi.md#chat_media_upload_append) | **POST** /2/chat/media/upload/{id}/append | Append Chat Media Upload
[**chat_media_upload_finalize**](ChatApi.md#chat_media_upload_finalize) | **POST** /2/chat/media/upload/{id}/finalize | Finalize Chat Media Upload
[**chat_media_upload_initialize**](ChatApi.md#chat_media_upload_initialize) | **POST** /2/chat/media/upload/initialize | Initialize Chat Media Upload
[**create_chat_conversation**](ChatApi.md#create_chat_conversation) | **POST** /2/chat/conversations/group | Create Chat Group Conversation
[**delete_chat_messages**](ChatApi.md#delete_chat_messages) | **POST** /2/chat/conversations/{id}/messages/delete | Delete Chat messages
[**get_chat_conversation**](ChatApi.md#get_chat_conversation) | **GET** /2/chat/conversations/{id} | Get Chat Conversation
[**get_chat_conversation_events**](ChatApi.md#get_chat_conversation_events) | **GET** /2/chat/conversations/{id}/events | Get Chat Conversation Events
[**get_chat_conversations**](ChatApi.md#get_chat_conversations) | **GET** /2/chat/conversations | Get Chat Conversations
[**initialize_chat_group**](ChatApi.md#initialize_chat_group) | **POST** /2/chat/conversations/group/initialize | Initialize Chat Group
[**mark_chat_conversation_read**](ChatApi.md#mark_chat_conversation_read) | **POST** /2/chat/conversations/{id}/read | Mark Conversation as Read
[**send_chat_message**](ChatApi.md#send_chat_message) | **POST** /2/chat/conversations/{id}/messages | Send Chat Message
[**send_chat_typing_indicator**](ChatApi.md#send_chat_typing_indicator) | **POST** /2/chat/conversations/{id}/typing | Send Typing Indicator



## add_chat_group_members

> models::AddChatGroupMembersResponse add_chat_group_members(id, add_chat_group_members_request)
Add members to a Chat group conversation

Adds one or more members to an existing encrypted Chat group conversation, rotating the conversation key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**add_chat_group_members_request** | [**AddChatGroupMembersRequest**](AddChatGroupMembersRequest.md) |  | [required] |

### Return type

[**models::AddChatGroupMembersResponse**](AddChatGroupMembersResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_conversation_keys

> models::AddConversationKeysResponse add_conversation_keys(id, add_conversation_keys_request)
Add Conversation Keys

Adds (initializes or rotates) the encryption keys for a Chat conversation. Call this before sending messages in a new 1:1 conversation, and again with a newer key version to rotate the conversation key. For 1:1 conversations, provide the recipient's user ID as the conversation id; the server constructs the canonical conversation ID from the authenticated user and recipient. The request body must contain the conversation key version and participant keys (the conversation key encrypted for each participant using their public key).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**add_conversation_keys_request** | [**AddConversationKeysRequest**](AddConversationKeysRequest.md) |  | [required] |

### Return type

[**models::AddConversationKeysResponse**](AddConversationKeysResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_user_public_key

> models::AddUserPublicKeyResponse add_user_public_key(id, add_user_public_key_request)
Add public key

Registers a user's public key for X Chat encryption.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**add_user_public_key_request** | [**AddUserPublicKeyRequest**](AddUserPublicKeyRequest.md) |  | [required] |

### Return type

[**models::AddUserPublicKeyResponse**](AddUserPublicKeyResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_media_download

> std::path::PathBuf chat_media_download(id, media_hash_key)
Download Chat Media

Downloads encrypted media bytes from an XChat conversation. The response body contains raw binary bytes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**media_hash_key** | **String** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_media_upload_append

> models::ChatMediaUploadAppendResponse chat_media_upload_append(id, chat_media_upload_append_request)
Append Chat Media Upload

Appends media data to an XChat upload session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**chat_media_upload_append_request** | [**ChatMediaUploadAppendRequest**](ChatMediaUploadAppendRequest.md) |  | [required] |

### Return type

[**models::ChatMediaUploadAppendResponse**](ChatMediaUploadAppendResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_media_upload_finalize

> models::ChatMediaUploadFinalizeResponse chat_media_upload_finalize(id, chat_media_upload_finalize_request)
Finalize Chat Media Upload

Finalizes an XChat media upload session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**chat_media_upload_finalize_request** | [**ChatMediaUploadFinalizeRequest**](ChatMediaUploadFinalizeRequest.md) |  | [required] |

### Return type

[**models::ChatMediaUploadFinalizeResponse**](ChatMediaUploadFinalizeResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_media_upload_initialize

> models::ChatMediaUploadInitializeResponse chat_media_upload_initialize(chat_media_upload_initialize_request)
Initialize Chat Media Upload

Initializes an XChat media upload session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chat_media_upload_initialize_request** | [**ChatMediaUploadInitializeRequest**](ChatMediaUploadInitializeRequest.md) |  | [required] |

### Return type

[**models::ChatMediaUploadInitializeResponse**](ChatMediaUploadInitializeResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_chat_conversation

> models::CreateChatConversationResponse create_chat_conversation(create_chat_conversation_request)
Create Chat Group Conversation

Creates a new encrypted Chat group conversation on behalf of the authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_chat_conversation_request** | [**CreateChatConversationRequest**](CreateChatConversationRequest.md) |  | [required] |

### Return type

[**models::CreateChatConversationResponse**](CreateChatConversationResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_chat_messages

> models::DeleteChatMessagesResponse delete_chat_messages(id, delete_chat_messages_request)
Delete Chat messages

Deletes one or more messages from a Chat conversation. For 1:1 conversations, provide the recipient's user ID; the server constructs the canonical conversation ID from the authenticated user and recipient. Delete for all removes a message you sent (or, in groups you administer, any message) for every participant; delete for self removes any message only from your own view.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**delete_chat_messages_request** | [**DeleteChatMessagesRequest**](DeleteChatMessagesRequest.md) |  | [required] |

### Return type

[**models::DeleteChatMessagesResponse**](DeleteChatMessagesResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chat_conversation

> models::GetChatConversationResponse get_chat_conversation(id, chat_conversation_fields, expansions, user_fields)
Get Chat Conversation

Returns metadata for a Chat conversation including type, muted status, and group details. Use chat_conversation.fields to select which fields are returned. Use expansions to hydrate member, admin, or participant user objects. Use user.fields to control which profile fields are returned for expanded users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**chat_conversation_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of ChatConversation fields to display. |  |
**expansions** | Option<[**Vec<String>**](String.md)> | A comma separated list of fields to expand. |  |
**user_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of User fields to display. |  |

### Return type

[**models::GetChatConversationResponse**](GetChatConversationResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chat_conversation_events

> models::GetChatConversationEventsResponse get_chat_conversation_events(id, max_results, pagination_token, chat_message_event_fields)
Get Chat Conversation Events

Retrieves messages and key change events for a specific Chat conversation with pagination support. For 1:1 conversations, provide the recipient's user ID; the server constructs the canonical conversation ID from the authenticated user and recipient.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**max_results** | Option<**i32**> |  |  |[default to 10]
**pagination_token** | Option<**String**> |  |  |
**chat_message_event_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of ChatMessageEvent fields to display. |  |

### Return type

[**models::GetChatConversationEventsResponse**](GetChatConversationEventsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chat_conversations

> models::GetChatConversationsResponse get_chat_conversations(max_results, pagination_token, chat_conversation_fields, expansions, user_fields)
Get Chat Conversations

Retrieves a list of Chat conversations for the authenticated user's inbox.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**max_results** | Option<**i32**> |  |  |[default to 10]
**pagination_token** | Option<**String**> |  |  |
**chat_conversation_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of ChatConversation fields to display. |  |
**expansions** | Option<[**Vec<String>**](String.md)> | A comma separated list of fields to expand. |  |
**user_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of User fields to display. |  |

### Return type

[**models::GetChatConversationsResponse**](GetChatConversationsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## initialize_chat_group

> models::InitializeChatGroupResponse initialize_chat_group()
Initialize Chat Group

Initializes a new Chat group conversation and returns a unique conversation ID. Use the returned conversation_id in a subsequent POST /chat/conversations/group call to fully create and configure the group.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::InitializeChatGroupResponse**](InitializeChatGroupResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mark_chat_conversation_read

> models::MarkChatConversationReadResponse mark_chat_conversation_read(id, mark_chat_conversation_read_request)
Mark Conversation as Read

Marks a specific Chat conversation as read on behalf of the authenticated user. For 1:1 conversations, provide the recipient's user ID; the server constructs the canonical conversation ID from the authenticated user and recipient.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**mark_chat_conversation_read_request** | [**MarkChatConversationReadRequest**](MarkChatConversationReadRequest.md) |  | [required] |

### Return type

[**models::MarkChatConversationReadResponse**](MarkChatConversationReadResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_chat_message

> models::SendChatMessageResponse send_chat_message(id, send_chat_message_request)
Send Chat Message

Sends an encrypted message to a specific Chat conversation. For 1:1 conversations, provide the recipient's user ID; the server constructs the canonical conversation ID from the authenticated user and recipient.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**send_chat_message_request** | [**SendChatMessageRequest**](SendChatMessageRequest.md) |  | [required] |

### Return type

[**models::SendChatMessageResponse**](SendChatMessageResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_chat_typing_indicator

> models::SendChatTypingIndicatorResponse send_chat_typing_indicator(id)
Send Typing Indicator

Sends a typing indicator to a specific Chat conversation on behalf of the authenticated user. For 1:1 conversations, provide the recipient's user ID; the server constructs the canonical conversation ID from the authenticated user and recipient.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::SendChatTypingIndicatorResponse**](SendChatTypingIndicatorResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

