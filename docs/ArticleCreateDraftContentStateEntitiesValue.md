# ArticleCreateDraftContentStateEntitiesValue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**data** | [**models::ArticleCreateDraftContentStateEntitiesValueData**](ArticleCreateDraftContentStateEntitiesValueData.md) |  | 
**mutability** | **Mutability** | Whether the entity can be edited. Recommended: immutable for post, image, divider, latex; mutable for link and markdown (matches the in-app Articles composer). (enum: immutable, mutable, segmented) | 
**r#type** | **Type** | The entity type. markdown carries code blocks, GFM tables, and other Markdown;          emoji maps to backend TWEMOJI (Twemoji is internal); divider is a horizontal rule;          latex renders TeX from the block text. Tables are not a separate enum value — use          type markdown with a pipe table in data.markdown. (enum: post, link, image, emoji, markdown, divider, latex) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


