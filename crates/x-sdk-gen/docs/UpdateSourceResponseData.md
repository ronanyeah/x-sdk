# UpdateSourceResponseData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at_ms** | Option<**String**> | Creation time, milliseconds since Unix epoch (decimal string). | [optional]
**id** | **String** | Source id; the `source_id` a scheduled broadcast binds to. Currently equal to `stream_key`. | 
**is_stream_active** | Option<**bool**> | Whether an encoder is currently sending video to the source. Publishing a broadcast on the source needs it. | [optional]
**name** | Option<**String**> | Label for the source. | [optional]
**region** | Option<**String**> | Ingest region: `atla` or `pdxa` for on-prem sources, the AWS region (e.g. `us-east-1`) for sources created before the move to on-prem. | [optional]
**rtmp_url** | Option<**String**> | RTMP ingest URL. | [optional]
**rtmps_url** | Option<**String**> | RTMPS ingest URL (port 443). | [optional]
**stream_key** | Option<**String**> | RTMP stream key. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


