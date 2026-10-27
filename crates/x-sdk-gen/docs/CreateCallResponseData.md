# CreateCallResponseData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**conversation_id** | Option<**String**> | XChat conversation the call belongs to, if any. | [optional]
**created_at** | Option<**String**> | Creation time (RFC 3339). | [optional]
**host_id** | Option<**String**> | X user id of the call's host. | [optional]
**id** | **String** | Unique identifier of the call; also its join secret. | 
**join_policy** | Option<**String**> | Who may join: open (anyone with the link), closed (the host admits people), or trusted (the host's Verified Organization). | [optional]
**join_url** | Option<**String**> | Link people open to join the call on X. | [optional]
**participant_count** | Option<**i32**> | People in the call right now. | [optional]
**title** | Option<**String**> | Title of the call. | [optional]
**viewer_may_join** | Option<**bool**> | Whether the authenticated user may join without asking the host. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


