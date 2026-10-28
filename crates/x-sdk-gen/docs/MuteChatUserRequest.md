# MuteChatUserRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**broadcast_id** | Option<**String**> | A broadcast whose host's chats the user is muted in. Required with `message_id`. | [optional]
**end_at_ms** | Option<**String**> | The Unix timestamp in milliseconds when the mute expires. Omit this field to mute indefinitely. | [optional]
**message_id** | Option<**String**> | A chat message in `broadcast_id` to remove while muting the user. | [optional]
**user_id** | **String** | The user to mute. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


