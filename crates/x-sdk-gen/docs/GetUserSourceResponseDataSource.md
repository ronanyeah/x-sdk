# GetUserSourceResponseDataSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**creation_origin** | Option<[**models::GetUserSourceResponseDataSourceCreationOrigin**](GetUserSourceResponseDataSourceCreationOrigin.md)> |  | [optional]
**creation_time** | Option<**i32**> | Creation time, ms since Unix epoch. | [optional]
**id** | **String** | Source id; also the RTMP stream key. | 
**is360** | Option<**bool**> | Whether the source streams 360° video. | [optional]
**is_stream_active** | Option<**bool**> | Whether an encoder is currently sending video to the source. | [optional]
**name** | Option<**String**> | Label for the source. | [optional]
**owner_id** | Option<**i32**> | Owner's numeric user id. Exceeds the exact integer range of many JSON parsers. | [optional]
**rtmp_region** | Option<**String**> | Ingest region: `atla` or `pdxa` for on-prem sources, the AWS region (e.g. `us-east-1`) for sources created before the move to on-prem. | [optional]
**rtmp_stream_key** | Option<**String**> | Stream key; equal to `id`. | [optional]
**rtmp_url** | Option<**String**> | RTMP ingest URL. | [optional]
**rtmps_url** | Option<**String**> | RTMPS ingest URL (port 443). | [optional]
**video_service** | Option<[**models::GetUserSourceResponseDataSourceVideoService**](GetUserSourceResponseDataSourceVideoService.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


