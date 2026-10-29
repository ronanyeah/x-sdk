# UpdateUserBroadcastStateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**chat_option** | Option<**i32**> | Who can chat: 0 default, 1 chat off, 2 everyone, 3 verified accounts, 4 accounts the host follows, 5 the host's subscribers, 6 accounts followed by an account the host follows. `PUBLISH` only. | [optional]
**locale** | Option<**String**> | Broadcast language. `PUBLISH` only. | [optional]
**should_not_tweet** | Option<**bool**> | Skip the announcing Post. Unset keeps the broadcast's setting, which defaults to posting. `PUBLISH` only. | [optional]
**state** | **String** | `PUBLISH` or `END`. | 
**title** | Option<**String**> | Broadcast title. `PUBLISH` only. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


