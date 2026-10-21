# EnableChatGroupInviteRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action_signatures** | Option<[**Vec<models::EnableChatGroupInviteActionSignatures>**](EnableChatGroupInviteActionSignatures.md)> | Cryptographic signatures for the action. | [optional]
**affiliate_user_id** | Option<**String**> | ID of the affiliate account to associate with the link. | [optional]
**decrypted_avatar_url** | Option<**String**> | Group avatar URL (https) to show on the invite preview. | [optional]
**decrypted_title** | Option<**String**> | Group name to show on the invite preview. | [optional]
**invite_url** | Option<**String**> | Invite link built by the client, in the form https://x.com/i/chat/group_join/<conversation_id>/<token>, where the token is exactly 10 letters, digits, '-' or '_'. The conversation segment must match the path ID. It is part of the signed payload, so it is required when action_signatures is sent. | [optional]
**validity_duration_msec** | Option<**String**> | How long the link stays valid, in milliseconds, from the time of the request. With action_signatures, send validity_expiry_timestamp_msec instead. | [optional]
**validity_expiry_timestamp_msec** | Option<**String**> | When the link expires, in epoch milliseconds. Must be in the future. If validity_duration_msec is also sent, the two must agree within a minute. | [optional]
**welcome_message** | Option<**String**> | Message shown to people who open the link. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


