# CreateUsersBookmarkRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**folder_id** | Option<**String**> | Optional ID of the Bookmark folder to add the Post(s) to. When omitted, Posts are added to the user's top-level Bookmarks only. A bulk request uses this one folder for every ID. | [optional]
**tweet_id** | Option<**String**> | The ID of the Post to add to the user's Bookmarks. Mutually exclusive with `tweet_ids`. | [optional]
**tweet_ids** | Option<**Vec<String>**> | Post IDs to add to the user's Bookmarks. At most 25. Mutually exclusive with `tweet_id`. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


