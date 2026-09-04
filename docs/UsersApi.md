# \UsersApi

All URIs are relative to *https://api.x.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**block_users_dms**](UsersApi.md#block_users_dms) | **POST** /2/users/{id}/dm/block | Block Users Dms
[**create_users_bookmark**](UsersApi.md#create_users_bookmark) | **POST** /2/users/{id}/bookmarks | Create Bookmark
[**create_users_bookmark_folder**](UsersApi.md#create_users_bookmark_folder) | **POST** /2/users/{id}/bookmarks/folders | Create Bookmark Folder
[**delete_users_bookmark**](UsersApi.md#delete_users_bookmark) | **DELETE** /2/users/{id}/bookmarks/{tweet_id} | Delete Bookmark
[**follow_list**](UsersApi.md#follow_list) | **POST** /2/users/{id}/followed_lists | Follow List
[**follow_user**](UsersApi.md#follow_user) | **POST** /2/users/{id}/following | Follow User
[**get_users_affiliates**](UsersApi.md#get_users_affiliates) | **GET** /2/users/{id}/affiliates | Get Users Affiliates
[**get_users_blocking**](UsersApi.md#get_users_blocking) | **GET** /2/users/{id}/blocking | Get Users Blocking
[**get_users_bookmark_folders**](UsersApi.md#get_users_bookmark_folders) | **GET** /2/users/{id}/bookmarks/folders | Get Users Bookmark Folders
[**get_users_bookmarks**](UsersApi.md#get_users_bookmarks) | **GET** /2/users/{id}/bookmarks | Get Users Bookmarks
[**get_users_bookmarks_by_folder_id**](UsersApi.md#get_users_bookmarks_by_folder_id) | **GET** /2/users/{id}/bookmarks/folders/{folder_id} | Get Users Bookmarks by Folder ID
[**get_users_by_id**](UsersApi.md#get_users_by_id) | **GET** /2/users/{id} | Get Users by ID
[**get_users_by_ids**](UsersApi.md#get_users_by_ids) | **GET** /2/users | Get Users by IDs
[**get_users_by_username**](UsersApi.md#get_users_by_username) | **GET** /2/users/by/username/{username} | Get Users by Username
[**get_users_by_usernames**](UsersApi.md#get_users_by_usernames) | **GET** /2/users/by | Get Users by Usernames
[**get_users_followed_lists**](UsersApi.md#get_users_followed_lists) | **GET** /2/users/{id}/followed_lists | Get Users Followed Lists
[**get_users_followers**](UsersApi.md#get_users_followers) | **GET** /2/users/{id}/followers | Get Users Followers
[**get_users_following**](UsersApi.md#get_users_following) | **GET** /2/users/{id}/following | Get Users Following
[**get_users_liked_posts**](UsersApi.md#get_users_liked_posts) | **GET** /2/users/{id}/liked_tweets | Get Users Liked Posts
[**get_users_list_memberships**](UsersApi.md#get_users_list_memberships) | **GET** /2/users/{id}/list_memberships | Get Users List Memberships
[**get_users_me**](UsersApi.md#get_users_me) | **GET** /2/users/me | Get Users Me
[**get_users_mentions**](UsersApi.md#get_users_mentions) | **GET** /2/users/{id}/mentions | Get Users Mentions
[**get_users_muting**](UsersApi.md#get_users_muting) | **GET** /2/users/{id}/muting | Get Users Muting
[**get_users_owned_lists**](UsersApi.md#get_users_owned_lists) | **GET** /2/users/{id}/owned_lists | Get Users Owned Lists
[**get_users_pinned_lists**](UsersApi.md#get_users_pinned_lists) | **GET** /2/users/{id}/pinned_lists | Get Users Pinned Lists
[**get_users_posts**](UsersApi.md#get_users_posts) | **GET** /2/users/{id}/tweets | Get Users Posts
[**get_users_public_key**](UsersApi.md#get_users_public_key) | **GET** /2/users/{id}/public_keys | Get public keys
[**get_users_public_keys**](UsersApi.md#get_users_public_keys) | **GET** /2/users/public_keys | Get public keys
[**get_users_reposts_of_me**](UsersApi.md#get_users_reposts_of_me) | **GET** /2/users/reposts_of_me | Get Users Reposts of Me
[**get_users_timeline**](UsersApi.md#get_users_timeline) | **GET** /2/users/{id}/timelines/reverse_chronological | Get Users Timeline
[**like_post**](UsersApi.md#like_post) | **POST** /2/users/{id}/likes | Like Post
[**mute_user**](UsersApi.md#mute_user) | **POST** /2/users/{id}/muting | Mute User
[**pin_list**](UsersApi.md#pin_list) | **POST** /2/users/{id}/pinned_lists | Pin List
[**repost_post**](UsersApi.md#repost_post) | **POST** /2/users/{id}/retweets | Repost Post
[**search_users**](UsersApi.md#search_users) | **GET** /2/users/search | Search Users
[**unblock_users_dms**](UsersApi.md#unblock_users_dms) | **POST** /2/users/{id}/dm/unblock | Unblock Users Dms
[**unfollow_list**](UsersApi.md#unfollow_list) | **DELETE** /2/users/{id}/followed_lists/{list_id} | Unfollow a List
[**unfollow_user**](UsersApi.md#unfollow_user) | **DELETE** /2/users/{source_user_id}/following/{target_user_id} | Unfollow User
[**unlike_post**](UsersApi.md#unlike_post) | **DELETE** /2/users/{id}/likes/{tweet_id} | Unlike Post
[**unmute_user**](UsersApi.md#unmute_user) | **DELETE** /2/users/{source_user_id}/muting/{target_user_id} | Unmute User
[**unpin_list**](UsersApi.md#unpin_list) | **DELETE** /2/users/{id}/pinned_lists/{list_id} | Unpin a List
[**unrepost_post**](UsersApi.md#unrepost_post) | **DELETE** /2/users/{id}/retweets/{source_tweet_id} | Unrepost Post



## block_users_dms

> models::BlockUsersDmsResponse block_users_dms(id)
Block Users Dms

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::BlockUsersDmsResponse**](BlockUsersDmsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_users_bookmark

> models::CreateUsersBookmarkResponse create_users_bookmark(id, create_users_bookmark_request)
Create Bookmark

Adds one or more Posts to the authenticated user's Bookmarks. Supply `tweet_id` for a single Post, or `tweet_ids` (at most 25) to bookmark several in one request. Optional `folder_id` applies to every Post in the request. Billing and rate limits still apply per HTTP request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**create_users_bookmark_request** | Option<[**CreateUsersBookmarkRequest**](CreateUsersBookmarkRequest.md)> |  | [required] |

### Return type

[**models::CreateUsersBookmarkResponse**](CreateUsersBookmarkResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_users_bookmark_folder

> models::CreateUsersBookmarkFolderResponse create_users_bookmark_folder(id, create_users_bookmark_folder_request)
Create Bookmark Folder

Creates a new Bookmark folder for the authenticated user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**create_users_bookmark_folder_request** | [**CreateUsersBookmarkFolderRequest**](CreateUsersBookmarkFolderRequest.md) |  | [required] |

### Return type

[**models::CreateUsersBookmarkFolderResponse**](CreateUsersBookmarkFolderResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_users_bookmark

> models::DeleteUsersBookmarkResponse delete_users_bookmark(id, tweet_id)
Delete Bookmark

Removes a Post from the authenticated user's Bookmarks by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**tweet_id** | **String** |  | [required] |

### Return type

[**models::DeleteUsersBookmarkResponse**](DeleteUsersBookmarkResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## follow_list

> models::FollowListResponse follow_list(id, follow_list_request)
Follow List

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**follow_list_request** | [**FollowListRequest**](FollowListRequest.md) |  | [required] |

### Return type

[**models::FollowListResponse**](FollowListResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## follow_user

> models::FollowUserResponse follow_user(id, follow_user_request)
Follow User

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**follow_user_request** | [**FollowUserRequest**](FollowUserRequest.md) |  | [required] |

### Return type

[**models::FollowUserResponse**](FollowUserResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_affiliates

> models::GetUsersAffiliatesResponse get_users_affiliates(id, max_results, pagination_token, user_fields, expansions, post_fields)
Get Users Affiliates

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**max_results** | Option<**i32**> |  |  |
**pagination_token** | Option<**String**> | A 64-bit signed integer. |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |
**expansions** | Option<[**Vec<models::UserExpansion>**](Models__UserExpansion.md)> | A comma separated list of fields to expand. |  |
**post_fields** | Option<[**Vec<models::PostField>**](Models__PostField.md)> | A comma separated list of Post fields to display. |  |

### Return type

[**models::GetUsersAffiliatesResponse**](GetUsersAffiliatesResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_blocking

> models::GetUsersBlockingResponse get_users_blocking(id, max_results, pagination_token, user_fields, expansions, post_fields)
Get Users Blocking

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**max_results** | Option<**i32**> |  |  |
**pagination_token** | Option<**String**> |  |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |
**expansions** | Option<[**Vec<models::UserExpansion>**](Models__UserExpansion.md)> | A comma separated list of fields to expand. |  |
**post_fields** | Option<[**Vec<models::PostField>**](Models__PostField.md)> | A comma separated list of Post fields to display. |  |

### Return type

[**models::GetUsersBlockingResponse**](GetUsersBlockingResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_bookmark_folders

> models::GetUsersBookmarkFoldersResponse get_users_bookmark_folders(id, max_results, pagination_token)
Get Users Bookmark Folders

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**max_results** | Option<**i32**> |  |  |
**pagination_token** | Option<**String**> | A base32hex-encoded pagination token. |  |

### Return type

[**models::GetUsersBookmarkFoldersResponse**](GetUsersBookmarkFoldersResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_bookmarks

> models::GetUsersBookmarksResponse get_users_bookmarks(id, max_results, pagination_token, post_fields, expansions, user_fields, media_fields, poll_fields, place_fields)
Get Users Bookmarks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**max_results** | Option<**i32**> |  |  |
**pagination_token** | Option<**String**> | A base32hex-encoded pagination token. |  |
**post_fields** | Option<[**Vec<models::PostField>**](Models__PostField.md)> | A comma separated list of Post fields to display. |  |
**expansions** | Option<[**Vec<models::PostExpansion>**](Models__PostExpansion.md)> | A comma separated list of fields to expand. |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |
**media_fields** | Option<[**Vec<models::MediaField>**](Models__MediaField.md)> | A comma separated list of Media fields to display. |  |
**poll_fields** | Option<[**Vec<models::PollField>**](Models__PollField.md)> | A comma separated list of Poll fields to display. |  |
**place_fields** | Option<[**Vec<models::PlaceField>**](Models__PlaceField.md)> | A comma separated list of Place fields to display. |  |

### Return type

[**models::GetUsersBookmarksResponse**](GetUsersBookmarksResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_bookmarks_by_folder_id

> models::GetUsersBookmarksByFolderIdResponse get_users_bookmarks_by_folder_id(id, folder_id, max_results, pagination_token)
Get Users Bookmarks by Folder ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**folder_id** | **String** |  | [required] |
**max_results** | Option<**i32**> |  |  |
**pagination_token** | Option<**String**> | A base32hex-encoded pagination token. |  |

### Return type

[**models::GetUsersBookmarksByFolderIdResponse**](GetUsersBookmarksByFolderIdResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_by_id

> models::GetUsersByIdResponse get_users_by_id(id, user_fields, expansions, post_fields)
Get Users by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |
**expansions** | Option<[**Vec<models::UserExpansion>**](Models__UserExpansion.md)> | A comma separated list of fields to expand. |  |
**post_fields** | Option<[**Vec<models::PostField>**](Models__PostField.md)> | A comma separated list of Post fields to display. |  |

### Return type

[**models::GetUsersByIdResponse**](GetUsersByIdResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_by_ids

> models::GetUsersByIdsResponse get_users_by_ids(ids, user_fields, expansions, post_fields)
Get Users by IDs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) |  | [required] |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |
**expansions** | Option<[**Vec<models::UserExpansion>**](Models__UserExpansion.md)> | A comma separated list of fields to expand. |  |
**post_fields** | Option<[**Vec<models::PostField>**](Models__PostField.md)> | A comma separated list of Post fields to display. |  |

### Return type

[**models::GetUsersByIdsResponse**](GetUsersByIdsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_by_username

> models::GetUsersByUsernameResponse get_users_by_username(username, user_fields, expansions, post_fields)
Get Users by Username

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** |  | [required] |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |
**expansions** | Option<[**Vec<models::UserExpansion>**](Models__UserExpansion.md)> | A comma separated list of fields to expand. |  |
**post_fields** | Option<[**Vec<models::PostField>**](Models__PostField.md)> | A comma separated list of Post fields to display. |  |

### Return type

[**models::GetUsersByUsernameResponse**](GetUsersByUsernameResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_by_usernames

> models::GetUsersByUsernamesResponse get_users_by_usernames(usernames, user_fields, expansions, post_fields)
Get Users by Usernames

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**usernames** | [**Vec<String>**](String.md) |  | [required] |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |
**expansions** | Option<[**Vec<models::UserExpansion>**](Models__UserExpansion.md)> | A comma separated list of fields to expand. |  |
**post_fields** | Option<[**Vec<models::PostField>**](Models__PostField.md)> | A comma separated list of Post fields to display. |  |

### Return type

[**models::GetUsersByUsernamesResponse**](GetUsersByUsernamesResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_followed_lists

> models::GetUsersFollowedListsResponse get_users_followed_lists(id, max_results, pagination_token, list_fields, expansions, user_fields)
Get Users Followed Lists

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**max_results** | Option<**i32**> |  |  |[default to 100]
**pagination_token** | Option<**String**> | A 64-bit signed integer. |  |
**list_fields** | Option<[**Vec<models::ListField>**](Models__ListField.md)> | A comma separated list of List fields to display. |  |
**expansions** | Option<[**Vec<models::ListExpansion>**](Models__ListExpansion.md)> | A comma separated list of fields to expand. |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |

### Return type

[**models::GetUsersFollowedListsResponse**](GetUsersFollowedListsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_followers

> models::GetUsersFollowersResponse get_users_followers(id, max_results, pagination_token, user_fields, expansions, post_fields)
Get Users Followers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**max_results** | Option<**i32**> |  |  |
**pagination_token** | Option<**String**> | A base32hex-encoded pagination token. |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |
**expansions** | Option<[**Vec<models::UserExpansion>**](Models__UserExpansion.md)> | A comma separated list of fields to expand. |  |
**post_fields** | Option<[**Vec<models::PostField>**](Models__PostField.md)> | A comma separated list of Post fields to display. |  |

### Return type

[**models::GetUsersFollowersResponse**](GetUsersFollowersResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_following

> models::GetUsersFollowingResponse get_users_following(id, max_results, pagination_token, user_fields, expansions, post_fields)
Get Users Following

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**max_results** | Option<**i32**> |  |  |
**pagination_token** | Option<**String**> | A base32hex-encoded pagination token. |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |
**expansions** | Option<[**Vec<models::UserExpansion>**](Models__UserExpansion.md)> | A comma separated list of fields to expand. |  |
**post_fields** | Option<[**Vec<models::PostField>**](Models__PostField.md)> | A comma separated list of Post fields to display. |  |

### Return type

[**models::GetUsersFollowingResponse**](GetUsersFollowingResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_liked_posts

> models::GetUsersLikedPostsResponse get_users_liked_posts(id, max_results, pagination_token, post_fields, expansions, user_fields, media_fields, poll_fields, place_fields)
Get Users Liked Posts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**max_results** | Option<**i32**> |  |  |
**pagination_token** | Option<**String**> | A base32hex-encoded pagination token. |  |
**post_fields** | Option<[**Vec<models::PostField>**](Models__PostField.md)> | A comma separated list of Post fields to display. |  |
**expansions** | Option<[**Vec<models::PostExpansion>**](Models__PostExpansion.md)> | A comma separated list of fields to expand. |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |
**media_fields** | Option<[**Vec<models::MediaField>**](Models__MediaField.md)> | A comma separated list of Media fields to display. |  |
**poll_fields** | Option<[**Vec<models::PollField>**](Models__PollField.md)> | A comma separated list of Poll fields to display. |  |
**place_fields** | Option<[**Vec<models::PlaceField>**](Models__PlaceField.md)> | A comma separated list of Place fields to display. |  |

### Return type

[**models::GetUsersLikedPostsResponse**](GetUsersLikedPostsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_list_memberships

> models::GetUsersListMembershipsResponse get_users_list_memberships(id, max_results, pagination_token, list_fields, expansions, user_fields)
Get Users List Memberships

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**max_results** | Option<**i32**> |  |  |[default to 100]
**pagination_token** | Option<**String**> |  |  |
**list_fields** | Option<[**Vec<models::ListField>**](Models__ListField.md)> | A comma separated list of List fields to display. |  |
**expansions** | Option<[**Vec<models::ListExpansion>**](Models__ListExpansion.md)> | A comma separated list of fields to expand. |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |

### Return type

[**models::GetUsersListMembershipsResponse**](GetUsersListMembershipsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_me

> models::GetUsersMeResponse get_users_me(user_fields, expansions, post_fields)
Get Users Me

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |
**expansions** | Option<[**Vec<models::UserExpansion>**](Models__UserExpansion.md)> | A comma separated list of fields to expand. |  |
**post_fields** | Option<[**Vec<models::PostField>**](Models__PostField.md)> | A comma separated list of Post fields to display. |  |

### Return type

[**models::GetUsersMeResponse**](GetUsersMeResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_mentions

> models::GetUsersMentionsResponse get_users_mentions(id, max_results, pagination_token, start_time, end_time, since_id, until_id, post_fields, expansions, user_fields, media_fields, poll_fields, place_fields)
Get Users Mentions

When both are provided, `start_time` must be earlier than `end_time`. When both are provided, `since_id` must be less than `until_id`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**max_results** | Option<**i32**> |  |  |
**pagination_token** | Option<**String**> | A base32hex-encoded pagination token. |  |
**start_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Must be on or after 2010-11-06. |  |
**end_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Must be on or after 2010-11-06. |  |
**since_id** | Option<**String**> |  |  |
**until_id** | Option<**String**> |  |  |
**post_fields** | Option<[**Vec<models::PostField>**](Models__PostField.md)> | A comma separated list of Post fields to display. |  |
**expansions** | Option<[**Vec<models::PostExpansion>**](Models__PostExpansion.md)> | A comma separated list of fields to expand. |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |
**media_fields** | Option<[**Vec<models::MediaField>**](Models__MediaField.md)> | A comma separated list of Media fields to display. |  |
**poll_fields** | Option<[**Vec<models::PollField>**](Models__PollField.md)> | A comma separated list of Poll fields to display. |  |
**place_fields** | Option<[**Vec<models::PlaceField>**](Models__PlaceField.md)> | A comma separated list of Place fields to display. |  |

### Return type

[**models::GetUsersMentionsResponse**](GetUsersMentionsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_muting

> models::GetUsersMutingResponse get_users_muting(id, max_results, pagination_token, user_fields, expansions, post_fields)
Get Users Muting

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**max_results** | Option<**i32**> |  |  |[default to 100]
**pagination_token** | Option<**String**> |  |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |
**expansions** | Option<[**Vec<models::UserExpansion>**](Models__UserExpansion.md)> | A comma separated list of fields to expand. |  |
**post_fields** | Option<[**Vec<models::PostField>**](Models__PostField.md)> | A comma separated list of Post fields to display. |  |

### Return type

[**models::GetUsersMutingResponse**](GetUsersMutingResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_owned_lists

> models::GetUsersOwnedListsResponse get_users_owned_lists(id, max_results, pagination_token, list_fields, expansions, user_fields)
Get Users Owned Lists

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**max_results** | Option<**i32**> |  |  |[default to 100]
**pagination_token** | Option<**String**> | A 64-bit signed integer. |  |
**list_fields** | Option<[**Vec<models::ListField>**](Models__ListField.md)> | A comma separated list of List fields to display. |  |
**expansions** | Option<[**Vec<models::ListExpansion>**](Models__ListExpansion.md)> | A comma separated list of fields to expand. |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |

### Return type

[**models::GetUsersOwnedListsResponse**](GetUsersOwnedListsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_pinned_lists

> models::GetUsersPinnedListsResponse get_users_pinned_lists(id, list_fields, expansions, user_fields)
Get Users Pinned Lists

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**list_fields** | Option<[**Vec<models::ListField>**](Models__ListField.md)> | A comma separated list of List fields to display. |  |
**expansions** | Option<[**Vec<models::ListExpansion>**](Models__ListExpansion.md)> | A comma separated list of fields to expand. |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |

### Return type

[**models::GetUsersPinnedListsResponse**](GetUsersPinnedListsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_posts

> models::GetUsersPostsResponse get_users_posts(id, max_results, pagination_token, start_time, end_time, since_id, until_id, exclude, post_fields, expansions, user_fields, media_fields, poll_fields, place_fields)
Get Users Posts

When both are provided, `start_time` must be earlier than `end_time`. When both are provided, `since_id` must be less than `until_id`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**max_results** | Option<**i32**> |  |  |
**pagination_token** | Option<**String**> | A base32hex-encoded pagination token. |  |
**start_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Must be on or after 2010-11-06. |  |
**end_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Must be on or after 2010-11-06. |  |
**since_id** | Option<**String**> |  |  |
**until_id** | Option<**String**> |  |  |
**exclude** | Option<[**Vec<String>**](String.md)> |  |  |
**post_fields** | Option<[**Vec<models::PostField>**](Models__PostField.md)> | A comma separated list of Post fields to display. |  |
**expansions** | Option<[**Vec<models::PostExpansion>**](Models__PostExpansion.md)> | A comma separated list of fields to expand. |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |
**media_fields** | Option<[**Vec<models::MediaField>**](Models__MediaField.md)> | A comma separated list of Media fields to display. |  |
**poll_fields** | Option<[**Vec<models::PollField>**](Models__PollField.md)> | A comma separated list of Poll fields to display. |  |
**place_fields** | Option<[**Vec<models::PlaceField>**](Models__PlaceField.md)> | A comma separated list of Place fields to display. |  |

### Return type

[**models::GetUsersPostsResponse**](GetUsersPostsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_public_key

> models::GetUsersPublicKeyResponse get_users_public_key(id, public_key_fields)
Get public keys

Returns a user's registered public keys for X Chat encryption.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**public_key_fields** | Option<[**Vec<models::PublicKeyField>**](Models__PublicKeyField.md)> | A comma separated list of PublicKey fields to display. |  |

### Return type

[**models::GetUsersPublicKeyResponse**](GetUsersPublicKeyResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_public_keys

> models::GetUsersPublicKeysResponse get_users_public_keys(ids, public_key_fields)
Get public keys

Returns registered public keys for X Chat encryption for the specified users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<String>**](String.md) |  | [required] |
**public_key_fields** | Option<[**Vec<models::PublicKeyField>**](Models__PublicKeyField.md)> | A comma separated list of PublicKey fields to display. |  |

### Return type

[**models::GetUsersPublicKeysResponse**](GetUsersPublicKeysResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_reposts_of_me

> models::GetUsersRepostsOfMeResponse get_users_reposts_of_me(max_results, pagination_token, post_fields, expansions, user_fields, media_fields, poll_fields, place_fields)
Get Users Reposts of Me

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**max_results** | Option<**i32**> |  |  |[default to 100]
**pagination_token** | Option<**String**> | A base32hex-encoded pagination token. |  |
**post_fields** | Option<[**Vec<models::PostField>**](Models__PostField.md)> | A comma separated list of Post fields to display. |  |
**expansions** | Option<[**Vec<models::PostExpansion>**](Models__PostExpansion.md)> | A comma separated list of fields to expand. |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |
**media_fields** | Option<[**Vec<models::MediaField>**](Models__MediaField.md)> | A comma separated list of Media fields to display. |  |
**poll_fields** | Option<[**Vec<models::PollField>**](Models__PollField.md)> | A comma separated list of Poll fields to display. |  |
**place_fields** | Option<[**Vec<models::PlaceField>**](Models__PlaceField.md)> | A comma separated list of Place fields to display. |  |

### Return type

[**models::GetUsersRepostsOfMeResponse**](GetUsersRepostsOfMeResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_timeline

> models::GetUsersTimelineResponse get_users_timeline(id, max_results, pagination_token, start_time, end_time, since_id, until_id, exclude, post_fields, expansions, user_fields, media_fields, poll_fields, place_fields)
Get Users Timeline

When both are provided, `start_time` must be earlier than `end_time`. When both are provided, `since_id` must be less than `until_id`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**max_results** | Option<**i32**> |  |  |
**pagination_token** | Option<**String**> | A base32hex-encoded pagination token. |  |
**start_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Must be on or after 2010-11-06. |  |
**end_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Must be on or after 2010-11-06. |  |
**since_id** | Option<**String**> |  |  |
**until_id** | Option<**String**> |  |  |
**exclude** | Option<[**Vec<String>**](String.md)> |  |  |
**post_fields** | Option<[**Vec<models::PostField>**](Models__PostField.md)> | A comma separated list of Post fields to display. |  |
**expansions** | Option<[**Vec<models::PostExpansion>**](Models__PostExpansion.md)> | A comma separated list of fields to expand. |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |
**media_fields** | Option<[**Vec<models::MediaField>**](Models__MediaField.md)> | A comma separated list of Media fields to display. |  |
**poll_fields** | Option<[**Vec<models::PollField>**](Models__PollField.md)> | A comma separated list of Poll fields to display. |  |
**place_fields** | Option<[**Vec<models::PlaceField>**](Models__PlaceField.md)> | A comma separated list of Place fields to display. |  |

### Return type

[**models::GetUsersTimelineResponse**](GetUsersTimelineResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## like_post

> models::LikePostResponse like_post(id, like_post_request)
Like Post

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**like_post_request** | [**LikePostRequest**](LikePostRequest.md) |  | [required] |

### Return type

[**models::LikePostResponse**](LikePostResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mute_user

> models::MuteUserResponse mute_user(id, mute_user_request)
Mute User

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**mute_user_request** | [**MuteUserRequest**](MuteUserRequest.md) |  | [required] |

### Return type

[**models::MuteUserResponse**](MuteUserResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pin_list

> models::PinListResponse pin_list(id, pin_list_request)
Pin List

Causes the authenticated user to pin a specific List by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**pin_list_request** | [**PinListRequest**](PinListRequest.md) |  | [required] |

### Return type

[**models::PinListResponse**](PinListResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## repost_post

> models::RepostPostResponse repost_post(id, repost_post_request)
Repost Post

Causes the authenticated user to repost a specific Post by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**repost_post_request** | [**RepostPostRequest**](RepostPostRequest.md) |  | [required] |

### Return type

[**models::RepostPostResponse**](RepostPostResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_users

> models::SearchUsersResponse search_users(query, max_results, next_token, user_fields, expansions, post_fields)
Search Users

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** |  | [required] |
**max_results** | Option<**i32**> |  |  |[default to 100]
**next_token** | Option<**String**> |  |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |
**expansions** | Option<[**Vec<models::UserExpansion>**](Models__UserExpansion.md)> | A comma separated list of fields to expand. |  |
**post_fields** | Option<[**Vec<models::PostField>**](Models__PostField.md)> | A comma separated list of Post fields to display. |  |

### Return type

[**models::SearchUsersResponse**](SearchUsersResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unblock_users_dms

> models::UnblockUsersDmsResponse unblock_users_dms(id)
Unblock Users Dms

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::UnblockUsersDmsResponse**](UnblockUsersDmsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unfollow_list

> models::UnfollowListResponse unfollow_list(id, list_id)
Unfollow a List

Causes the authenticated user to unfollow a List by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**list_id** | **String** |  | [required] |

### Return type

[**models::UnfollowListResponse**](UnfollowListResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unfollow_user

> models::UnfollowUserResponse unfollow_user(source_user_id, target_user_id)
Unfollow User

Causes the authenticated user to unfollow a specific user by their ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_user_id** | **String** |  | [required] |
**target_user_id** | **String** |  | [required] |

### Return type

[**models::UnfollowUserResponse**](UnfollowUserResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlike_post

> models::UnlikePostResponse unlike_post(id, tweet_id)
Unlike Post

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**tweet_id** | **String** |  | [required] |

### Return type

[**models::UnlikePostResponse**](UnlikePostResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unmute_user

> models::UnmuteUserResponse unmute_user(source_user_id, target_user_id)
Unmute User

Causes the authenticated user to unmute the target user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**source_user_id** | **String** |  | [required] |
**target_user_id** | **String** |  | [required] |

### Return type

[**models::UnmuteUserResponse**](UnmuteUserResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unpin_list

> models::UnpinListResponse unpin_list(id, list_id)
Unpin a List

Causes the authenticated user to unpin a List by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**list_id** | **String** |  | [required] |

### Return type

[**models::UnpinListResponse**](UnpinListResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unrepost_post

> models::UnrepostPostResponse unrepost_post(id, source_tweet_id)
Unrepost Post

Causes the authenticated user to unrepost a specific Post by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**source_tweet_id** | **String** |  | [required] |

### Return type

[**models::UnrepostPostResponse**](UnrepostPostResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

