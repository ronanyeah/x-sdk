# \DirectMessagesApi

All URIs are relative to *https://api.x.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_direct_messages_by_conversation_id**](DirectMessagesApi.md#create_direct_messages_by_conversation_id) | **POST** /2/dm_conversations/{dm_conversation_id}/messages | Create Direct Messages by Conversation ID
[**create_direct_messages_by_participant_id**](DirectMessagesApi.md#create_direct_messages_by_participant_id) | **POST** /2/dm_conversations/with/{participant_id}/messages | Create DM message by participant ID
[**create_direct_messages_conversation**](DirectMessagesApi.md#create_direct_messages_conversation) | **POST** /2/dm_conversations | Create DM conversation
[**delete_direct_messages_events**](DirectMessagesApi.md#delete_direct_messages_events) | **DELETE** /2/dm_events/{event_id} | Delete DM event
[**dm_conversations_media_download**](DirectMessagesApi.md#dm_conversations_media_download) | **GET** /2/dm_conversations/media/{dm_id}/{media_id}/{resource_id} | Download DM Media
[**get_direct_messages_events**](DirectMessagesApi.md#get_direct_messages_events) | **GET** /2/dm_events | Get Direct Messages Events
[**get_direct_messages_events_by_conversation_id**](DirectMessagesApi.md#get_direct_messages_events_by_conversation_id) | **GET** /2/dm_conversations/{id}/dm_events | Get Direct Messages Events by Conversation ID
[**get_direct_messages_events_by_id**](DirectMessagesApi.md#get_direct_messages_events_by_id) | **GET** /2/dm_events/{event_id} | Get Direct Messages Events by ID
[**get_direct_messages_events_by_participant_id**](DirectMessagesApi.md#get_direct_messages_events_by_participant_id) | **GET** /2/dm_conversations/with/{participant_id}/dm_events | Get Direct Messages Events by Participant ID



## create_direct_messages_by_conversation_id

> models::CreateDirectMessagesByConversationIdResponse create_direct_messages_by_conversation_id(dm_conversation_id, create_direct_messages_by_conversation_id_request)
Create Direct Messages by Conversation ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dm_conversation_id** | **String** |  | [required] |
**create_direct_messages_by_conversation_id_request** | Option<[**CreateDirectMessagesByConversationIdRequest**](CreateDirectMessagesByConversationIdRequest.md)> |  | [required] |

### Return type

[**models::CreateDirectMessagesByConversationIdResponse**](CreateDirectMessagesByConversationIdResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_direct_messages_by_participant_id

> models::CreateDirectMessagesByParticipantIdResponse create_direct_messages_by_participant_id(participant_id, create_direct_messages_by_participant_id_request)
Create DM message by participant ID

Sends a new direct message to a specific participant by their ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**participant_id** | **String** |  | [required] |
**create_direct_messages_by_participant_id_request** | Option<[**CreateDirectMessagesByParticipantIdRequest**](CreateDirectMessagesByParticipantIdRequest.md)> |  | [required] |

### Return type

[**models::CreateDirectMessagesByParticipantIdResponse**](CreateDirectMessagesByParticipantIdResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_direct_messages_conversation

> models::CreateDirectMessagesConversationResponse create_direct_messages_conversation(create_direct_messages_conversation_request)
Create DM conversation

Initiates a new direct message conversation with specified participants.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_direct_messages_conversation_request** | [**CreateDirectMessagesConversationRequest**](CreateDirectMessagesConversationRequest.md) |  | [required] |

### Return type

[**models::CreateDirectMessagesConversationResponse**](CreateDirectMessagesConversationResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_direct_messages_events

> models::DeleteDirectMessagesEventsResponse delete_direct_messages_events(event_id)
Delete DM event

Deletes a specific direct message event by its ID, if owned by the authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_id** | **String** |  | [required] |

### Return type

[**models::DeleteDirectMessagesEventsResponse**](DeleteDirectMessagesEventsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dm_conversations_media_download

> std::path::PathBuf dm_conversations_media_download(dm_id, media_id, resource_id)
Download DM Media

Downloads media attached to a legacy Direct Message. The requesting user must be a participant in the conversation containing the specified DM event. The response body contains raw binary bytes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dm_id** | **String** |  | [required] |
**media_id** | **String** |  | [required] |
**resource_id** | **String** |  | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_direct_messages_events

> models::GetDirectMessagesEventsResponse get_direct_messages_events(max_results, pagination_token, event_types, dm_event_fields, expansions, user_fields, post_fields, media_fields)
Get Direct Messages Events

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**max_results** | Option<**i32**> |  |  |[default to 100]
**pagination_token** | Option<**String**> | A base32hex-encoded pagination token. |  |
**event_types** | Option<[**Vec<String>**](String.md)> |  |  |[default to ["MessageCreate","ParticipantsLeave","ParticipantsJoin"]]
**dm_event_fields** | Option<[**Vec<models::DmEventField>**](Models__DmEventField.md)> | A comma separated list of DmEvent fields to display. |  |
**expansions** | Option<[**Vec<models::DmEventExpansion>**](Models__DmEventExpansion.md)> | A comma separated list of fields to expand. |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |
**post_fields** | Option<[**Vec<models::PostField>**](Models__PostField.md)> | A comma separated list of Post fields to display. |  |
**media_fields** | Option<[**Vec<models::MediaField>**](Models__MediaField.md)> | A comma separated list of Media fields to display. |  |

### Return type

[**models::GetDirectMessagesEventsResponse**](GetDirectMessagesEventsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_direct_messages_events_by_conversation_id

> models::GetDirectMessagesEventsByConversationIdResponse get_direct_messages_events_by_conversation_id(id, max_results, pagination_token, event_types, dm_event_fields, expansions, user_fields, post_fields, media_fields)
Get Direct Messages Events by Conversation ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**max_results** | Option<**i32**> |  |  |[default to 100]
**pagination_token** | Option<**String**> | A base32hex-encoded pagination token. |  |
**event_types** | Option<[**Vec<String>**](String.md)> |  |  |[default to ["MessageCreate","ParticipantsLeave","ParticipantsJoin"]]
**dm_event_fields** | Option<[**Vec<models::DmEventField>**](Models__DmEventField.md)> | A comma separated list of DmEvent fields to display. |  |
**expansions** | Option<[**Vec<models::DmEventExpansion>**](Models__DmEventExpansion.md)> | A comma separated list of fields to expand. |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |
**post_fields** | Option<[**Vec<models::PostField>**](Models__PostField.md)> | A comma separated list of Post fields to display. |  |
**media_fields** | Option<[**Vec<models::MediaField>**](Models__MediaField.md)> | A comma separated list of Media fields to display. |  |

### Return type

[**models::GetDirectMessagesEventsByConversationIdResponse**](GetDirectMessagesEventsByConversationIdResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_direct_messages_events_by_id

> models::GetDirectMessagesEventsByIdResponse get_direct_messages_events_by_id(event_id, dm_event_fields, expansions, user_fields, post_fields, media_fields)
Get Direct Messages Events by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_id** | **String** |  | [required] |
**dm_event_fields** | Option<[**Vec<models::DmEventField>**](Models__DmEventField.md)> | A comma separated list of DmEvent fields to display. |  |
**expansions** | Option<[**Vec<models::DmEventExpansion>**](Models__DmEventExpansion.md)> | A comma separated list of fields to expand. |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |
**post_fields** | Option<[**Vec<models::PostField>**](Models__PostField.md)> | A comma separated list of Post fields to display. |  |
**media_fields** | Option<[**Vec<models::MediaField>**](Models__MediaField.md)> | A comma separated list of Media fields to display. |  |

### Return type

[**models::GetDirectMessagesEventsByIdResponse**](GetDirectMessagesEventsByIdResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_direct_messages_events_by_participant_id

> models::GetDirectMessagesEventsByParticipantIdResponse get_direct_messages_events_by_participant_id(participant_id, max_results, pagination_token, event_types, dm_event_fields, expansions, user_fields, post_fields, media_fields)
Get Direct Messages Events by Participant ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**participant_id** | **String** |  | [required] |
**max_results** | Option<**i32**> |  |  |[default to 100]
**pagination_token** | Option<**String**> | A base32hex-encoded pagination token. |  |
**event_types** | Option<[**Vec<String>**](String.md)> |  |  |[default to ["MessageCreate","ParticipantsLeave","ParticipantsJoin"]]
**dm_event_fields** | Option<[**Vec<models::DmEventField>**](Models__DmEventField.md)> | A comma separated list of DmEvent fields to display. |  |
**expansions** | Option<[**Vec<models::DmEventExpansion>**](Models__DmEventExpansion.md)> | A comma separated list of fields to expand. |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |
**post_fields** | Option<[**Vec<models::PostField>**](Models__PostField.md)> | A comma separated list of Post fields to display. |  |
**media_fields** | Option<[**Vec<models::MediaField>**](Models__MediaField.md)> | A comma separated list of Media fields to display. |  |

### Return type

[**models::GetDirectMessagesEventsByParticipantIdResponse**](GetDirectMessagesEventsByParticipantIdResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

