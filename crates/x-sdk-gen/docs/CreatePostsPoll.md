# CreatePostsPoll

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**duration_minutes** | **i32** | Duration of the poll in minutes. | 
**options** | **Vec<String>** | Poll options (2-4 choices, 1-25 characters each). | 
**reply_settings** | Option<**ReplySettings**> | Who can reply to the poll Tweet. Accepted for compatibility; it carries no backend argument and is dropped before the mutation is issued (see `translate_body`). (enum: following, mentionedUsers, subscribers, verified) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


