# \SpacesApi

All URIs are relative to *https://api.x.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_spaces_buyers**](SpacesApi.md#get_spaces_buyers) | **GET** /2/spaces/{id}/buyers | Get Space ticket buyers
[**get_spaces_by_creator_ids**](SpacesApi.md#get_spaces_by_creator_ids) | **GET** /2/spaces/by/creator_ids | Get Spaces by Creator IDs
[**get_spaces_by_id**](SpacesApi.md#get_spaces_by_id) | **GET** /2/spaces/{id} | Get Spaces by ID
[**get_spaces_by_ids**](SpacesApi.md#get_spaces_by_ids) | **GET** /2/spaces | Get Spaces by IDs
[**get_spaces_posts**](SpacesApi.md#get_spaces_posts) | **GET** /2/spaces/{id}/tweets | Get Space Posts
[**search_spaces**](SpacesApi.md#search_spaces) | **GET** /2/spaces/search | Search Spaces



## get_spaces_buyers

> models::GetSpacesBuyersResponse get_spaces_buyers(id, max_results, pagination_token, user_fields, expansions, post_fields)
Get Space ticket buyers

Retrieves a list of Users who purchased tickets to a specific Space by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**max_results** | Option<**i32**> |  |  |[default to 100]
**pagination_token** | Option<**String**> | A base32hex-encoded pagination token. |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |
**expansions** | Option<[**Vec<models::UserExpansion>**](Models__UserExpansion.md)> | A comma separated list of fields to expand. |  |
**post_fields** | Option<[**Vec<models::PostField>**](Models__PostField.md)> | A comma separated list of Post fields to display. |  |

### Return type

[**models::GetSpacesBuyersResponse**](GetSpacesBuyersResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_spaces_by_creator_ids

> models::GetSpacesByCreatorIdsResponse get_spaces_by_creator_ids(user_ids, space_fields, expansions, user_fields, topic_fields)
Get Spaces by Creator IDs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_ids** | [**Vec<String>**](String.md) |  | [required] |
**space_fields** | Option<[**Vec<models::SpaceField>**](Models__SpaceField.md)> | A comma separated list of Space fields to display. |  |
**expansions** | Option<[**Vec<models::SpaceExpansion>**](Models__SpaceExpansion.md)> | A comma separated list of fields to expand. |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |
**topic_fields** | Option<[**Vec<models::TopicField>**](Models__TopicField.md)> | A comma separated list of Topic fields to display. |  |

### Return type

[**models::GetSpacesByCreatorIdsResponse**](GetSpacesByCreatorIdsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_spaces_by_id

> models::GetSpacesByIdResponse get_spaces_by_id(id, space_fields, expansions, user_fields, topic_fields)
Get Spaces by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**space_fields** | Option<[**Vec<models::SpaceField>**](Models__SpaceField.md)> | A comma separated list of Space fields to display. |  |
**expansions** | Option<[**Vec<models::SpaceExpansion>**](Models__SpaceExpansion.md)> | A comma separated list of fields to expand. |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |
**topic_fields** | Option<[**Vec<models::TopicField>**](Models__TopicField.md)> | A comma separated list of Topic fields to display. |  |

### Return type

[**models::GetSpacesByIdResponse**](GetSpacesByIdResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_spaces_by_ids

> models::GetSpacesByIdsResponse get_spaces_by_ids(ids, space_fields, expansions, user_fields, topic_fields)
Get Spaces by IDs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) |  | [required] |
**space_fields** | Option<[**Vec<models::SpaceField>**](Models__SpaceField.md)> | A comma separated list of Space fields to display. |  |
**expansions** | Option<[**Vec<models::SpaceExpansion>**](Models__SpaceExpansion.md)> | A comma separated list of fields to expand. |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |
**topic_fields** | Option<[**Vec<models::TopicField>**](Models__TopicField.md)> | A comma separated list of Topic fields to display. |  |

### Return type

[**models::GetSpacesByIdsResponse**](GetSpacesByIdsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_spaces_posts

> models::GetSpacesPostsResponse get_spaces_posts(id, max_results, pagination_token, post_fields, expansions, user_fields, media_fields, poll_fields, place_fields)
Get Space Posts

Retrieves a list of Posts shared in a specific Space by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**max_results** | Option<**i32**> |  |  |[default to 100]
**pagination_token** | Option<**String**> | A base32hex-encoded pagination token. |  |
**post_fields** | Option<[**Vec<models::PostField>**](Models__PostField.md)> | A comma separated list of Post fields to display. |  |
**expansions** | Option<[**Vec<models::PostExpansion>**](Models__PostExpansion.md)> | A comma separated list of fields to expand. |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |
**media_fields** | Option<[**Vec<models::MediaField>**](Models__MediaField.md)> | A comma separated list of Media fields to display. |  |
**poll_fields** | Option<[**Vec<models::PollField>**](Models__PollField.md)> | A comma separated list of Poll fields to display. |  |
**place_fields** | Option<[**Vec<models::PlaceField>**](Models__PlaceField.md)> | A comma separated list of Place fields to display. |  |

### Return type

[**models::GetSpacesPostsResponse**](GetSpacesPostsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_spaces

> models::SearchSpacesResponse search_spaces(query, state, max_results, space_fields, expansions, user_fields, topic_fields)
Search Spaces

Retrieves a list of Spaces matching the specified search query.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** |  | [required] |
**state** | Option<**String**> |  |  |[default to all]
**max_results** | Option<**i32**> |  |  |[default to 100]
**space_fields** | Option<[**Vec<models::SpaceField>**](Models__SpaceField.md)> | A comma separated list of Space fields to display. |  |
**expansions** | Option<[**Vec<models::SpaceExpansion>**](Models__SpaceExpansion.md)> | A comma separated list of fields to expand. |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |
**topic_fields** | Option<[**Vec<models::TopicField>**](Models__TopicField.md)> | A comma separated list of Topic fields to display. |  |

### Return type

[**models::SearchSpacesResponse**](SearchSpacesResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

