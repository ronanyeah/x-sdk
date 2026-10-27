# CreateScheduledBroadcastRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**available_for_replay** | Option<**bool**> | Enable replay. | [optional]
**chat_option** | Option<**String**> | Who can chat (numeric string): `0` default, `1` chat off, `2` everyone, `3` verified accounts, `4` accounts the host follows, `5` the host's subscribers, `6` accounts followed by an account the host follows. | [optional]
**description** | Option<**String**> | Description. | [optional]
**is_locked** | Option<**bool**> | Lock the broadcast. | [optional]
**locale** | Option<**String**> | Locale. | [optional]
**manual_publish** | Option<**bool**> | If true, do not auto-publish at start; call POST .../live when ready. | [optional]
**recurrence** | Option<[**models::CreateScheduledBroadcastRecurrence**](CreateScheduledBroadcastRecurrence.md)> |  | [optional]
**scheduled_end_ms** | Option<**String**> | Optional end time, ms since Unix epoch (decimal string). Omit for an open-ended broadcast. | [optional]
**scheduled_start_ms** | **String** | Start time, ms since Unix epoch (decimal string). | 
**source_id** | **String** | Ingest / source id to bind (same as sources `rtmp_stream_key`). | 
**telecast_id** | Option<**String**> | Deprecated: telecast associations are being retired and this field will be ignored. | [optional]
**thumbnail_media_id** | Option<**String**> | Pre-live slate media id (numeric string). | [optional]
**title** | Option<**String**> | Title / status text. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


