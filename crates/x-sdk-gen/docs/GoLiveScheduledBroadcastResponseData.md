# GoLiveScheduledBroadcastResponseData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**available_for_replay** | Option<**bool**> | Whether replay is enabled. | [optional]
**broadcast_id** | Option<**String**> | Alphanumeric broadcast id (path `:id` for get/update/delete/live). | [optional]
**chat_option** | Option<**String**> | Who can chat (numeric string): `0` default, `1` chat off, `2` everyone, `3` verified accounts, `4` accounts the host follows, `5` the host's subscribers, `6` accounts followed by an account the host follows. | [optional]
**description** | Option<**String**> | Optional description. | [optional]
**locale** | Option<**String**> | Optional locale. | [optional]
**manual_publish** | Option<**bool**> | When true, coordinator will not auto-publish; call POST .../live when ready. | [optional]
**recurring_schedule_id** | Option<**String**> | Deprecated: always null for new schedules. | [optional]
**scheduled_broadcast_id** | Option<**String**> | Numeric scheduler id. Required in the update request body. | [optional]
**scheduled_end_ms** | Option<**String**> | Scheduled end, milliseconds since Unix epoch (decimal string). | [optional]
**scheduled_start_ms** | Option<**String**> | Scheduled start, milliseconds since Unix epoch (decimal string). | [optional]
**source_id** | Option<**String**> | Bound ingest / source id (`rtmp_stream_key`). | [optional]
**state** | Option<**String**> | Scheduler state: `Created` or `Scheduled` before start, then `Running`, `Ended`, or `Error`. | [optional]
**telecast_id** | Option<**String**> | Deprecated: always null for new schedules. | [optional]
**thumbnail_media_id** | Option<**String**> | Optional pre-live slate media id. | [optional]
**title** | Option<**String**> | Broadcast title / status text. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


