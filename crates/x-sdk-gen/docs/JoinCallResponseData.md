# JoinCallResponseData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**admission** | Option<**String**> | immediate, or requires_approval when the host must admit the session. | [optional]
**call** | Option<[**models::JoinCallResponseDataCall**](JoinCallResponseDataCall.md)> |  | [optional]
**expires_at** | Option<**String**> | When media_token expires (RFC 3339). | [optional]
**media_token** | Option<**String**> | Bearer token for media_url; single use, short-lived. | [optional]
**media_type** | Option<**String**> | websocket. | [optional]
**media_url** | Option<**String**> | Where to open the media WebSocket. | [optional]
**session_id** | **String** | Media session id. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


