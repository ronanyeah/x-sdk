# ArticleCreateDraftContentStateBlocks

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**data** | Option<[**models::ArticleCreateDraftContentStateBlocksData**](ArticleCreateDraftContentStateBlocksData.md)> |  | [optional]
**entity_ranges** | Option<[**Vec<models::ArticleCreateDraftContentStateBlocksEntityRanges>**](ArticleCreateDraftContentStateBlocksEntityRanges.md)> | References to entries in entities. | [optional]
**inline_style_ranges** | Option<[**Vec<models::ArticleCreateDraftContentStateBlocksInlineStyleRanges>**](ArticleCreateDraftContentStateBlocksInlineStyleRanges.md)> | Inline formatting ranges. | [optional]
**key** | Option<**String**> | Optional block key. | [optional]
**text** | **String** | The text content of this block. For latex entities, this holds the LaTeX source;          for divider/markdown/image/post atomic placeholders, typically a single space. | 
**r#type** | **Type** | The block type. Use atomic for non-text embeds (image, post, markdown, divider, latex). (enum: unstyled, header-one, header-two, header-three, unordered-list-item, ordered-list-item, blockquote, atomic) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


