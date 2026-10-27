# AddChatGroupMembersRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action_signatures** | Option<[**Vec<models::AddChatGroupMembersActionSignatures>**](AddChatGroupMembersActionSignatures.md)> | Signatures for this action, forwarded in the order they are sent. | [optional]
**conversation_key_version** | Option<**String**> | Version of the new rotated conversation key. | [optional]
**conversation_participant_keys** | Option<[**Vec<models::AddChatGroupMembersConversationParticipantKeys>**](AddChatGroupMembersConversationParticipantKeys.md)> | Encrypted conversation keys for each participant. | [optional]
**encrypted_avatar_url** | Option<**String**> | Re-encrypted group avatar URL with new conversation key. | [optional]
**encrypted_title** | Option<**String**> | Re-encrypted group title with new conversation key. | [optional]
**user_ids** | **Vec<String>** | List of user IDs to add to the group conversation. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


