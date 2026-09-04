# \PostsApi

All URIs are relative to *https://api.x.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_posts**](PostsApi.md#create_posts) | **POST** /2/tweets | Create Posts
[**delete_posts**](PostsApi.md#delete_posts) | **DELETE** /2/tweets/{id} | Delete Posts
[**get_posts_analytics**](PostsApi.md#get_posts_analytics) | **GET** /2/tweets/analytics | Get Posts Analytics
[**get_posts_by_id**](PostsApi.md#get_posts_by_id) | **GET** /2/tweets/{id} | Get Posts by ID
[**get_posts_by_ids**](PostsApi.md#get_posts_by_ids) | **GET** /2/tweets | Get Posts by IDs
[**get_posts_counts_all**](PostsApi.md#get_posts_counts_all) | **GET** /2/tweets/counts/all | Get Posts Counts All
[**get_posts_counts_recent**](PostsApi.md#get_posts_counts_recent) | **GET** /2/tweets/counts/recent | Get Posts Counts Recent
[**get_posts_liking_users**](PostsApi.md#get_posts_liking_users) | **GET** /2/tweets/{id}/liking_users | Get Posts Liking Users
[**get_posts_quoted_posts**](PostsApi.md#get_posts_quoted_posts) | **GET** /2/tweets/{id}/quote_tweets | Get Posts Quoted Posts
[**get_posts_reposted_by**](PostsApi.md#get_posts_reposted_by) | **GET** /2/tweets/{id}/retweeted_by | Get Posts Reposted by
[**get_posts_reposts**](PostsApi.md#get_posts_reposts) | **GET** /2/tweets/{id}/retweets | Get Posts Reposts
[**hide_posts_reply**](PostsApi.md#hide_posts_reply) | **PUT** /2/tweets/{tweet_id}/hidden | Hide reply
[**search_posts_all**](PostsApi.md#search_posts_all) | **GET** /2/tweets/search/all | Search Posts All
[**search_posts_recent**](PostsApi.md#search_posts_recent) | **GET** /2/tweets/search/recent | Search Posts Recent



## create_posts

> models::CreatePostsResponse create_posts(create_posts_request)
Create Posts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_posts_request** | [**CreatePostsRequest**](CreatePostsRequest.md) |  | [required] |

### Return type

[**models::CreatePostsResponse**](CreatePostsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_posts

> models::DeletePostsResponse delete_posts(id)
Delete Posts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::DeletePostsResponse**](DeletePostsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_posts_analytics

> models::GetPostsAnalyticsResponse get_posts_analytics(ids, start_time, end_time, granularity, analytics_fields)
Get Posts Analytics

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) |  | [required] |
**start_time** | **chrono::DateTime<chrono::FixedOffset>** |  | [required] |
**end_time** | **chrono::DateTime<chrono::FixedOffset>** |  | [required] |
**granularity** | Option<**String**> |  |  |[default to total]
**analytics_fields** | Option<[**Vec<models::AnalyticsField>**](Models__AnalyticsField.md)> | A comma separated list of Analytics fields to display. |  |

### Return type

[**models::GetPostsAnalyticsResponse**](GetPostsAnalyticsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_posts_by_id

> models::GetPostsByIdResponse get_posts_by_id(id, post_fields, expansions, user_fields, media_fields, poll_fields, place_fields)
Get Posts by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**post_fields** | Option<[**Vec<models::PostField>**](Models__PostField.md)> | A comma separated list of Post fields to display. |  |
**expansions** | Option<[**Vec<models::PostExpansion>**](Models__PostExpansion.md)> | A comma separated list of fields to expand. |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |
**media_fields** | Option<[**Vec<models::MediaField>**](Models__MediaField.md)> | A comma separated list of Media fields to display. |  |
**poll_fields** | Option<[**Vec<models::PollField>**](Models__PollField.md)> | A comma separated list of Poll fields to display. |  |
**place_fields** | Option<[**Vec<models::PlaceField>**](Models__PlaceField.md)> | A comma separated list of Place fields to display. |  |

### Return type

[**models::GetPostsByIdResponse**](GetPostsByIdResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_posts_by_ids

> models::GetPostsByIdsResponse get_posts_by_ids(ids, post_fields, expansions, user_fields, media_fields, poll_fields, place_fields)
Get Posts by IDs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) |  | [required] |
**post_fields** | Option<[**Vec<models::PostField>**](Models__PostField.md)> | A comma separated list of Post fields to display. |  |
**expansions** | Option<[**Vec<models::PostExpansion>**](Models__PostExpansion.md)> | A comma separated list of fields to expand. |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |
**media_fields** | Option<[**Vec<models::MediaField>**](Models__MediaField.md)> | A comma separated list of Media fields to display. |  |
**poll_fields** | Option<[**Vec<models::PollField>**](Models__PollField.md)> | A comma separated list of Poll fields to display. |  |
**place_fields** | Option<[**Vec<models::PlaceField>**](Models__PlaceField.md)> | A comma separated list of Place fields to display. |  |

### Return type

[**models::GetPostsByIdsResponse**](GetPostsByIdsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_posts_counts_all

> models::GetPostsCountsAllResponse get_posts_counts_all(query, start_time, end_time, since_id, until_id, next_token, pagination_token, granularity, search_count_fields)
Get Posts Counts All

At most one of `pagination_token`, `next_token` may be provided.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** |  | [required] |
**start_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  |  |
**end_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  |  |
**since_id** | Option<**String**> |  |  |
**until_id** | Option<**String**> |  |  |
**next_token** | Option<**String**> | A base32hex-encoded pagination token. |  |
**pagination_token** | Option<**String**> | A base32hex-encoded pagination token. |  |
**granularity** | Option<**String**> |  |  |[default to hour]
**search_count_fields** | Option<**String**> |  |  |

### Return type

[**models::GetPostsCountsAllResponse**](GetPostsCountsAllResponse.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_posts_counts_recent

> models::GetPostsCountsRecentResponse get_posts_counts_recent(query, start_time, end_time, since_id, until_id, next_token, pagination_token, granularity, search_count_fields)
Get Posts Counts Recent

At most one of `pagination_token`, `next_token` may be provided.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** |  | [required] |
**start_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Must be within the last 7 days. |  |
**end_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  |  |
**since_id** | Option<**String**> |  |  |
**until_id** | Option<**String**> |  |  |
**next_token** | Option<**String**> | A base32hex-encoded pagination token. |  |
**pagination_token** | Option<**String**> | A base32hex-encoded pagination token. |  |
**granularity** | Option<**String**> |  |  |[default to hour]
**search_count_fields** | Option<**String**> |  |  |

### Return type

[**models::GetPostsCountsRecentResponse**](GetPostsCountsRecentResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_posts_liking_users

> models::GetPostsLikingUsersResponse get_posts_liking_users(id, max_results, pagination_token, user_fields, expansions, post_fields)
Get Posts Liking Users

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

[**models::GetPostsLikingUsersResponse**](GetPostsLikingUsersResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_posts_quoted_posts

> models::GetPostsQuotedPostsResponse get_posts_quoted_posts(id, max_results, pagination_token, exclude, post_fields, expansions, user_fields, media_fields, poll_fields, place_fields)
Get Posts Quoted Posts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**max_results** | Option<**i32**> |  |  |[default to 10]
**pagination_token** | Option<**String**> | A base32hex-encoded pagination token. |  |
**exclude** | Option<[**Vec<String>**](String.md)> |  |  |
**post_fields** | Option<[**Vec<models::PostField>**](Models__PostField.md)> | A comma separated list of Post fields to display. |  |
**expansions** | Option<[**Vec<models::PostExpansion>**](Models__PostExpansion.md)> | A comma separated list of fields to expand. |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |
**media_fields** | Option<[**Vec<models::MediaField>**](Models__MediaField.md)> | A comma separated list of Media fields to display. |  |
**poll_fields** | Option<[**Vec<models::PollField>**](Models__PollField.md)> | A comma separated list of Poll fields to display. |  |
**place_fields** | Option<[**Vec<models::PlaceField>**](Models__PlaceField.md)> | A comma separated list of Place fields to display. |  |

### Return type

[**models::GetPostsQuotedPostsResponse**](GetPostsQuotedPostsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_posts_reposted_by

> models::GetPostsRepostedByResponse get_posts_reposted_by(id, max_results, pagination_token, user_fields, expansions, post_fields)
Get Posts Reposted by

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

[**models::GetPostsRepostedByResponse**](GetPostsRepostedByResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_posts_reposts

> models::GetPostsRepostsResponse get_posts_reposts(id, max_results, pagination_token, post_fields, expansions, user_fields, media_fields, poll_fields, place_fields)
Get Posts Reposts

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

[**models::GetPostsRepostsResponse**](GetPostsRepostsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## hide_posts_reply

> models::HidePostsReplyResponse hide_posts_reply(tweet_id, hide_posts_reply_request)
Hide reply

Hides or unhides a reply to a conversation owned by the authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tweet_id** | **String** |  | [required] |
**hide_posts_reply_request** | [**HidePostsReplyRequest**](HidePostsReplyRequest.md) |  | [required] |

### Return type

[**models::HidePostsReplyResponse**](HidePostsReplyResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_posts_all

> models::SearchPostsAllResponse search_posts_all(query, start_time, end_time, since_id, until_id, max_results, next_token, pagination_token, sort_order, post_fields, expansions, user_fields, media_fields, poll_fields, place_fields)
Search Posts All

At most one of `start_time`, `since_id` may be provided. At most one of `end_time`, `until_id` may be provided. At most one of `pagination_token`, `next_token` may be provided.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** |  | [required] |
**start_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  |  |
**end_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  |  |
**since_id** | Option<**String**> |  |  |
**until_id** | Option<**String**> |  |  |
**max_results** | Option<**i32**> |  |  |[default to 10]
**next_token** | Option<**String**> | A base32hex-encoded pagination token. |  |
**pagination_token** | Option<**String**> | A base32hex-encoded pagination token. |  |
**sort_order** | Option<**String**> |  |  |
**post_fields** | Option<[**Vec<models::PostField>**](Models__PostField.md)> | A comma separated list of Post fields to display. |  |
**expansions** | Option<[**Vec<models::PostExpansion>**](Models__PostExpansion.md)> | A comma separated list of fields to expand. |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |
**media_fields** | Option<[**Vec<models::MediaField>**](Models__MediaField.md)> | A comma separated list of Media fields to display. |  |
**poll_fields** | Option<[**Vec<models::PollField>**](Models__PollField.md)> | A comma separated list of Poll fields to display. |  |
**place_fields** | Option<[**Vec<models::PlaceField>**](Models__PlaceField.md)> | A comma separated list of Place fields to display. |  |

### Return type

[**models::SearchPostsAllResponse**](SearchPostsAllResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_posts_recent

> models::SearchPostsRecentResponse search_posts_recent(query, max_results, next_token, pagination_token, start_time, end_time, since_id, until_id, sort_order, post_fields, expansions, user_fields, media_fields, poll_fields, place_fields)
Search Posts Recent

At most one of `start_time`, `since_id` may be provided. At most one of `end_time`, `until_id` may be provided. At most one of `pagination_token`, `next_token` may be provided.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** |  | [required] |
**max_results** | Option<**i32**> |  |  |[default to 10]
**next_token** | Option<**String**> | A base32hex-encoded pagination token. |  |
**pagination_token** | Option<**String**> | A base32hex-encoded pagination token. |  |
**start_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Must be within the last 7 days. |  |
**end_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  |  |
**since_id** | Option<**String**> |  |  |
**until_id** | Option<**String**> |  |  |
**sort_order** | Option<**String**> |  |  |
**post_fields** | Option<[**Vec<models::PostField>**](Models__PostField.md)> | A comma separated list of Post fields to display. |  |
**expansions** | Option<[**Vec<models::PostExpansion>**](Models__PostExpansion.md)> | A comma separated list of fields to expand. |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |
**media_fields** | Option<[**Vec<models::MediaField>**](Models__MediaField.md)> | A comma separated list of Media fields to display. |  |
**poll_fields** | Option<[**Vec<models::PollField>**](Models__PollField.md)> | A comma separated list of Poll fields to display. |  |
**place_fields** | Option<[**Vec<models::PlaceField>**](Models__PlaceField.md)> | A comma separated list of Place fields to display. |  |

### Return type

[**models::SearchPostsRecentResponse**](SearchPostsRecentResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

