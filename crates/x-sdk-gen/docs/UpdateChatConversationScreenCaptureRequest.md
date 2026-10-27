# UpdateChatConversationScreenCaptureRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**blocking** | Option<**bool**> | Whether participants are prevented from taking screenshots. | [optional]
**blocking_action_signatures** | Option<[**Vec<models::UpdateChatConversationScreenCaptureBlockingActionSignatures>**](UpdateChatConversationScreenCaptureBlockingActionSignatures.md)> | Signatures for the screenshot blocking change, forwarded in the order they are sent. | [optional]
**detection** | Option<**bool**> | Whether participants are notified when someone takes a screenshot. | [optional]
**detection_action_signatures** | Option<[**Vec<models::UpdateChatConversationScreenCaptureDetectionActionSignatures>**](UpdateChatConversationScreenCaptureDetectionActionSignatures.md)> | Signatures for the screenshot detection change, forwarded in the order they are sent. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


