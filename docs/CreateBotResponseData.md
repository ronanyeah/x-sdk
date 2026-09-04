# CreateBotResponseData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Unique identifier of the bot's User account. | 
**name** | Option<**String**> | The bot's display name. | [optional]
**scopes** | Option<**Vec<String>**> | Scopes granted to the bot's bearer token. | [optional]
**token** | **String** | The bot's bearer token. Returned exactly once; store it securely. | 
**token_expires_at_ms** | Option<**i32**> | Expiry of the bot's bearer token, in epoch milliseconds. | [optional]
**username** | Option<**String**> | The bot's X handle (screen name). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


