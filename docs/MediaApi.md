# \MediaApi

All URIs are relative to *https://api.x.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**append_media_upload**](MediaApi.md#append_media_upload) | **POST** /2/media/upload/{id}/append | Append Media Upload
[**create_media_metadata**](MediaApi.md#create_media_metadata) | **POST** /2/media/metadata | Create Media metadata
[**create_media_subtitles**](MediaApi.md#create_media_subtitles) | **POST** /2/media/subtitles | Create Media subtitles
[**delete_media_subtitles**](MediaApi.md#delete_media_subtitles) | **DELETE** /2/media/subtitles | Delete Media subtitles
[**finalize_media_upload**](MediaApi.md#finalize_media_upload) | **POST** /2/media/upload/{id}/finalize | Finalize Media upload
[**get_media_analytics**](MediaApi.md#get_media_analytics) | **GET** /2/media/analytics | Get Media analytics
[**get_media_by_media_key**](MediaApi.md#get_media_by_media_key) | **GET** /2/media/{media_key} | Get Media by media key
[**get_media_by_media_keys**](MediaApi.md#get_media_by_media_keys) | **GET** /2/media | Get Media by media keys
[**get_media_upload_status**](MediaApi.md#get_media_upload_status) | **GET** /2/media/upload | Get Media upload status
[**initialize_media_upload**](MediaApi.md#initialize_media_upload) | **POST** /2/media/upload/initialize | Initialize media upload
[**media_upload**](MediaApi.md#media_upload) | **POST** /2/media/upload | Upload media



## append_media_upload

> models::AppendMediaUploadResponse append_media_upload(id, append_media_upload_request)
Append Media Upload

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**append_media_upload_request** | [**AppendMediaUploadRequest**](AppendMediaUploadRequest.md) |  | [required] |

### Return type

[**models::AppendMediaUploadResponse**](AppendMediaUploadResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_media_metadata

> models::CreateMediaMetadataResponse create_media_metadata(create_media_metadata_request)
Create Media metadata

Creates metadata for a Media file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_media_metadata_request** | [**CreateMediaMetadataRequest**](CreateMediaMetadataRequest.md) |  | [required] |

### Return type

[**models::CreateMediaMetadataResponse**](CreateMediaMetadataResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_media_subtitles

> models::CreateMediaSubtitlesResponse create_media_subtitles(create_media_subtitles_request)
Create Media subtitles

Creates subtitles for a specific Media file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_media_subtitles_request** | [**CreateMediaSubtitlesRequest**](CreateMediaSubtitlesRequest.md) |  | [required] |

### Return type

[**models::CreateMediaSubtitlesResponse**](CreateMediaSubtitlesResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_media_subtitles

> models::DeleteMediaSubtitlesResponse delete_media_subtitles(delete_media_subtitles_request)
Delete Media subtitles

Deletes subtitles for a specific Media file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**delete_media_subtitles_request** | [**DeleteMediaSubtitlesRequest**](DeleteMediaSubtitlesRequest.md) |  | [required] |

### Return type

[**models::DeleteMediaSubtitlesResponse**](DeleteMediaSubtitlesResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## finalize_media_upload

> models::FinalizeMediaUploadResponse finalize_media_upload(id)
Finalize Media upload

Finalizes a Media upload request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::FinalizeMediaUploadResponse**](FinalizeMediaUploadResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_media_analytics

> models::GetMediaAnalyticsResponse get_media_analytics(media_keys, start_time, end_time, granularity, media_analytics_fields)
Get Media analytics

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**media_keys** | [**Vec<String>**](String.md) |  | [required] |
**start_time** | **chrono::DateTime<chrono::FixedOffset>** |  | [required] |
**end_time** | **chrono::DateTime<chrono::FixedOffset>** |  | [required] |
**granularity** | Option<**String**> |  |  |[default to daily]
**media_analytics_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of MediaAnalytics fields to display. |  |

### Return type

[**models::GetMediaAnalyticsResponse**](GetMediaAnalyticsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_media_by_media_key

> models::GetMediaByMediaKeyResponse get_media_by_media_key(media_key, media_fields)
Get Media by media key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**media_key** | **String** |  | [required] |
**media_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Media fields to display. |  |

### Return type

[**models::GetMediaByMediaKeyResponse**](GetMediaByMediaKeyResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_media_by_media_keys

> models::GetMediaByMediaKeysResponse get_media_by_media_keys(media_keys, media_fields)
Get Media by media keys

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**media_keys** | [**Vec<String>**](String.md) |  | [required] |
**media_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Media fields to display. |  |

### Return type

[**models::GetMediaByMediaKeysResponse**](GetMediaByMediaKeysResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [BearerToken](../README.md#BearerToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_media_upload_status

> models::GetMediaUploadStatusResponse get_media_upload_status(media_id, command)
Get Media upload status

Retrieves the status of a Media upload by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**media_id** | **String** |  | [required] |
**command** | Option<**String**> |  |  |

### Return type

[**models::GetMediaUploadStatusResponse**](GetMediaUploadStatusResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## initialize_media_upload

> models::InitializeMediaUploadResponse initialize_media_upload(initialize_media_upload_request)
Initialize media upload

Initializes a media upload.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**initialize_media_upload_request** | [**InitializeMediaUploadRequest**](InitializeMediaUploadRequest.md) |  | [required] |

### Return type

[**models::InitializeMediaUploadResponse**](InitializeMediaUploadResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## media_upload

> models::MediaUploadResponse media_upload(media_upload_request)
Upload media

Uploads a media file for use in posts or other content.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**media_upload_request** | [**MediaUploadRequest**](MediaUploadRequest.md) |  | [required] |

### Return type

[**models::MediaUploadResponse**](MediaUploadResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json, multipart/form-data
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

