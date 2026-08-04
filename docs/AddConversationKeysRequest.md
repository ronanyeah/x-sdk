# AddConversationKeysRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action_signatures** | Option<[**Vec<models::AddConversationKeysActionSignatures>**](AddConversationKeysActionSignatures.md)> | Cryptographic signatures for the action. | [optional]
**base64_encoded_key_rotation** | Option<**String**> | Base64-encoded key rotation payload for ratchet tree key management. | [optional]
**conversation_key_version** | **String** | Version of the conversation encryption key (typically a timestamp in milliseconds). | 
**conversation_participant_keys** | [**Vec<models::AddConversationKeysConversationParticipantKeys>**](AddConversationKeysConversationParticipantKeys.md) | Encrypted conversation keys for each participant. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


