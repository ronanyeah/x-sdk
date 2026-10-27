# ChatMessageRequestDetail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**conversation_id** | Option<**String**> | The 1:1 conversation ID: both user IDs joined by a hyphen. | [optional]
**conversation_key_events** | Option<**Vec<String>**> | The conversation key change events needed to decrypt `message_events`. | [optional]
**has_more_events** | Option<**bool**> | Whether older message events may exist. Always `true` when `message_events` isn't empty. | [optional]
**message_events** | Option<**Vec<String>**> | The newest message events, newest first, as encoded message events. Their contents are end-to-end encrypted unless the conversation is unencrypted. | [optional]
**message_request_state** | Option<**String**> | `received` while the other user's request is pending, `sent` while the authenticated user's request is pending, `none` otherwise. | [optional]
**participant_id** | Option<**String**> | The other user in the conversation: the sender when `message_request_state` is `received`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


