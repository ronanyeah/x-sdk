# ChatMessageRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**conversation_id** | Option<**String**> | The 1:1 conversation ID: both user IDs joined by a hyphen. | [optional]
**conversation_key_events** | Option<**Vec<String>**> | The conversation key change events needed to decrypt `message_events`. | [optional]
**encoded_message_create_event** | Option<**String**> | The message that previews the request, as an encoded message event. Its contents are end-to-end encrypted unless the conversation is unencrypted. | [optional]
**has_more_events** | Option<**bool**> | Whether the conversation has older message events than `message_events`. | [optional]
**message_events** | Option<**Vec<String>**> | The conversation's latest message events, newest first, as encoded message events. Their contents are end-to-end encrypted unless the conversation is unencrypted. | [optional]
**sender_id** | Option<**String**> | The user ID of the user who sent the request. | [optional]
**sender_recently_modified_profile** | Option<**bool**> | Whether the sender recently changed their name or avatar, a possible sign of impersonation. | [optional]
**social_proof_user_ids** | Option<**Vec<String>**> | IDs of users the authenticated user follows who also follow the sender. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


