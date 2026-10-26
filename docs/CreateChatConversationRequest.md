# CreateChatConversationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action_signatures** | Option<[**Vec<models::CreateChatConversationActionSignatures>**](CreateChatConversationActionSignatures.md)> | Signatures for this action, forwarded in the order they are sent. | [optional]
**base64_encoded_key_rotation** | Option<**String**> | Base64-encoded key rotation payload. | [optional]
**conversation_id** | **String** | Client-generated conversation ID. | 
**conversation_key_version** | **String** | Version of the conversation encryption key. | 
**conversation_participant_keys** | [**Vec<models::CreateChatConversationConversationParticipantKeys>**](CreateChatConversationConversationParticipantKeys.md) | Encrypted conversation keys for each participant. | 
**group_admins** | Option<**Vec<String>**> | User IDs of group admins. Defaults to the creator if omitted. | [optional]
**group_avatar_url** | Option<**String**> | URL of the avatar image for the group conversation. | [optional]
**group_description** | Option<**String**> | Description for the group conversation. | [optional]
**group_members** | **Vec<String>** | User IDs of group members to include in the conversation. | 
**group_name** | Option<**String**> | Display name for the group conversation. | [optional]
**ttl_msec** | Option<**String**> | Message time-to-live in milliseconds. Messages expire after this duration. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


