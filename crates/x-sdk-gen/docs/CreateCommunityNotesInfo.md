# CreateCommunityNotesInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**classification** | **Classification** | Community Note classification. (enum: misinformed_or_potentially_misleading, not_misleading) | 
**is_media_note** | Option<**bool**> | Whether the note is a media note. | [optional]
**misleading_tags** | **Vec<MisleadingTags>** | Tags describing why the Post is misleading. (enum: disputed_claim_as_fact, factual_error, manipulated_media, misinterpreted_satire, missing_important_context, other, outdated_information) | 
**text** | **String** | The text summary of the Community Note (must contain a URL). | 
**trustworthy_sources** | **bool** | Whether the note cites trustworthy sources. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


