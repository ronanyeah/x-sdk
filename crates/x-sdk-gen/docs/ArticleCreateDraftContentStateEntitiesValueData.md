# ArticleCreateDraftContentStateEntitiesValueData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**caption** | Option<**String**> | Caption text. | [optional]
**entity_key** | Option<**String**> | Opaque entity key. Used with type emoji (and reserved for other keyed embeds) to match in-app Articles composer payloads. | [optional]
**markdown** | Option<**String**> | Markdown body for type markdown. Use fenced code blocks (```lang ... ```) for code. Max weighted length is enforced by the Articles backend (10,000 per article). | [optional]
**media_items** | Option<[**Vec<models::ArticleCreateDraftContentStateEntitiesValueDataMediaItems>**](ArticleCreateDraftContentStateEntitiesValueDataMediaItems.md)> | Media keys. Used with type image. | [optional]
**post_id** | Option<**String**> | The ID of the post to embed. Used with type post. | [optional]
**url** | Option<**String**> | The URL. Used with type link. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


