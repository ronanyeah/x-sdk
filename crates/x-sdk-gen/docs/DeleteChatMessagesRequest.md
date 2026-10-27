# DeleteChatMessagesRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action_signatures** | [**Vec<models::DeleteChatMessagesActionSignatures>**](DeleteChatMessagesActionSignatures.md) | Signatures for this action, forwarded in the order they are sent. | 
**delete_message_action** | **DeleteMessageAction** | delete_for_all removes the messages for every participant; delete_for_self removes them only from the authenticated user's view. (enum: delete_for_all, delete_for_self) | 
**media_hash_keys** | Option<**Vec<String>**> | Hash keys of encrypted media blobs to delete alongside the messages (delete_for_all only). | [optional]
**sequence_ids** | **Vec<String>** | Sequence IDs of the messages to delete. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


