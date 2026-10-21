# ChatMessageEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**conversation_id** | Option<**String**> |  | [optional]
**conversation_key_version** | Option<**String**> | The version of the conversation key that encrypts the event's contents. Absent when the event is not encrypted, such as a message sent before the conversation was encrypted. | [optional]
**conversation_token** | Option<**String**> | Usually present on encrypted events; unencrypted events have none. | [optional]
**created_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**encoded_event** | Option<**String**> |  | [optional]
**id** | Option<**String**> |  | [optional]
**is_trusted** | Option<**bool**> |  | [optional]
**message_event_signature** | Option<**serde_json::Value**> | Usually present on encrypted events; unencrypted events have none. Treat an encrypted event without one as unverified. | [optional]
**previous_id** | Option<**String**> |  | [optional]
**sender_id** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


