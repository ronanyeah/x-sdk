# InitializeMediaUploadRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**additional_owners** | Option<**Vec<String>**> | User ids granted access to the uploaded media. | [optional]
**media_category** | Option<**MediaCategory**> | The media category of the upload. (enum: amplify_video, tweet_gif, tweet_image, tweet_video, dm_gif, dm_image, dm_video, subtitles) | [optional]
**media_type** | Option<**MediaType**> | The type of media. (enum: video/mp4, video/webm, video/mp2t, video/quicktime, text/srt, text/vtt, image/jpeg, image/gif, image/bmp, image/png, image/webp, image/pjpeg, image/tiff, model/gltf-binary, model/vnd.usdz+zip) | [optional]
**shared** | Option<**bool**> | Whether this media is shared or not. | [optional]
**total_bytes** | Option<**i32**> | The total size of the media upload in bytes. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


