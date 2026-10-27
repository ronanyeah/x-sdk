# RemoveChatGroupMembersGroupKeyRotationData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**base64_encoded_key_rotation** | Option<**String**> | Base64-encoded key rotation payload. | [optional]
**conversation_key_version** | Option<**String**> | Version of the new rotated conversation key. | [optional]
**conversation_participant_keys** | Option<[**Vec<models::RemoveChatGroupMembersGroupKeyRotationDataConversationParticipantKeys>**](RemoveChatGroupMembersGroupKeyRotationDataConversationParticipantKeys.md)> | Encrypted conversation keys for each participant. | [optional]
**encrypted_avatar** | Option<**String**> | Re-encrypted group avatar with new conversation key. | [optional]
**encrypted_description** | Option<**String**> | Re-encrypted group description with new conversation key. | [optional]
**encrypted_title** | Option<**String**> | Re-encrypted group title with new conversation key. | [optional]
**rotated_pmk_bundle** | Option<[**Vec<models::RemoveChatGroupMembersGroupKeyRotationDataRotatedPmkBundle>**](RemoveChatGroupMembersGroupKeyRotationDataRotatedPmkBundle.md)> | Message keys re-encrypted with the rotated conversation key. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


