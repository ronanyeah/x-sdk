# ChatConversation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**admin_ids** | Option<**Vec<String>**> |  | [optional]
**admin_settings** | Option<[**models::ChatConversationAdminSettings**](ChatConversationAdminSettings.md)> |  | [optional]
**created_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**group_avatar_url** | Option<**String**> |  | [optional]
**group_description** | Option<**String**> | The group description, encrypted with the conversation key like group_name. | [optional]
**group_description_key_version** | Option<**String**> | The conversation key version that group_description is encrypted with. | [optional]
**group_name** | Option<**String**> |  | [optional]
**id** | Option<**String**> |  | [optional]
**invite** | Option<[**models::ChatConversationInvite**](ChatConversationInvite.md)> |  | [optional]
**is_archived** | Option<**bool**> | Whether the authenticated user archived this conversation. | [optional]
**is_muted** | Option<**bool**> |  | [optional]
**last_read_event_id** | Option<**String**> | The event ID you have read up to: the seen_until_sequence_id of your latest read receipt. Absent when you have no read receipt. | [optional]
**member_ids** | Option<**Vec<String>**> |  | [optional]
**message_ttl_ms** | Option<**i32**> |  | [optional]
**newest_event_id** | Option<**String**> | The ID of the latest notifying message in the conversation. | [optional]
**participant_ids** | Option<**Vec<String>**> |  | [optional]
**pending_member_ids** | Option<**Vec<String>**> | Unique identifiers of Users who asked to join this group and are awaiting approval. Returned only to callers who can add members. | [optional]
**screen_capture_blocking_enabled** | Option<**bool**> |  | [optional]
**screen_capture_detection_enabled** | Option<**bool**> |  | [optional]
**r#type** | Option<**String**> |  | [optional]
**unread** | Option<**bool**> | Whether the conversation is unread for you: its latest notifying message was sent by someone else after last_read_event_id, or you have no read receipt. Marking the conversation unread does not set it. | [optional]
**updated_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


