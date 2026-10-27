# MediaUploadResponseData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**expires_after_secs** | Option<**i32**> | Seconds until the upload session expires. | [optional]
**id** | **String** | Unique identifier of the media. | 
**image** | Option<[**models::MediaUploadResponseDataImage**](MediaUploadResponseDataImage.md)> |  | [optional]
**media_key** | Option<**String**> | The media key for the uploaded media. | [optional]
**processing_info** | Option<[**models::MediaUploadResponseDataProcessingInfo**](MediaUploadResponseDataProcessingInfo.md)> |  | [optional]
**size** | Option<**i32**> | Total size of the media in bytes. | [optional]
**video** | Option<[**models::MediaUploadResponseDataVideo**](MediaUploadResponseDataVideo.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


