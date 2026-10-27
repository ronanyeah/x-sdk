# EnableChatGroupInviteResponseData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**conversation_id** | Option<**String**> | ID of the group conversation the link joins. | [optional]
**expires_at** | Option<**String**> | When the link expires (ISO 8601). Absent when the link does not expire. | [optional]
**group_avatar_url** | Option<**String**> | Group avatar URL shown on the invite preview, stored unencrypted. Absent when the backend flags the avatar as NSFW. | [optional]
**group_name** | Option<**String**> | Group name shown on the invite preview, stored unencrypted. | [optional]
**invite_url** | Option<**String**> | Shareable invite link. | [optional]
**token** | Option<**String**> | Invite token. Together with the conversation ID it identifies the link. | [optional]
**welcome_message** | Option<**String**> | Message shown to people who open the link. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


