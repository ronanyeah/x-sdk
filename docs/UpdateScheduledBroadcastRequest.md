# UpdateScheduledBroadcastRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**available_for_replay** | Option<**bool**> | Enable replay. | [optional]
**chat_option** | Option<**String**> | Chat permission option (numeric string). | [optional]
**description** | Option<**String**> | Description. | [optional]
**is_locked** | Option<**bool**> | Lock the broadcast. | [optional]
**locale** | Option<**String**> | Locale. | [optional]
**manual_publish** | Option<**bool**> | If true, do not auto-publish at start; call POST .../live when ready. | [optional]
**roll_forward** | Option<**bool**> | When true, shift the head of a recurring series. | [optional]
**scheduled_broadcast_id** | **String** | Numeric scheduler id from create/list/get. | 
**scheduled_end_ms** | **String** | End time, ms since Unix epoch (decimal string). | 
**scheduled_start_ms** | **String** | Start time, ms since Unix epoch (decimal string). | 
**source_id** | Option<**String**> | Ingest / source id. | [optional]
**thumbnail_media_id** | Option<**String**> | Pre-live slate media id (numeric string). | [optional]
**title** | Option<**String**> | Title / status text. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


