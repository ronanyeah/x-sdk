# \ChatApi

All URIs are relative to *https://api.x.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**accept_chat_message_request**](ChatApi.md#accept_chat_message_request) | **POST** /2/chat/message_requests/{id}/accept | Accept a Chat message request
[**add_chat_group_admins**](ChatApi.md#add_chat_group_admins) | **POST** /2/chat/conversations/{id}/admins | Add admins to a Chat group conversation
[**add_chat_group_members**](ChatApi.md#add_chat_group_members) | **POST** /2/chat/conversations/{id}/members | Add members to a Chat group conversation
[**add_conversation_keys**](ChatApi.md#add_conversation_keys) | **POST** /2/chat/conversations/{id}/keys | Add Conversation Keys
[**add_user_public_key**](ChatApi.md#add_user_public_key) | **POST** /2/users/{id}/public_keys | Add public key
[**approve_chat_group_admin_request**](ChatApi.md#approve_chat_group_admin_request) | **POST** /2/chat/conversations/{id}/admin_requests/{user_id}/approve | Approve an admin request in a Chat group conversation
[**archive_chat_conversation**](ChatApi.md#archive_chat_conversation) | **POST** /2/chat/conversations/{id}/archive | Archive a Chat conversation
[**chat_media_download**](ChatApi.md#chat_media_download) | **GET** /2/chat/media/{id}/{media_hash_key} | Download Chat Media
[**chat_media_upload_append**](ChatApi.md#chat_media_upload_append) | **POST** /2/chat/media/upload/{id}/append | Append Chat Media Upload
[**chat_media_upload_finalize**](ChatApi.md#chat_media_upload_finalize) | **POST** /2/chat/media/upload/{id}/finalize | Finalize Chat Media Upload
[**chat_media_upload_initialize**](ChatApi.md#chat_media_upload_initialize) | **POST** /2/chat/media/upload/initialize | Initialize Chat Media Upload
[**create_chat_conversation**](ChatApi.md#create_chat_conversation) | **POST** /2/chat/conversations/group | Create Chat Group Conversation
[**create_chat_group_admin_request**](ChatApi.md#create_chat_group_admin_request) | **POST** /2/chat/conversations/{id}/admin_requests | Request admin permissions in a Chat group conversation
[**create_chat_group_join_request**](ChatApi.md#create_chat_group_join_request) | **POST** /2/chat/conversations/{id}/join_requests | Request to join a Chat group
[**delete_chat_conversation**](ChatApi.md#delete_chat_conversation) | **DELETE** /2/chat/conversations/{id} | Delete a Chat conversation
[**delete_chat_conversation_message_ttl**](ChatApi.md#delete_chat_conversation_message_ttl) | **DELETE** /2/chat/conversations/{id}/message_ttl | Turn off disappearing messages in a Chat conversation
[**delete_chat_messages**](ChatApi.md#delete_chat_messages) | **POST** /2/chat/conversations/{id}/messages/delete | Delete Chat messages
[**disable_chat_group_invite**](ChatApi.md#disable_chat_group_invite) | **DELETE** /2/chat/conversations/{id}/invite | Disable a Chat group invite link
[**enable_chat_group_invite**](ChatApi.md#enable_chat_group_invite) | **POST** /2/chat/conversations/{id}/invite | Enable a Chat group invite link
[**get_chat_conversation**](ChatApi.md#get_chat_conversation) | **GET** /2/chat/conversations/{id} | Get Chat Conversation; unread state via chat_conversation.fields=unread,last_read_event_id,newest_event_id
[**get_chat_conversation_events**](ChatApi.md#get_chat_conversation_events) | **GET** /2/chat/conversations/{id}/events | Get Chat Conversation Events, newest first; unread ones have an id above last_read_event_id
[**get_chat_conversation_pinned_messages**](ChatApi.md#get_chat_conversation_pinned_messages) | **GET** /2/chat/conversations/{id}/pinned_messages | Get Chat Conversation Pinned Messages
[**get_chat_conversations**](ChatApi.md#get_chat_conversations) | **GET** /2/chat/conversations | Get Chat Conversations; unread state via chat_conversation.fields=unread,last_read_event_id,newest_event_id
[**get_chat_group_admin_requests**](ChatApi.md#get_chat_group_admin_requests) | **GET** /2/chat/conversations/{id}/admin_requests | Get pending admin requests for a Chat group conversation
[**get_chat_group_invite**](ChatApi.md#get_chat_group_invite) | **GET** /2/chat/conversations/{id}/invites/{token} | Get a Chat group invite link
[**get_chat_message_request**](ChatApi.md#get_chat_message_request) | **GET** /2/chat/message_requests/{id} | Get Chat Message Request
[**get_chat_message_requests**](ChatApi.md#get_chat_message_requests) | **GET** /2/chat/message_requests | Get Chat Message Requests
[**get_chat_settings**](ChatApi.md#get_chat_settings) | **GET** /2/chat/settings | Get Chat settings
[**initialize_chat_group**](ChatApi.md#initialize_chat_group) | **POST** /2/chat/conversations/group/initialize | Initialize Chat Group
[**mark_chat_conversation_read**](ChatApi.md#mark_chat_conversation_read) | **POST** /2/chat/conversations/{id}/read | Mark Conversation as Read
[**mark_chat_conversation_unread**](ChatApi.md#mark_chat_conversation_unread) | **POST** /2/chat/conversations/{id}/unread | Mark Conversation as Unread
[**mute_chat_conversation**](ChatApi.md#mute_chat_conversation) | **POST** /2/chat/conversations/{id}/mute | Mute a Chat conversation
[**reject_chat_group_admin_request**](ChatApi.md#reject_chat_group_admin_request) | **POST** /2/chat/conversations/{id}/admin_requests/{user_id}/reject | Reject an admin request in a Chat group conversation
[**reject_chat_group_join_requests**](ChatApi.md#reject_chat_group_join_requests) | **POST** /2/chat/conversations/{id}/join_requests/reject | Reject join requests for a Chat group conversation
[**remove_chat_group_admins**](ChatApi.md#remove_chat_group_admins) | **POST** /2/chat/conversations/{id}/admins/remove | Remove admins from a Chat group conversation
[**remove_chat_group_members**](ChatApi.md#remove_chat_group_members) | **POST** /2/chat/conversations/{id}/members/remove | Remove members from a Chat group conversation
[**send_chat_message**](ChatApi.md#send_chat_message) | **POST** /2/chat/conversations/{id}/messages | Send Chat Message
[**send_chat_typing_indicator**](ChatApi.md#send_chat_typing_indicator) | **POST** /2/chat/conversations/{id}/typing | Send Typing Indicator
[**unarchive_chat_conversation**](ChatApi.md#unarchive_chat_conversation) | **DELETE** /2/chat/conversations/{id}/archive | Unarchive a Chat conversation
[**unmute_chat_conversation**](ChatApi.md#unmute_chat_conversation) | **DELETE** /2/chat/conversations/{id}/mute | Unmute a Chat conversation
[**update_chat_conversation_message_ttl**](ChatApi.md#update_chat_conversation_message_ttl) | **PUT** /2/chat/conversations/{id}/message_ttl | Turn on disappearing messages in a Chat conversation
[**update_chat_conversation_screen_capture**](ChatApi.md#update_chat_conversation_screen_capture) | **PUT** /2/chat/conversations/{id}/screen_capture | Update screen capture settings for a Chat conversation
[**update_chat_group**](ChatApi.md#update_chat_group) | **PUT** /2/chat/conversations/{id} | Update a Chat group conversation
[**update_chat_group_admin_settings**](ChatApi.md#update_chat_group_admin_settings) | **PUT** /2/chat/conversations/{id}/admin_settings | Update the admin-only settings of a Chat group conversation
[**update_chat_settings**](ChatApi.md#update_chat_settings) | **PUT** /2/chat/settings | Update Chat settings



## accept_chat_message_request

> models::AcceptChatMessageRequestResponse accept_chat_message_request(id)
Accept a Chat message request

Accepts the other user's pending message request in a 1:1 Chat conversation, which moves to the authenticated user's inbox. Without a pending request, including after a successful accept, the response is 404. Replying to a request does not accept it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The other user's ID, or both user IDs joined by a hyphen. | [required] |

### Return type

[**models::AcceptChatMessageRequestResponse**](AcceptChatMessageRequestResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_chat_group_admins

> models::AddChatGroupAdminsResponse add_chat_group_admins(id, add_chat_group_admins_request)
Add admins to a Chat group conversation

Grants admin rights to one or more members of an existing encrypted Chat group conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the group conversation, starting with `g`. | [required] |
**add_chat_group_admins_request** | [**AddChatGroupAdminsRequest**](AddChatGroupAdminsRequest.md) |  | [required] |

### Return type

[**models::AddChatGroupAdminsResponse**](AddChatGroupAdminsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_chat_group_members

> models::AddChatGroupMembersResponse add_chat_group_members(id, add_chat_group_members_request)
Add members to a Chat group conversation

Adds one or more members to an existing encrypted Chat group conversation, rotating the conversation key. Adding a user who has a pending join request approves that request; approval needs the same key-rotation body as adding any member.

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


## approve_chat_group_admin_request

> models::ApproveChatGroupAdminRequestResponse approve_chat_group_admin_request(id, user_id, approve_chat_group_admin_request_request)
Approve an admin request in a Chat group conversation

Makes the user who requested admin permissions an admin of an encrypted Chat group conversation. Only a group admin can approve.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the group conversation, starting with `g`. | [required] |
**user_id** | **String** | The ID of the user who requested admin permissions. | [required] |
**approve_chat_group_admin_request_request** | [**ApproveChatGroupAdminRequestRequest**](ApproveChatGroupAdminRequestRequest.md) |  | [required] |

### Return type

[**models::ApproveChatGroupAdminRequestResponse**](ApproveChatGroupAdminRequestResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## archive_chat_conversation

> models::ArchiveChatConversationResponse archive_chat_conversation(id, archive_chat_conversation_request)
Archive a Chat conversation

Archives a Chat conversation for the authenticated user only. Archiving an archived conversation changes nothing. Works for group and 1:1 conversations. For 1:1 conversations, provide the recipient's user ID; the server constructs the canonical conversation ID from the authenticated user and recipient.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the conversation: a group ID starting with `g`, the other user's ID for a 1:1 conversation, or both user IDs of a 1:1 conversation joined by a hyphen. | [required] |
**archive_chat_conversation_request** | [**ArchiveChatConversationRequest**](ArchiveChatConversationRequest.md) |  | [required] |

### Return type

[**models::ArchiveChatConversationResponse**](ArchiveChatConversationResponse.md)

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


## create_chat_group_admin_request

> models::CreateChatGroupAdminRequestResponse create_chat_group_admin_request(id, create_chat_group_admin_request_request)
Request admin permissions in a Chat group conversation

Asks the admins of an encrypted Chat group conversation to make the authenticated user an admin. An admin approves or rejects the request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the group conversation, starting with `g`. | [required] |
**create_chat_group_admin_request_request** | [**CreateChatGroupAdminRequestRequest**](CreateChatGroupAdminRequestRequest.md) |  | [required] |

### Return type

[**models::CreateChatGroupAdminRequestResponse**](CreateChatGroupAdminRequestResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_chat_group_join_request

> models::CreateChatGroupJoinRequestResponse create_chat_group_join_request(id, create_chat_group_join_request_request)
Request to join a Chat group

Asks to join an encrypted Chat group conversation through its invite link. The request stays pending until a group admin adds the user to the group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the group conversation, starting with `g`. | [required] |
**create_chat_group_join_request_request** | [**CreateChatGroupJoinRequestRequest**](CreateChatGroupJoinRequestRequest.md) |  | [required] |

### Return type

[**models::CreateChatGroupJoinRequestResponse**](CreateChatGroupJoinRequestResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_chat_conversation

> models::DeleteChatConversationResponse delete_chat_conversation(id, delete_chat_conversation_request)
Delete a Chat conversation

Deletes a Chat conversation for the authenticated user only; other participants keep it. Works for group and 1:1 conversations. For 1:1 conversations, provide the recipient's user ID; the server constructs the canonical conversation ID from the authenticated user and recipient. Deleting a pending message request's conversation deletes the request, like Delete in the X app.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the conversation: a group ID starting with `g`, the other user's ID for a 1:1 conversation, or both user IDs of a 1:1 conversation joined by a hyphen. | [required] |
**delete_chat_conversation_request** | [**DeleteChatConversationRequest**](DeleteChatConversationRequest.md) |  | [required] |

### Return type

[**models::DeleteChatConversationResponse**](DeleteChatConversationResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_chat_conversation_message_ttl

> models::DeleteChatConversationMessageTtlResponse delete_chat_conversation_message_ttl(id, delete_chat_conversation_message_ttl_request)
Turn off disappearing messages in a Chat conversation

Stops new messages in a 1:1 or group Chat conversation from disappearing, for every participant. For 1:1 conversations, provide the recipient's user ID; the server constructs the canonical conversation ID from the authenticated user and recipient.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the conversation: a group ID starting with `g`, the other user's ID for a 1:1 conversation, or both user IDs of a 1:1 conversation joined by a hyphen. | [required] |
**delete_chat_conversation_message_ttl_request** | [**DeleteChatConversationMessageTtlRequest**](DeleteChatConversationMessageTtlRequest.md) |  | [required] |

### Return type

[**models::DeleteChatConversationMessageTtlResponse**](DeleteChatConversationMessageTtlResponse.md)

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


## disable_chat_group_invite

> models::DisableChatGroupInviteResponse disable_chat_group_invite(id, disable_chat_group_invite_request)
Disable a Chat group invite link

Turns off the invite link for an encrypted Chat group conversation. The existing link stops working.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the group conversation, starting with `g`. | [required] |
**disable_chat_group_invite_request** | [**DisableChatGroupInviteRequest**](DisableChatGroupInviteRequest.md) |  | [required] |

### Return type

[**models::DisableChatGroupInviteResponse**](DisableChatGroupInviteResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_chat_group_invite

> models::EnableChatGroupInviteResponse enable_chat_group_invite(id, enable_chat_group_invite_request)
Enable a Chat group invite link

Turns on the invite link for an encrypted Chat group conversation and returns the link details. The client builds the link and sends it as invite_url, in the form https://x.com/i/chat/group_join/<conversation_id>/<token>, where the token is exactly 10 letters, digits, '-' or '_'. invite_url is required when action_signatures is sent. To make the link expire, send validity_duration_msec or validity_expiry_timestamp_msec. With action_signatures, send validity_expiry_timestamp_msec, because the signed payload carries the expiry. The group name and avatar sent here are stored unencrypted and shown to anyone who opens the link.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the group conversation, starting with `g`. | [required] |
**enable_chat_group_invite_request** | [**EnableChatGroupInviteRequest**](EnableChatGroupInviteRequest.md) |  | [required] |

### Return type

[**models::EnableChatGroupInviteResponse**](EnableChatGroupInviteResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chat_conversation

> models::GetChatConversationResponse get_chat_conversation(id, chat_conversation_fields, expansions, user_fields)
Get Chat Conversation; unread state via chat_conversation.fields=unread,last_read_event_id,newest_event_id

Returns metadata for a Chat conversation including type, muted status, and group details. Use chat_conversation.fields to select which fields are returned. Use expansions to hydrate member, admin, or participant user objects. Use user.fields to control which profile fields are returned for expanded users. The unread, last_read_event_id and newest_event_id fields give your read state, computed from your latest read receipt and the conversation's latest notifying message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**chat_conversation_fields** | Option<[**Vec<models::ChatConversationField>**](Models__ChatConversationField.md)> | A comma separated list of ChatConversation fields to display. |  |
**expansions** | Option<[**Vec<models::ChatConversationExpansion>**](Models__ChatConversationExpansion.md)> | A comma separated list of fields to expand. |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |

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
Get Chat Conversation Events, newest first; unread ones have an id above last_read_event_id

Retrieves the events of a specific Chat conversation, newest first, with pagination support: messages, key changes, group changes, and read receipts. Events with an id above the conversation's last_read_event_id come after the point you read up to; to collect them, page until an event's id is at or below it. Only `id` and `encoded_event` are always present. Events are either end-to-end encrypted or unencrypted, such as messages sent before the conversation was encrypted. Encrypted events carry `conversation_key_version`, and usually `conversation_token` and `message_event_signature`; treat an encrypted event without `message_event_signature` as unverified. An event without `conversation_key_version` is unencrypted. Decode each `encoded_event` client-side, and decrypt it with the conversation key named by `conversation_key_version` when that is present. A page holds up to `max_results` events and can hold fewer while `has_more` is true. For 1:1 conversations, provide the recipient's user ID; the server constructs the canonical conversation ID from the authenticated user and recipient.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**max_results** | Option<**i32**> | The maximum number of events to return. |  |[default to 10]
**pagination_token** | Option<**String**> | The next_token from a previous response, to fetch the next page. |  |
**chat_message_event_fields** | Option<[**Vec<models::ChatMessageEventField>**](Models__ChatMessageEventField.md)> | A comma separated list of ChatMessageEvent fields to display. |  |

### Return type

[**models::GetChatConversationEventsResponse**](GetChatConversationEventsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chat_conversation_pinned_messages

> models::GetChatConversationPinnedMessagesResponse get_chat_conversation_pinned_messages(id, pagination_token, chat_pinned_message_fields)
Get Chat Conversation Pinned Messages

Retrieves the pinned message events of a Chat conversation with pagination support. Each event is a Base64-encoded Thrift pin event whose contents are end-to-end encrypted; decode and decrypt it client-side. For 1:1 conversations, provide the recipient's user ID; the server constructs the canonical conversation ID from the authenticated user and recipient.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the conversation: a group ID starting with `g`, the other user's ID for a 1:1 conversation, or both user IDs of a 1:1 conversation joined by a hyphen. | [required] |
**pagination_token** | Option<**String**> | The next_token from a previous response, to fetch the next page. |  |
**chat_pinned_message_fields** | Option<[**Vec<models::ChatPinnedMessageField>**](Models__ChatPinnedMessageField.md)> | A comma separated list of ChatPinnedMessage fields to display. |  |

### Return type

[**models::GetChatConversationPinnedMessagesResponse**](GetChatConversationPinnedMessagesResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chat_conversations

> models::GetChatConversationsResponse get_chat_conversations(max_results, pagination_token, unread_only, chat_conversation_fields, expansions, user_fields)
Get Chat Conversations; unread state via chat_conversation.fields=unread,last_read_event_id,newest_event_id

Retrieves a list of Chat conversations for the authenticated user's inbox. The unread, last_read_event_id and newest_event_id fields give your read state, computed from your latest read receipt and each conversation's latest notifying message. With unread_only=true, only conversations that are unread for you are returned; a page can then hold fewer than max_results, so keep paging while has_more is true.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**max_results** | Option<**i32**> |  |  |[default to 200]
**pagination_token** | Option<**String**> |  |  |
**unread_only** | Option<**bool**> | Return only unread conversations. A page can be short; keep paging while has_more. |  |[default to false]
**chat_conversation_fields** | Option<[**Vec<models::ChatConversationField>**](Models__ChatConversationField.md)> | A comma separated list of ChatConversation fields to display. |  |
**expansions** | Option<[**Vec<models::ChatConversationExpansion>**](Models__ChatConversationExpansion.md)> | A comma separated list of fields to expand. |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |

### Return type

[**models::GetChatConversationsResponse**](GetChatConversationsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chat_group_admin_requests

> models::GetChatGroupAdminRequestsResponse get_chat_group_admin_requests(id, pagination_token, user_fields, expansions, post_fields)
Get pending admin requests for a Chat group conversation

Returns the users who have asked to become admins of an encrypted Chat group conversation. Only a group admin can list admin requests. Use user.fields to select which profile fields are returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the group conversation, starting with `g`. | [required] |
**pagination_token** | Option<**String**> | The next_token from a previous response, to fetch the next page. |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |
**expansions** | Option<[**Vec<models::UserExpansion>**](Models__UserExpansion.md)> | A comma separated list of fields to expand. |  |
**post_fields** | Option<[**Vec<models::PostField>**](Models__PostField.md)> | A comma separated list of Post fields to display. |  |

### Return type

[**models::GetChatGroupAdminRequestsResponse**](GetChatGroupAdminRequestsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chat_group_invite

> models::GetChatGroupInviteResponse get_chat_group_invite(id, token, chat_group_invite_fields)
Get a Chat group invite link

Returns the preview of an encrypted Chat group conversation from its invite link: group name, avatar, member count, and the authenticated user's join state. The conversation ID and the token both come from the invite link.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the group conversation, starting with `g`. | [required] |
**token** | **String** | The 10-character invite token from the group's invite link. | [required] |
**chat_group_invite_fields** | Option<[**Vec<models::ChatGroupInviteField>**](Models__ChatGroupInviteField.md)> | A comma separated list of ChatGroupInvite fields to display. |  |

### Return type

[**models::GetChatGroupInviteResponse**](GetChatGroupInviteResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chat_message_request

> models::GetChatMessageRequestResponse get_chat_message_request(id, chat_message_request_detail_fields)
Get Chat Message Request

Retrieves the message request state of a 1:1 Chat conversation and its newest message events. Like opening the request in an X app, reading it marks the returned messages as delivered and updates the unread badge.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The other user's ID, or both user IDs joined by a hyphen. | [required] |
**chat_message_request_detail_fields** | Option<[**Vec<models::ChatMessageRequestDetailField>**](Models__ChatMessageRequestDetailField.md)> | A comma separated list of ChatMessageRequestDetail fields to display. |  |

### Return type

[**models::GetChatMessageRequestResponse**](GetChatMessageRequestResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chat_message_requests

> models::GetChatMessageRequestsResponse get_chat_message_requests(inbox_type, max_results, pagination_token, chat_message_request_fields)
Get Chat Message Requests

Retrieves the authenticated user's pending Chat message requests from one inbox. Message contents are end-to-end encrypted unless the conversation is unencrypted. Accepting or deleting requests between pages can skip or repeat requests, and a page can be empty while `next_token` is set.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inbox_type** | Option<**String**> | Which message request inbox to list: `you_may_know` (the apps' Priority tab) or `low_quality` (the apps' Hidden tab). |  |[default to you_may_know]
**max_results** | Option<**i32**> | The maximum number of message requests to return. |  |[default to 10]
**pagination_token** | Option<**String**> | The next_token from a previous response, to fetch the next page. |  |
**chat_message_request_fields** | Option<[**Vec<models::ChatMessageRequestField>**](Models__ChatMessageRequestField.md)> | A comma separated list of ChatMessageRequest fields to display. |  |

### Return type

[**models::GetChatMessageRequestsResponse**](GetChatMessageRequestsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chat_settings

> models::GetChatSettingsResponse get_chat_settings(chat_settings_fields)
Get Chat settings

Returns the authenticated user's Chat privacy settings: who can message them, read receipts, the message quality filter, and audio/video call permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chat_settings_fields** | Option<[**Vec<models::ChatSettingsField>**](Models__ChatSettingsField.md)> | A comma separated list of ChatSettings fields to display. |  |

### Return type

[**models::GetChatSettingsResponse**](GetChatSettingsResponse.md)

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

> models::MarkChatConversationReadResponse mark_chat_conversation_read(id)
Mark Conversation as Read

Marks a specific Chat conversation as read on behalf of the authenticated user. For 1:1 conversations, provide the recipient's user ID; the server constructs the canonical conversation ID from the authenticated user and recipient.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::MarkChatConversationReadResponse**](MarkChatConversationReadResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mark_chat_conversation_unread

> models::MarkChatConversationUnreadResponse mark_chat_conversation_unread(id)
Mark Conversation as Unread

Marks a specific Chat conversation as unread on behalf of the authenticated user. For 1:1 conversations, provide the recipient's user ID; the server constructs the canonical conversation ID from the authenticated user and recipient.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::MarkChatConversationUnreadResponse**](MarkChatConversationUnreadResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mute_chat_conversation

> models::MuteChatConversationResponse mute_chat_conversation(id, mute_chat_conversation_request)
Mute a Chat conversation

Mutes notifications for a Chat conversation for the authenticated user only. Works for group and 1:1 conversations. For 1:1 conversations, provide the recipient's user ID; the server constructs the canonical conversation ID from the authenticated user and recipient.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the conversation: a group ID starting with `g`, the other user's ID for a 1:1 conversation, or both user IDs of a 1:1 conversation joined by a hyphen. | [required] |
**mute_chat_conversation_request** | [**MuteChatConversationRequest**](MuteChatConversationRequest.md) |  | [required] |

### Return type

[**models::MuteChatConversationResponse**](MuteChatConversationResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reject_chat_group_admin_request

> models::RejectChatGroupAdminRequestResponse reject_chat_group_admin_request(id, user_id, reject_chat_group_admin_request_request)
Reject an admin request in a Chat group conversation

Declines a user's pending request for admin permissions in an encrypted Chat group conversation. Only a group admin can reject.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the group conversation, starting with `g`. | [required] |
**user_id** | **String** | The ID of the user who requested admin permissions. | [required] |
**reject_chat_group_admin_request_request** | [**RejectChatGroupAdminRequestRequest**](RejectChatGroupAdminRequestRequest.md) |  | [required] |

### Return type

[**models::RejectChatGroupAdminRequestResponse**](RejectChatGroupAdminRequestResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reject_chat_group_join_requests

> models::RejectChatGroupJoinRequestsResponse reject_chat_group_join_requests(id, reject_chat_group_join_requests_request)
Reject join requests for a Chat group conversation

Rejects pending requests to join an encrypted Chat group conversation. Only group admins can reject requests. To approve a request instead, add the requesting users with addChatGroupMembers; adding a user clears their pending request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the group conversation, starting with `g`. | [required] |
**reject_chat_group_join_requests_request** | [**RejectChatGroupJoinRequestsRequest**](RejectChatGroupJoinRequestsRequest.md) |  | [required] |

### Return type

[**models::RejectChatGroupJoinRequestsResponse**](RejectChatGroupJoinRequestsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_chat_group_admins

> models::RemoveChatGroupAdminsResponse remove_chat_group_admins(id, remove_chat_group_admins_request)
Remove admins from a Chat group conversation

Revokes admin rights from one or more admins of an existing encrypted Chat group conversation. They stay members of the group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the group conversation, starting with `g`. | [required] |
**remove_chat_group_admins_request** | [**RemoveChatGroupAdminsRequest**](RemoveChatGroupAdminsRequest.md) |  | [required] |

### Return type

[**models::RemoveChatGroupAdminsResponse**](RemoveChatGroupAdminsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_chat_group_members

> models::RemoveChatGroupMembersResponse remove_chat_group_members(id, remove_chat_group_members_request)
Remove members from a Chat group conversation

Removes one or more members from an existing encrypted Chat group conversation. To leave a group, pass your own user ID. Leaving does not delete the conversation for the caller.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**remove_chat_group_members_request** | [**RemoveChatGroupMembersRequest**](RemoveChatGroupMembersRequest.md) |  | [required] |

### Return type

[**models::RemoveChatGroupMembersResponse**](RemoveChatGroupMembersResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_chat_message

> models::SendChatMessageResponse send_chat_message(id, send_chat_message_request)
Send Chat Message

Sends an encrypted message to a specific Chat conversation. For 1:1 conversations, provide the recipient's user ID; the server constructs the canonical conversation ID from the authenticated user and recipient. A recipient who cannot be messaged (closed inbox, system/notification account, or a 1:1 block) returns HTTP 403 with type https://api.x.com/2/problems/recipient-not-messageable.

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


## unarchive_chat_conversation

> models::UnarchiveChatConversationResponse unarchive_chat_conversation(id, unarchive_chat_conversation_request)
Unarchive a Chat conversation

Unarchives a Chat conversation for the authenticated user only. Unarchiving a conversation that is not archived changes nothing. Works for group and 1:1 conversations. For 1:1 conversations, provide the recipient's user ID; the server constructs the canonical conversation ID from the authenticated user and recipient.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the conversation: a group ID starting with `g`, the other user's ID for a 1:1 conversation, or both user IDs of a 1:1 conversation joined by a hyphen. | [required] |
**unarchive_chat_conversation_request** | [**UnarchiveChatConversationRequest**](UnarchiveChatConversationRequest.md) |  | [required] |

### Return type

[**models::UnarchiveChatConversationResponse**](UnarchiveChatConversationResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unmute_chat_conversation

> models::UnmuteChatConversationResponse unmute_chat_conversation(id, unmute_chat_conversation_request)
Unmute a Chat conversation

Unmutes notifications for a Chat conversation for the authenticated user only. Works for group and 1:1 conversations. For 1:1 conversations, provide the recipient's user ID; the server constructs the canonical conversation ID from the authenticated user and recipient.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the conversation: a group ID starting with `g`, the other user's ID for a 1:1 conversation, or both user IDs of a 1:1 conversation joined by a hyphen. | [required] |
**unmute_chat_conversation_request** | [**UnmuteChatConversationRequest**](UnmuteChatConversationRequest.md) |  | [required] |

### Return type

[**models::UnmuteChatConversationResponse**](UnmuteChatConversationResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_chat_conversation_message_ttl

> models::UpdateChatConversationMessageTtlResponse update_chat_conversation_message_ttl(id, update_chat_conversation_message_ttl_request)
Turn on disappearing messages in a Chat conversation

Sets how long new messages in a 1:1 or group Chat conversation last before they disappear, for every participant. For 1:1 conversations, provide the recipient's user ID; the server constructs the canonical conversation ID from the authenticated user and recipient.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the conversation: a group ID starting with `g`, the other user's ID for a 1:1 conversation, or both user IDs of a 1:1 conversation joined by a hyphen. | [required] |
**update_chat_conversation_message_ttl_request** | [**UpdateChatConversationMessageTtlRequest**](UpdateChatConversationMessageTtlRequest.md) |  | [required] |

### Return type

[**models::UpdateChatConversationMessageTtlResponse**](UpdateChatConversationMessageTtlResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_chat_conversation_screen_capture

> models::UpdateChatConversationScreenCaptureResponse update_chat_conversation_screen_capture(id, update_chat_conversation_screen_capture_request)
Update screen capture settings for a Chat conversation

Turns screenshot blocking or screenshot detection on or off in a Chat conversation, for every participant. In a group conversation, only admins can change these settings. Each request changes one setting: send either blocking or detection. For 1:1 conversations, provide the recipient's user ID; the server constructs the canonical conversation ID from the authenticated user and recipient.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the conversation: a group ID starting with `g`, the other user's ID for a 1:1 conversation, or both user IDs of a 1:1 conversation joined by a hyphen. | [required] |
**update_chat_conversation_screen_capture_request** | Option<[**UpdateChatConversationScreenCaptureRequest**](UpdateChatConversationScreenCaptureRequest.md)> |  | [required] |

### Return type

[**models::UpdateChatConversationScreenCaptureResponse**](UpdateChatConversationScreenCaptureResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_chat_group

> models::UpdateChatGroupResponse update_chat_group(id, update_chat_group_request)
Update a Chat group conversation

Renames an encrypted Chat group conversation, changes its avatar, or sets its description. The client encrypts each value with the conversation key and lists the values it changed in modified_fields.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the group conversation, starting with `g`. | [required] |
**update_chat_group_request** | [**UpdateChatGroupRequest**](UpdateChatGroupRequest.md) |  | [required] |

### Return type

[**models::UpdateChatGroupResponse**](UpdateChatGroupResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_chat_group_admin_settings

> models::UpdateChatGroupAdminSettingsResponse update_chat_group_admin_settings(id, update_chat_group_admin_settings_request)
Update the admin-only settings of a Chat group conversation

Sets which actions only admins can perform in an existing encrypted Chat group conversation. A setting set to true restricts that action to admins, and false opens it to all members. Setting `send_message` to true makes the group announcement-only. Omitted settings keep their stored value. When sending `action_signatures`, sign over the full merged settings: the stored values with this request's changes applied, not only the settings in the request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID of the group conversation, starting with `g`. | [required] |
**update_chat_group_admin_settings_request** | Option<[**UpdateChatGroupAdminSettingsRequest**](UpdateChatGroupAdminSettingsRequest.md)> |  | [required] |

### Return type

[**models::UpdateChatGroupAdminSettingsResponse**](UpdateChatGroupAdminSettingsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_chat_settings

> models::UpdateChatSettingsResponse update_chat_settings(update_chat_settings_request)
Update Chat settings

Updates the authenticated user's Chat privacy settings: who can message them, read receipts, the message quality filter, and audio/video call permissions. allow_dms_from, dm_receipt_settings, and dm_quality_filter accept only their listed values, matched exactly and case-sensitively; anything else is a 400. av_settings must set at least one of its settings. Whether settings omitted from a partial av_settings keep their values or are reset is not yet confirmed; send every av_settings field you want to keep.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_chat_settings_request** | Option<[**UpdateChatSettingsRequest**](UpdateChatSettingsRequest.md)> |  | [required] |

### Return type

[**models::UpdateChatSettingsResponse**](UpdateChatSettingsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

