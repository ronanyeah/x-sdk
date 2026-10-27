# CreatePostsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**card_uri** | Option<**String**> | Card URI parameter. | [optional]
**community_id** | Option<**String**> | Community to post the tweet to. | [optional]
**direct_message_deep_link** | Option<**String**> | Direct message deep link. | [optional]
**edit_options** | Option<[**models::CreatePostsEditOptions**](CreatePostsEditOptions.md)> |  | [optional]
**for_super_followers_only** | Option<**bool**> | Restrict tweet to super followers. | [optional]
**geo** | Option<[**models::CreatePostsGeo**](CreatePostsGeo.md)> |  | [optional]
**made_with_ai** | Option<**bool**> | Disclose that the tweet contains AI-generated media. | [optional]
**media** | Option<[**models::CreatePostsMedia**](CreatePostsMedia.md)> |  | [optional]
**nullcast** | Option<**bool**> | If true, the tweet is not shown in the public timeline. | [optional]
**paid_partnership** | Option<**bool**> | Disclose that the tweet is a paid partnership. | [optional]
**poll** | Option<[**models::CreatePostsPoll**](CreatePostsPoll.md)> |  | [optional]
**quote_tweet_id** | Option<**String**> | Tweet ID to quote. | [optional]
**reply** | Option<[**models::CreatePostsReply**](CreatePostsReply.md)> |  | [optional]
**reply_settings** | Option<**ReplySettings**> | Who can reply to this tweet. (enum: following, mentionedUsers, subscribers, verified) | [optional]
**share_with_followers** | Option<**bool**> | Share an exclusive (super-follower) tweet with all followers. | [optional]
**text** | Option<**String**> | Text of the tweet. Required unless media is provided.  Defaulted to an empty string so it is always sent: the backend's `tweet_text` variable is non-null and rejects an absent value. | [optional][default to ]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


