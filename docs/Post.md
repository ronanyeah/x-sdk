# Post

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**article** | Option<**serde_json::Value**> |  | [optional]
**article_title** | Option<**serde_json::Value**> | Metadata about the long-form Article attached to this Post, if any. | [optional]
**attachments** | Option<[**models::PostAttachments**](PostAttachments.md)> |  | [optional]
**author_id** | Option<**String**> | Unique identifier of the author of this Post. | [optional]
**card_uri** | Option<**String**> |  | [optional]
**community_id** | Option<**String**> | The unique identifier of the Community this Post belongs to, if any. | [optional]
**context_annotations** | Option<[**Vec<models::PostContextAnnotationsInner>**](PostContextAnnotationsInner.md)> | Annotations inferred about the Post (domain and entity context). | [optional]
**conversation_id** | Option<**String**> | The ID of the conversation this Post belongs to (matches the root Post's ID). | [optional]
**created_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Creation time of the Post. | [optional]
**display_text_range** | Option<**Vec<i64>**> | The inclusive start and exclusive end indices of the displayable content of the Post. | [optional]
**edit_controls** | Option<[**models::PostEditControls**](PostEditControls.md)> |  | [optional]
**edit_history_post_ids** | Option<**Vec<String>**> | A list of Post IDs in this Post's edit history chain. | [optional]
**entities** | Option<[**models::PostEntities**](PostEntities.md)> |  | [optional]
**geo** | Option<[**models::PostGeo**](PostGeo.md)> |  | [optional]
**id** | Option<**String**> | Unique identifier of this Post. | [optional]
**in_reply_to_user_id** | Option<**String**> | Unique identifier of the User this Post is replying to. | [optional]
**lang** | Option<**String**> | Language of the Post, if detected by X. Returned as a BCP47 language tag. | [optional]
**matched_media_notes** | Option<[**Vec<models::PostMatchedMediaNotesInner>**](PostMatchedMediaNotesInner.md)> | Community-Notes media matches for this Post. | [optional]
**media_metadata** | Option<[**Vec<models::PostMediaMetadataInner>**](PostMediaMetadataInner.md)> | Metadata for media attached to this Post. | [optional]
**non_public_metrics** | Option<**serde_json::Value**> | Nonpublic engagement metrics for the Post at the time of the request. | [optional]
**note_post** | Option<[**models::PostNotePost**](PostNotePost.md)> |  | [optional]
**note_request_suggestions** | Option<[**Vec<models::PostNoteRequestSuggestionsInner>**](PostNoteRequestSuggestionsInner.md)> | Community-Notes request suggestions for this Post. | [optional]
**organic_metrics** | Option<**serde_json::Value**> | Organic nonpublic engagement metrics for the Post at the time of the request. | [optional]
**paid_partnership** | Option<**bool**> | Indicates if this Post is a paid partnership, i.e. it has been disclosed by the author as containing paid promotion. | [optional]
**possibly_sensitive** | Option<**bool**> | Indicates if this Post contains URLs marked as sensitive, for example content suitable for mature audiences. | [optional]
**promoted_metrics** | Option<**serde_json::Value**> | Promoted nonpublic engagement metrics for the Post at the time of the request. | [optional]
**public_metrics** | Option<[**models::PostPublicMetrics**](PostPublicMetrics.md)> |  | [optional]
**referenced_posts** | Option<[**Vec<models::PostReferencedPostsInner>**](PostReferencedPostsInner.md)> | A list of Posts this Post refers to. If the Post is a Retweet, Quote or Reply, it includes the referenced Post's type and ID. | [optional]
**reply_settings** | Option<**String**> | Shows who can reply to this Post. | [optional]
**scopes** | Option<[**models::PostScopes**](PostScopes.md)> |  | [optional]
**source** | Option<**String**> | The name of the app the user posted from. This is deprecated. | [optional]
**suggested_source_links** | Option<**Vec<String>**> | URLs suggested as sources for this Post. | [optional]
**suggested_source_links_with_counts** | Option<[**Vec<models::PostSuggestedSourceLinksWithCountsInner>**](PostSuggestedSourceLinksWithCountsInner.md)> | Suggested source URLs for this Post, each with the number of times it was suggested. | [optional]
**text** | Option<**String**> | The content of the Post. | [optional]
**username** | Option<**String**> |  | [optional]
**withheld** | Option<[**models::PostWithheld**](PostWithheld.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


