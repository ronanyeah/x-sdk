# RespondToScheduledCallRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**occurrence_start_time** | Option<**String**> | Start of the occurrence that scope this or this_and_following refers to (RFC 3339). | [optional]
**response** | **Response** | The answer to the invitation. (enum: accepted, declined, tentative) | 
**scope** | Option<**Scope**> | Which occurrences of a recurring call: all (the default without occurrence_start_time), this (the default with it), or this_and_following. (enum: all, this, this_and_following) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


