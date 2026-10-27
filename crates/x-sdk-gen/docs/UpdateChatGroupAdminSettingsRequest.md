# UpdateChatGroupAdminSettingsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action_signatures** | Option<[**Vec<models::UpdateChatGroupAdminSettingsActionSignatures>**](UpdateChatGroupAdminSettingsActionSignatures.md)> | Signatures for this action, forwarded in the order they are sent. | [optional]
**add_member** | Option<**bool**> | When true, only admins can add members. | [optional]
**block_screen_capture** | Option<**bool**> | When true, only admins can change screen capture blocking. Through the API, only admins can change screen capture in a group, whatever this setting says. | [optional]
**edit_group_info** | Option<**bool**> | When true, only admins can edit the group name, avatar, and description. | [optional]
**edit_message_ttl** | Option<**bool**> | When true, only admins can change the disappearing-message timer. | [optional]
**send_message** | Option<**bool**> | When true, only admins can send messages. | [optional]
**start_call** | Option<**bool**> | When true, only admins can start calls. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


