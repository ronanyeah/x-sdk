# \CallsApi

All URIs are relative to *https://api.x.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_scheduled_call**](CallsApi.md#cancel_scheduled_call) | **DELETE** /2/calls/scheduled/{id} | Cancel a scheduled call
[**get_call**](CallsApi.md#get_call) | **GET** /2/calls/{id} | Get a call
[**get_scheduled_call**](CallsApi.md#get_scheduled_call) | **GET** /2/calls/scheduled/{id} | Get a scheduled call
[**list_scheduled_calls**](CallsApi.md#list_scheduled_calls) | **GET** /2/calls/scheduled | List scheduled calls
[**respond_to_scheduled_call**](CallsApi.md#respond_to_scheduled_call) | **POST** /2/calls/scheduled/{id}/rsvp | Respond to a scheduled call
[**schedule_call**](CallsApi.md#schedule_call) | **POST** /2/calls/scheduled | Schedule a call
[**update_scheduled_call**](CallsApi.md#update_scheduled_call) | **PUT** /2/calls/scheduled/{id} | Update a scheduled call



## cancel_scheduled_call

> models::CancelScheduledCallResponse cancel_scheduled_call(id, scope, occurrence_start_time, notify_invitees)
Cancel a scheduled call

Cancels a scheduled call the authenticated user organizes, and takes it off invitees' calendars. For a recurring call, scope and occurrence_start_time pick the occurrences: all of them (the default), one, or one and every one after it. Cancelling what is already cancelled succeeds.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**scope** | Option<**String**> | Which occurrences: all (the default without occurrence_start_time), this (the default with it), or this_and_following. |  |
**occurrence_start_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Start of the occurrence that scope this or this_and_following refers to (RFC 3339). |  |
**notify_invitees** | Option<**bool**> | Whether invitees get an email about it. Defaults to true. |  |

### Return type

[**models::CancelScheduledCallResponse**](CancelScheduledCallResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_call

> models::GetCallResponse get_call(id, call_fields, expansions, user_fields)
Get a call

Returns a call's title, join policy, host and participant count, and whether the authenticated user may join it without asking the host.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**call_fields** | Option<[**Vec<models::CallField>**](Models__CallField.md)> | A comma separated list of Call fields to display. |  |
**expansions** | Option<[**Vec<models::CallExpansion>**](Models__CallExpansion.md)> | A comma separated list of fields to expand. |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |

### Return type

[**models::GetCallResponse**](GetCallResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scheduled_call

> models::GetScheduledCallResponse get_scheduled_call(id, scheduled_call_fields, expansions, call_fields, user_fields)
Get a scheduled call

Returns a scheduled call the authenticated user organizes or is invited to, at its occurrence running now or its next one within a year: its time, call, recurrence and who is invited and how they answered. Changes can take a few seconds to show here. Use scheduled_call.fields to select which fields are returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**scheduled_call_fields** | Option<[**Vec<models::ScheduledCallField>**](Models__ScheduledCallField.md)> | A comma separated list of ScheduledCall fields to display. |  |
**expansions** | Option<[**Vec<models::ScheduledCallExpansion>**](Models__ScheduledCallExpansion.md)> | A comma separated list of fields to expand. |  |
**call_fields** | Option<[**Vec<models::CallField>**](Models__CallField.md)> | A comma separated list of Call fields to display. |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |

### Return type

[**models::GetScheduledCallResponse**](GetScheduledCallResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_scheduled_calls

> models::ListScheduledCallsResponse list_scheduled_calls(start_time, end_time, max_results, scheduled_call_fields, expansions, call_fields, user_fields)
List scheduled calls

Returns the X calls on the authenticated user's calendar, earliest first: events on their X Calls calendar, and events on the Microsoft Outlook calendars they linked to X whose conference is an X call. Calls still running at start_time are included. The window defaults to the next 24 hours and may span at most 7 days. Use scheduled_call.fields to select which fields are returned.  When both are provided, `start_time` must be earlier than `end_time`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**start_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Calls still running at this time or starting after it (RFC 3339). Defaults to now. |  |
**end_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Calls starting before this time (RFC 3339). Defaults to 24 hours after start_time. |  |
**max_results** | Option<**i32**> | The most calls to return. Defaults to 20. |  |
**scheduled_call_fields** | Option<[**Vec<models::ScheduledCallField>**](Models__ScheduledCallField.md)> | A comma separated list of ScheduledCall fields to display. |  |
**expansions** | Option<[**Vec<models::ScheduledCallExpansion>**](Models__ScheduledCallExpansion.md)> | A comma separated list of fields to expand. |  |
**call_fields** | Option<[**Vec<models::CallField>**](Models__CallField.md)> | A comma separated list of Call fields to display. |  |
**user_fields** | Option<[**Vec<models::UserField>**](Models__UserField.md)> | A comma separated list of User fields to display. |  |

### Return type

[**models::ListScheduledCallsResponse**](ListScheduledCallsResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## respond_to_scheduled_call

> models::RespondToScheduledCallResponse respond_to_scheduled_call(id, respond_to_scheduled_call_request)
Respond to a scheduled call

Accepts, declines or tentatively accepts an invitation to a scheduled call for the authenticated user. For a recurring call, scope and occurrence_start_time pick the occurrences the answer is for: all of them (the default), one, or one and every one after it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**respond_to_scheduled_call_request** | [**RespondToScheduledCallRequest**](RespondToScheduledCallRequest.md) |  | [required] |

### Return type

[**models::RespondToScheduledCallResponse**](RespondToScheduledCallResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## schedule_call

> models::ScheduleCallResponse schedule_call(schedule_call_request)
Schedule a call

Puts an X call on the authenticated user's X Calls calendar, with them as its organizer, and invites people to it. The call exists right away: share its join_url, or put an agent in it at start time with POST /2/calls/:id/join. A recurrence (an RFC 5545 RRULE) repeats it. Invitees get an email unless notify_invitees is false.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schedule_call_request** | [**ScheduleCallRequest**](ScheduleCallRequest.md) |  | [required] |

### Return type

[**models::ScheduleCallResponse**](ScheduleCallResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_scheduled_call

> models::UpdateScheduledCallResponse update_scheduled_call(id, update_scheduled_call_request)
Update a scheduled call

Moves, renames or re-invites a scheduled call the authenticated user organizes. Only the fields sent change. For a recurring call, scope and occurrence_start_time pick the occurrences: all of them (the default), one, or one and every one after it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**update_scheduled_call_request** | Option<[**UpdateScheduledCallRequest**](UpdateScheduledCallRequest.md)> |  | [required] |

### Return type

[**models::UpdateScheduledCallResponse**](UpdateScheduledCallResponse.md)

### Authorization

[OAuth2UserToken](../README.md#OAuth2UserToken), [UserToken](../README.md#UserToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

