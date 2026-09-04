# CreateActivitySubscriptionFilter

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**direction** | Option<**Direction**> | Optional direction filter for directional events. Not supported for mute.* or block.* events. (enum: inbound, outbound) | [optional]
**keyword** | Option<**String**> | Optional keyword filter. | [optional]
**qualifiers** | Option<**serde_json::Value**> | Optional event-specific string predicates. | [optional]
**user_id** | Option<**String**> | User the subscription is scoped to. For mute.* and block.* events, this must be the authenticated source user. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


