# UpdateChatConversationScreenCaptureRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**blocking** | Option<**bool**> | Whether participants are prevented from taking screenshots. | [optional]
**blocking_action_signatures** | Option<[**Vec<models::UpdateChatConversationScreenCaptureBlockingActionSignatures>**](UpdateChatConversationScreenCaptureBlockingActionSignatures.md)> | Cryptographic signatures for the screenshot blocking change. | [optional]
**detection** | Option<**bool**> | Whether participants are notified when someone takes a screenshot. | [optional]
**detection_action_signatures** | Option<[**Vec<models::UpdateChatConversationScreenCaptureDetectionActionSignatures>**](UpdateChatConversationScreenCaptureDetectionActionSignatures.md)> | Cryptographic signatures for the screenshot detection change. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


