# ActivityStreamResponseData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | The time the activity occurred, not when it was delivered. | [optional]
**event_type** | Option<**String**> |  | [optional]
**event_uuid** | Option<**String**> | The unique identifier of an Activity event. | [optional]
**filter** | Option<[**models::ActivitySubscriptionFilter**](ActivitySubscriptionFilter.md)> |  | [optional]
**includes** | Option<[**models::Expansions**](Expansions.md)> |  | [optional]
**payload** | Option<[**models::ActivityStreamingResponsePayload**](ActivityStreamingResponsePayload.md)> |  | [optional]
**tag** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


