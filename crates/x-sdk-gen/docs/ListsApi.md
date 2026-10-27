# \ListsApi

All URIs are relative to *https://api.x.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_lists_member**](ListsApi.md#add_lists_member) | **POST** /2/lists/{id}/members | Add Lists Member
[**create_lists**](ListsApi.md#create_lists) | **POST** /2/lists | Create Lists
[**delete_lists**](ListsApi.md#delete_lists) | **DELETE** /2/lists/{id} | Delete List
[**get_lists_by_id**](ListsApi.md#get_lists_by_id) | **GET** /2/lists/{id} | Get Lists by ID
[**get_lists_followers**](ListsApi.md#get_lists_followers) | **GET** /2/lists/{id}/followers | Get Lists Followers
[**get_lists_members**](ListsApi.md#get_lists_members) | **GET** /2/lists/{id}/members | Get Lists Members
[**get_lists_posts**](ListsApi.md#get_lists_posts) | **GET** /2/lists/{id}/tweets | Get Lists Posts
[**remove_lists_member_by_user_id**](ListsApi.md#remove_lists_member_by_user_id) | **DELETE** /2/lists/{id}/members/{user_id} | Remove a List member
[**update_lists**](ListsApi.md#update_lists) | **PUT** /2/lists/{id} | Update List



## add_lists_member

> models::AddListsMemberResponse add_lists_member(id, add_lists_member_request)
Add Lists Member

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**add_lists_member_request** | [**AddListsMemberRequest**](AddListsMemberRequest.md) |  | [required] |

### Return type

[**models::AddListsMemberResponse**](AddListsMemberResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_lists

> models::CreateListsResponse create_lists(create_lists_request)
Create Lists

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_lists_request** | [**CreateListsRequest**](CreateListsRequest.md) |  | [required] |

### Return type

[**models::CreateListsResponse**](CreateListsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_lists

> models::DeleteListsResponse delete_lists(id)
Delete List

Deletes a specific List owned by the authenticated user by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::DeleteListsResponse**](DeleteListsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lists_by_id

> models::GetListsByIdResponse get_lists_by_id(id, list_fields, expansions, user_fields)
Get Lists by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**list_fields** | Option<[**Vec<models::ListField>**](Models__ListField.md)> | A comma separated list of List fields to display. |  |
**expansions** | Option<[**Vec<models::ListExpansion>**](Models__ListExpansion.md)> | A comma separated list of fields to expand. |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |

### Return type

[**models::GetListsByIdResponse**](GetListsByIdResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lists_followers

> models::GetListsFollowersResponse get_lists_followers(id, max_results, pagination_token, user_fields, expansions, post_fields)
Get Lists Followers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**max_results** | Option<**i32**> |  |  |[default to 100]
**pagination_token** | Option<**String**> | A 64-bit signed integer. |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |
**expansions** | Option<[**Vec<models::UserExpansion>**](Models__UserExpansion.md)> | A comma separated list of fields to expand. |  |
**post_fields** | Option<[**Vec<models::PostField>**](Models__PostField.md)> | A comma separated list of Post fields to display. |  |

### Return type

[**models::GetListsFollowersResponse**](GetListsFollowersResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lists_members

> models::GetListsMembersResponse get_lists_members(id, max_results, pagination_token, user_fields, expansions, post_fields)
Get Lists Members

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**max_results** | Option<**i32**> |  |  |[default to 100]
**pagination_token** | Option<**String**> | A 64-bit signed integer. |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |
**expansions** | Option<[**Vec<models::UserExpansion>**](Models__UserExpansion.md)> | A comma separated list of fields to expand. |  |
**post_fields** | Option<[**Vec<models::PostField>**](Models__PostField.md)> | A comma separated list of Post fields to display. |  |

### Return type

[**models::GetListsMembersResponse**](GetListsMembersResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_lists_posts

> models::GetListsPostsResponse get_lists_posts(id, max_results, pagination_token, post_fields, expansions, user_fields, media_fields, poll_fields, place_fields)
Get Lists Posts

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

[**models::GetListsPostsResponse**](GetListsPostsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_lists_member_by_user_id

> models::RemoveListsMemberByUserIdResponse remove_lists_member_by_user_id(id, user_id)
Remove a List member

Removes a User from a List by their ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**user_id** | **String** |  | [required] |

### Return type

[**models::RemoveListsMemberByUserIdResponse**](RemoveListsMemberByUserIdResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_lists

> models::UpdateListsResponse update_lists(id, update_lists_request)
Update List

Updates the details of a specific List owned by the authenticated user by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**update_lists_request** | [**UpdateListsRequest**](UpdateListsRequest.md) |  | [required] |

### Return type

[**models::UpdateListsResponse**](UpdateListsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

