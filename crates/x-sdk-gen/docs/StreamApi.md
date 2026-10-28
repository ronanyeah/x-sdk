# \StreamApi

All URIs are relative to *https://api.x.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**activity_stream**](StreamApi.md#activity_stream) | **GET** /2/activity/stream | Activity Stream
[**get_rule_counts**](StreamApi.md#get_rule_counts) | **GET** /2/tweets/search/stream/rules/counts | Get Rule Counts
[**get_rules**](StreamApi.md#get_rules) | **GET** /2/tweets/search/stream/rules | Get filtered-stream rules
[**stream_labels_compliance**](StreamApi.md#stream_labels_compliance) | **GET** /2/tweets/label/stream | Stream Post labels
[**stream_likes_compliance**](StreamApi.md#stream_likes_compliance) | **GET** /2/likes/compliance/stream | Stream Likes compliance data
[**stream_likes_firehose**](StreamApi.md#stream_likes_firehose) | **GET** /2/likes/firehose/stream | Stream all Likes
[**stream_likes_sample10**](StreamApi.md#stream_likes_sample10) | **GET** /2/likes/sample10/stream | Stream sampled Likes
[**stream_posts**](StreamApi.md#stream_posts) | **GET** /2/tweets/search/stream | Stream filtered Posts
[**stream_posts_compliance**](StreamApi.md#stream_posts_compliance) | **GET** /2/tweets/compliance/stream | Stream Posts compliance data
[**stream_posts_firehose**](StreamApi.md#stream_posts_firehose) | **GET** /2/tweets/firehose/stream | Stream all Posts
[**stream_posts_firehose_en**](StreamApi.md#stream_posts_firehose_en) | **GET** /2/tweets/firehose/stream/lang/en | Stream English Posts
[**stream_posts_firehose_ja**](StreamApi.md#stream_posts_firehose_ja) | **GET** /2/tweets/firehose/stream/lang/ja | Stream Japanese Posts
[**stream_posts_firehose_ko**](StreamApi.md#stream_posts_firehose_ko) | **GET** /2/tweets/firehose/stream/lang/ko | Stream Korean Posts
[**stream_posts_firehose_pt**](StreamApi.md#stream_posts_firehose_pt) | **GET** /2/tweets/firehose/stream/lang/pt | Stream Portuguese Posts
[**stream_posts_sample**](StreamApi.md#stream_posts_sample) | **GET** /2/tweets/sample/stream | Stream sampled Posts
[**stream_posts_sample10**](StreamApi.md#stream_posts_sample10) | **GET** /2/tweets/sample10/stream | Stream 10% sampled Posts
[**stream_users_compliance**](StreamApi.md#stream_users_compliance) | **GET** /2/users/compliance/stream | Stream Users compliance data
[**update_rules**](StreamApi.md#update_rules) | **POST** /2/tweets/search/stream/rules | Update stream rules



## activity_stream

> std::path::PathBuf activity_stream(backfill_minutes, start_time, end_time)
Activity Stream

Stream of X Activities

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**backfill_minutes** | Option<**i32**> | The number of minutes of backfill requested. |  |
**start_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | YYYY-MM-DDTHH:mm:ssZ. The earliest UTC timestamp from which the activities will be provided. |  |
**end_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | YYYY-MM-DDTHH:mm:ssZ. The latest UTC timestamp from which the activities will be provided. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rule_counts

> models::GetRuleCountsResponse get_rule_counts(rules_count_fields)
Get Rule Counts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rules_count_fields** | Option<**String**> |  |  |

### Return type

[**models::GetRuleCountsResponse**](GetRuleCountsResponse.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rules

> models::GetRulesResponse get_rules(ids, max_results, pagination_token)
Get filtered-stream rules

Returns the active filtered-stream rules for the authenticated app. Provide `ids` to fetch specific rules; omit it to list all rules.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | Option<[**Vec<String>**](String.md)> |  |  |
**max_results** | Option<**i32**> |  |  |[default to 1000]
**pagination_token** | Option<**String**> | A base32hex-encoded pagination token that decodes to 8 bytes. |  |

### Return type

[**models::GetRulesResponse**](GetRulesResponse.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stream_labels_compliance

> std::path::PathBuf stream_labels_compliance(backfill_minutes, start_time, end_time)
Stream Post labels

Streams all labeling events applied to Posts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**backfill_minutes** | Option<**i32**> | The number of minutes of backfill requested. |  |
**start_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | YYYY-MM-DDTHH:mm:ssZ. The earliest UTC timestamp from which the Post labels will be provided. |  |
**end_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | YYYY-MM-DDTHH:mm:ssZ. The latest UTC timestamp from which the Post labels will be provided. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stream_likes_compliance

> std::path::PathBuf stream_likes_compliance(backfill_minutes, start_time, end_time)
Stream Likes compliance data

Streams all compliance data related to Likes for Users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**backfill_minutes** | Option<**i32**> | The number of minutes of backfill requested. |  |
**start_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | YYYY-MM-DDTHH:mm:ssZ. The earliest UTC timestamp from which the Likes Compliance events will be provided. |  |
**end_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | YYYY-MM-DDTHH:mm:ssZ. The latest UTC timestamp from which the Likes Compliance events will be provided. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stream_likes_firehose

> std::path::PathBuf stream_likes_firehose(partition, backfill_minutes, start_time, end_time, like_with_tweet_author_fields, expansions, media_fields, user_fields, tweet_fields)
Stream all Likes

Streams all public Likes in real-time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**partition** | **i32** | The partition number. | [required] |
**backfill_minutes** | Option<**i32**> | The number of minutes of backfill requested. |  |
**start_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | YYYY-MM-DDTHH:mm:ssZ. The earliest UTC timestamp to which the Likes will be provided. |  |
**end_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | YYYY-MM-DDTHH:mm:ssZ. The latest UTC timestamp to which the Posts will be provided. |  |
**like_with_tweet_author_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of LikeWithTweetAuthor fields to display. |  |
**expansions** | Option<[**Vec<String>**](String.md)> | A comma separated list of fields to expand. |  |
**media_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Media fields to display. |  |
**user_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of User fields to display. |  |
**tweet_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Tweet fields to display. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stream_likes_sample10

> std::path::PathBuf stream_likes_sample10(partition, backfill_minutes, start_time, end_time, like_with_tweet_author_fields, expansions, media_fields, user_fields, tweet_fields)
Stream sampled Likes

Streams a 10% sample of public Likes in real-time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**partition** | **i32** | The partition number. | [required] |
**backfill_minutes** | Option<**i32**> | The number of minutes of backfill requested. |  |
**start_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | YYYY-MM-DDTHH:mm:ssZ. The earliest UTC timestamp to which the Likes will be provided. |  |
**end_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | YYYY-MM-DDTHH:mm:ssZ. The latest UTC timestamp to which the Posts will be provided. |  |
**like_with_tweet_author_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of LikeWithTweetAuthor fields to display. |  |
**expansions** | Option<[**Vec<String>**](String.md)> | A comma separated list of fields to expand. |  |
**media_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Media fields to display. |  |
**user_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of User fields to display. |  |
**tweet_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Tweet fields to display. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stream_posts

> std::path::PathBuf stream_posts(backfill_minutes, start_time, end_time, tweet_fields, expansions, media_fields, poll_fields, user_fields, place_fields)
Stream filtered Posts

Streams Posts in real-time matching the active rule set.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**backfill_minutes** | Option<**i32**> | The number of minutes of backfill requested. |  |
**start_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | YYYY-MM-DDTHH:mm:ssZ. The earliest UTC timestamp to which the Posts will be provided. |  |
**end_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | YYYY-MM-DDTHH:mm:ssZ. The latest UTC timestamp to which the Posts will be provided. |  |
**tweet_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Tweet fields to display. |  |
**expansions** | Option<[**Vec<String>**](String.md)> | A comma separated list of fields to expand. |  |
**media_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Media fields to display. |  |
**poll_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Poll fields to display. |  |
**user_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of User fields to display. |  |
**place_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Place fields to display. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stream_posts_compliance

> std::path::PathBuf stream_posts_compliance(partition, backfill_minutes, start_time, end_time)
Stream Posts compliance data

Streams all compliance data related to Posts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**partition** | **i32** | The partition number. | [required] |
**backfill_minutes** | Option<**i32**> | The number of minutes of backfill requested. |  |
**start_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | YYYY-MM-DDTHH:mm:ssZ. The earliest UTC timestamp from which the Post Compliance events will be provided. |  |
**end_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | YYYY-MM-DDTHH:mm:ssZ. The latest UTC timestamp from which the Post Compliance events will be provided. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stream_posts_firehose

> std::path::PathBuf stream_posts_firehose(partition, backfill_minutes, tweet_fields, expansions, media_fields, poll_fields, user_fields, place_fields, start_time, end_time)
Stream all Posts

Streams all public Posts in real-time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**partition** | **i32** | The partition number. | [required] |
**backfill_minutes** | Option<**i32**> | The number of minutes of backfill requested. |  |
**tweet_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Tweet fields to display. |  |
**expansions** | Option<[**Vec<String>**](String.md)> | A comma separated list of fields to expand. |  |
**media_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Media fields to display. |  |
**poll_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Poll fields to display. |  |
**user_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of User fields to display. |  |
**place_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Place fields to display. |  |
**start_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | YYYY-MM-DDTHH:mm:ssZ. The earliest UTC timestamp to which the Posts will be provided. |  |
**end_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | YYYY-MM-DDTHH:mm:ssZ. The latest UTC timestamp to which the Posts will be provided. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stream_posts_firehose_en

> std::path::PathBuf stream_posts_firehose_en(partition, backfill_minutes, tweet_fields, expansions, media_fields, poll_fields, user_fields, place_fields, start_time, end_time)
Stream English Posts

Streams all public English-language Posts in real-time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**partition** | **i32** | The partition number. | [required] |
**backfill_minutes** | Option<**i32**> | The number of minutes of backfill requested. |  |
**tweet_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Tweet fields to display. |  |
**expansions** | Option<[**Vec<String>**](String.md)> | A comma separated list of fields to expand. |  |
**media_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Media fields to display. |  |
**poll_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Poll fields to display. |  |
**user_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of User fields to display. |  |
**place_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Place fields to display. |  |
**start_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | YYYY-MM-DDTHH:mm:ssZ. The earliest UTC timestamp to which the Posts will be provided. |  |
**end_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | YYYY-MM-DDTHH:mm:ssZ. The latest UTC timestamp to which the Posts will be provided. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stream_posts_firehose_ja

> std::path::PathBuf stream_posts_firehose_ja(partition, backfill_minutes, tweet_fields, expansions, media_fields, poll_fields, user_fields, place_fields, start_time, end_time)
Stream Japanese Posts

Streams all public Japanese-language Posts in real-time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**partition** | **i32** | The partition number. | [required] |
**backfill_minutes** | Option<**i32**> | The number of minutes of backfill requested. |  |
**tweet_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Tweet fields to display. |  |
**expansions** | Option<[**Vec<String>**](String.md)> | A comma separated list of fields to expand. |  |
**media_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Media fields to display. |  |
**poll_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Poll fields to display. |  |
**user_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of User fields to display. |  |
**place_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Place fields to display. |  |
**start_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | YYYY-MM-DDTHH:mm:ssZ. The earliest UTC timestamp to which the Posts will be provided. |  |
**end_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | YYYY-MM-DDTHH:mm:ssZ. The latest UTC timestamp to which the Posts will be provided. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stream_posts_firehose_ko

> std::path::PathBuf stream_posts_firehose_ko(partition, backfill_minutes, tweet_fields, expansions, media_fields, poll_fields, user_fields, place_fields, start_time, end_time)
Stream Korean Posts

Streams all public Korean-language Posts in real-time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**partition** | **i32** | The partition number. | [required] |
**backfill_minutes** | Option<**i32**> | The number of minutes of backfill requested. |  |
**tweet_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Tweet fields to display. |  |
**expansions** | Option<[**Vec<String>**](String.md)> | A comma separated list of fields to expand. |  |
**media_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Media fields to display. |  |
**poll_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Poll fields to display. |  |
**user_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of User fields to display. |  |
**place_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Place fields to display. |  |
**start_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | YYYY-MM-DDTHH:mm:ssZ. The earliest UTC timestamp to which the Posts will be provided. |  |
**end_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | YYYY-MM-DDTHH:mm:ssZ. The latest UTC timestamp to which the Posts will be provided. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stream_posts_firehose_pt

> std::path::PathBuf stream_posts_firehose_pt(partition, backfill_minutes, tweet_fields, expansions, media_fields, poll_fields, user_fields, place_fields, start_time, end_time)
Stream Portuguese Posts

Streams all public Portuguese-language Posts in real-time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**partition** | **i32** | The partition number. | [required] |
**backfill_minutes** | Option<**i32**> | The number of minutes of backfill requested. |  |
**tweet_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Tweet fields to display. |  |
**expansions** | Option<[**Vec<String>**](String.md)> | A comma separated list of fields to expand. |  |
**media_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Media fields to display. |  |
**poll_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Poll fields to display. |  |
**user_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of User fields to display. |  |
**place_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Place fields to display. |  |
**start_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | YYYY-MM-DDTHH:mm:ssZ. The earliest UTC timestamp to which the Posts will be provided. |  |
**end_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | YYYY-MM-DDTHH:mm:ssZ. The latest UTC timestamp to which the Posts will be provided. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stream_posts_sample

> std::path::PathBuf stream_posts_sample(backfill_minutes, tweet_fields, expansions, media_fields, poll_fields, user_fields, place_fields)
Stream sampled Posts

Streams a 1% sample of public Posts in real-time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**backfill_minutes** | Option<**i32**> | The number of minutes of backfill requested. |  |
**tweet_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Tweet fields to display. |  |
**expansions** | Option<[**Vec<String>**](String.md)> | A comma separated list of fields to expand. |  |
**media_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Media fields to display. |  |
**poll_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Poll fields to display. |  |
**user_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of User fields to display. |  |
**place_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Place fields to display. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stream_posts_sample10

> std::path::PathBuf stream_posts_sample10(partition, backfill_minutes, tweet_fields, expansions, media_fields, poll_fields, user_fields, place_fields, start_time, end_time)
Stream 10% sampled Posts

Streams a 10% sample of public Posts in real-time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**partition** | **i32** | The partition number. | [required] |
**backfill_minutes** | Option<**i32**> | The number of minutes of backfill requested. |  |
**tweet_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Tweet fields to display. |  |
**expansions** | Option<[**Vec<String>**](String.md)> | A comma separated list of fields to expand. |  |
**media_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Media fields to display. |  |
**poll_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Poll fields to display. |  |
**user_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of User fields to display. |  |
**place_fields** | Option<[**Vec<String>**](String.md)> | A comma separated list of Place fields to display. |  |
**start_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | YYYY-MM-DDTHH:mm:ssZ. The earliest UTC timestamp to which the Posts will be provided. |  |
**end_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | YYYY-MM-DDTHH:mm:ssZ. The latest UTC timestamp to which the Posts will be provided. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stream_users_compliance

> std::path::PathBuf stream_users_compliance(partition, backfill_minutes, start_time, end_time)
Stream Users compliance data

Streams all compliance data related to Users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**partition** | **i32** | The partition number. | [required] |
**backfill_minutes** | Option<**i32**> | The number of minutes of backfill requested. |  |
**start_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | YYYY-MM-DDTHH:mm:ssZ. The earliest UTC timestamp from which the User Compliance events will be provided. |  |
**end_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | YYYY-MM-DDTHH:mm:ssZ. The latest UTC timestamp from which the User Compliance events will be provided. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_rules

> models::UpdateRulesResponse update_rules(update_rules_request, dry_run, delete_all)
Update stream rules

Adds or deletes rules from the active rule set for the filtered stream. Exactly one of `add`, `delete`, or `?delete_all=true` must be specified. Use `?dry_run=true` to validate without committing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_rules_request** | [**UpdateRulesRequest**](UpdateRulesRequest.md) |  | [required] |
**dry_run** | Option<**bool**> |  |  |
**delete_all** | Option<**bool**> |  |  |

### Return type

[**models::UpdateRulesResponse**](UpdateRulesResponse.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

