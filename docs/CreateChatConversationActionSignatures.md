# CreateChatConversationActionSignatures

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**encoded_message_event_detail** | **String** | Base64-encoded Thrift MessageEventDetail for the signed action. | 
**message_event_signature** | [**models::CreateChatConversationActionSignaturesMessageEventSignature**](CreateChatConversationActionSignaturesMessageEventSignature.md) |  | 
**message_id** | **String** | Client-generated ID of the message being signed. | 
**signature_payload** | Option<**String**> | Payload string the client signed; used only in server-side failure logs. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


