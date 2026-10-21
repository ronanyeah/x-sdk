# UpdateChatGroupRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action_signatures** | Option<[**Vec<models::UpdateChatGroupActionSignatures>**](UpdateChatGroupActionSignatures.md)> | Cryptographic signatures for the action. | [optional]
**base64_encoded_key_rotation** | Option<**String**> | Base64-encoded key rotation payload. | [optional]
**conversation_key_version** | **String** | Version of the conversation key the values are encrypted with. | 
**conversation_participant_keys** | Option<[**Vec<models::UpdateChatGroupConversationParticipantKeys>**](UpdateChatGroupConversationParticipantKeys.md)> | Encrypted conversation keys for each participant. | [optional]
**group_avatar_url** | Option<**String**> | Encrypted group avatar URL. | [optional]
**group_description** | Option<**String**> | Encrypted group description. | [optional]
**group_name** | Option<**String**> | Encrypted group name. | [optional]
**modified_fields** | **Vec<ModifiedFields>** | The group values this request changes. (enum: group_name, group_avatar_url, group_description) | 
**rotated_pmk_bundle** | Option<[**Vec<models::UpdateChatGroupRotatedPmkBundle>**](UpdateChatGroupRotatedPmkBundle.md)> | Message keys re-encrypted with the rotated conversation key. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


