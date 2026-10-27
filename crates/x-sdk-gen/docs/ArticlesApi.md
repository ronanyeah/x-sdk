# \ArticlesApi

All URIs are relative to *https://api.x.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**article_create_draft**](ArticlesApi.md#article_create_draft) | **POST** /2/articles/draft | Create draft Article
[**article_publish**](ArticlesApi.md#article_publish) | **POST** /2/articles/{article_id}/publish | Publish Article



## article_create_draft

> models::ArticleCreateDraftResponse article_create_draft(article_create_draft_request)
Create draft Article

Creates a new Article draft that can later be published.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**article_create_draft_request** | [**ArticleCreateDraftRequest**](ArticleCreateDraftRequest.md) |  | [required] |

### Return type

[**models::ArticleCreateDraftResponse**](ArticleCreateDraftResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## article_publish

> models::ArticlePublishResponse article_publish(article_id)
Publish Article

Publishes a draft Article, making it publicly visible.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**article_id** | **String** |  | [required] |

### Return type

[**models::ArticlePublishResponse**](ArticlePublishResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

