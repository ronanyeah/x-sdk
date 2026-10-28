# ListUserBroadcastsResponseDataBroadcasts

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**available_for_replay** | Option<**bool**> | Whether a replay is available. | [optional]
**broadcast_id** | Option<**String**> | Broadcast id. | [optional]
**chat_option** | Option<**i32**> | Who can chat: 0 default, 1 chat off, 2 everyone, 3 verified accounts, 4 accounts the host follows, 5 the host's subscribers, 6 accounts followed by an account the host follows. | [optional]
**created_at_ms** | Option<**String**> | Creation time, milliseconds since Unix epoch (decimal string). | [optional]
**end_ms** | Option<**String**> | End time, milliseconds since Unix epoch (decimal string). | [optional]
**height** | Option<**i32**> | Video height in pixels. | [optional]
**id** | **String** | Broadcast id; equal to `broadcast_id`. | 
**image_url** | Option<**String**> | Thumbnail URL. | [optional]
**image_url_medium** | Option<**String**> | Medium thumbnail URL. | [optional]
**image_url_small** | Option<**String**> | Small thumbnail URL. | [optional]
**is_high_latency** | Option<**bool**> | Whether the broadcast is high latency. | [optional]
**language** | Option<**String**> | Broadcast language. | [optional]
**media_id** | Option<**String**> | Numeric media id from `media_key`. | [optional]
**media_key** | Option<**String**> | Media key of the broadcast's video. | [optional]
**scheduled_end_ms** | Option<**String**> | Scheduled end time, milliseconds since Unix epoch (decimal string). | [optional]
**scheduled_start_ms** | Option<**String**> | Scheduled start time, milliseconds since Unix epoch (decimal string). | [optional]
**start_ms** | Option<**String**> | Start time, milliseconds since Unix epoch (decimal string). | [optional]
**state** | Option<**String**> | `NOT_STARTED`, `RUNNING`, `TIMED_OUT`, `ENDED`, `PRE_PUBLISHED` or `CANCELED`. | [optional]
**status** | Option<**String**> | Broadcast title. | [optional]
**total_watched** | Option<**String**> | Total viewers (decimal string). | [optional]
**total_watching** | Option<**String**> | Current viewers (decimal string). | [optional]
**tweet_id** | Option<**String**> | Id of the Post announcing the broadcast. | [optional]
**twitter_user_id** | Option<**String**> | Owner's user id. | [optional]
**updated_at_ms** | Option<**String**> | Last update time, milliseconds since Unix epoch (decimal string). | [optional]
**width** | Option<**i32**> | Video width in pixels. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


