# \CommunityNotesApi

All URIs are relative to *https://api.x.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_community_notes**](CommunityNotesApi.md#create_community_notes) | **POST** /2/notes | Create Community Notes
[**delete_community_notes**](CommunityNotesApi.md#delete_community_notes) | **DELETE** /2/notes/{id} | Delete a Community Note
[**evaluate_community_notes**](CommunityNotesApi.md#evaluate_community_notes) | **POST** /2/notes/evaluate | Evaluate Community Notes
[**search_community_notes_written**](CommunityNotesApi.md#search_community_notes_written) | **GET** /2/notes/search/notes_written | Search Community Notes Written
[**search_eligible_posts**](CommunityNotesApi.md#search_eligible_posts) | **GET** /2/notes/search/posts_eligible_for_notes | Search Eligible Posts



## create_community_notes

> models::CreateCommunityNotesResponse create_community_notes(create_community_notes_request)
Create Community Notes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_community_notes_request** | [**CreateCommunityNotesRequest**](CreateCommunityNotesRequest.md) |  | [required] |

### Return type

[**models::CreateCommunityNotesResponse**](CreateCommunityNotesResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_community_notes

> models::DeleteCommunityNotesResponse delete_community_notes(id)
Delete a Community Note

Deletes a community note.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::DeleteCommunityNotesResponse**](DeleteCommunityNotesResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## evaluate_community_notes

> models::EvaluateCommunityNotesResponse evaluate_community_notes(evaluate_community_notes_request)
Evaluate Community Notes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**evaluate_community_notes_request** | [**EvaluateCommunityNotesRequest**](EvaluateCommunityNotesRequest.md) |  | [required] |

### Return type

[**models::EvaluateCommunityNotesResponse**](EvaluateCommunityNotesResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_community_notes_written

> models::SearchCommunityNotesWrittenResponse search_community_notes_written(test_mode, max_results, pagination_token, note_fields)
Search Community Notes Written

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**test_mode** | **bool** |  | [required] |
**max_results** | Option<**i32**> |  |  |[default to 10]
**pagination_token** | Option<**String**> |  |  |
**note_fields** | Option<[**Vec<models::NoteField>**](Models__NoteField.md)> | A comma separated list of Note fields to display. |  |

### Return type

[**models::SearchCommunityNotesWrittenResponse**](SearchCommunityNotesWrittenResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_eligible_posts

> models::SearchEligiblePostsResponse search_eligible_posts(test_mode, max_results, pagination_token, post_selection, post_fields, expansions, user_fields, media_fields, poll_fields, place_fields)
Search Eligible Posts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**test_mode** | **bool** |  | [required] |
**max_results** | Option<**i32**> |  |  |[default to 10]
**pagination_token** | Option<**String**> |  |  |
**post_selection** | Option<**String**> |  |  |
**post_fields** | Option<[**Vec<models::PostField>**](Models__PostField.md)> | A comma separated list of Post fields to display. |  |
**expansions** | Option<[**Vec<models::PostExpansion>**](Models__PostExpansion.md)> | A comma separated list of fields to expand. |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |
**media_fields** | Option<[**Vec<models::MediaField>**](Models__MediaField.md)> | A comma separated list of Media fields to display. |  |
**poll_fields** | Option<[**Vec<models::PollField>**](Models__PollField.md)> | A comma separated list of Poll fields to display. |  |
**place_fields** | Option<[**Vec<models::PlaceField>**](Models__PlaceField.md)> | A comma separated list of Place fields to display. |  |

### Return type

[**models::SearchEligiblePostsResponse**](SearchEligiblePostsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

