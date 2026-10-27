# GetSourceResponseDataSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**creation_time** | Option<**i32**> | Creation time, ms since Unix epoch. | [optional]
**id** | **String** | Source id; also the RTMP stream key. | 
**is_stream_active** | Option<**bool**> | Whether the source is currently receiving video. | [optional]
**name** | Option<**String**> | Label for the source. | [optional]
**owner_id** | Option<**i32**> | Owner's numeric user id. Exceeds the exact integer range of many JSON parsers. | [optional]
**rtmp_region** | Option<**String**> | Ingest region, e.g. `us-east-1`. | [optional]
**rtmp_stream_key** | Option<**String**> | Stream key; equal to `id`. | [optional]
**rtmp_url** | Option<**String**> | RTMP ingest URL. | [optional]
**rtmps_url** | Option<**String**> | RTMPS ingest URL (port 443). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


