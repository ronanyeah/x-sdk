# CreatePostsReply

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**auto_populate_reply_metadata** | Option<**bool**> | If true, reply metadata is automatically populated. Accepted for compatibility; it carries no backend argument and is dropped before the mutation is issued (see `translate_body`). | [optional]
**exclude_reply_user_ids** | Option<**Vec<String>**> | User IDs to exclude from the reply thread. | [optional]
**in_reply_to_tweet_id** | **String** | The ID of the tweet being replied to. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


