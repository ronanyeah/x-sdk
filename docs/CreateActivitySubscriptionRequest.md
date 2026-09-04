# CreateActivitySubscriptionRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_type** | **EventType** | Activity event type in dot notation. `chat.conversation_join` is a deprecated alias of `chat.conversation.join`; responses always use the canonical name. (enum: profile.update.bio, profile.update.profile_picture, profile.update.banner_picture, profile.update.screenname, profile.update.geo, profile.update.url, profile.update.verified_badge, profile.update.affiliate_badge, profile.update.handle, news.new, follow.follow, follow.unfollow, spaces.start, spaces.end, broadcast.start, broadcast.end, broadcast.chat, chat.received, chat.sent, chat.conversation.join, chat.conversation_join, chat.conversation.member_added, chat.conversation.member_removed, chat.conversation.admin_added, chat.conversation.admin_removed, chat.update.group_name, chat.update.restrictions, dm.sent, dm.received, dm.indicate_typing, dm.read, post.create, post.delete, post.mention.create, post.reply.create, post.quote.create, post.repost.create, like.create, mute.mute, mute.unmute, block.block, block.unblock) | 
**filter** | [**models::CreateActivitySubscriptionFilter**](CreateActivitySubscriptionFilter.md) |  | 
**tag** | Option<**String**> | Optional caller-defined tag. | [optional]
**webhook_id** | Option<**String**> | Webhook to deliver the subscription events to. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


