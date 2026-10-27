# UpdateScheduledCallRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | What the call is about. | [optional]
**duration_minutes** | Option<**i32**> | How long the call lasts, in minutes. | [optional]
**invitee_ids** | Option<**Vec<String>**> | Everyone to invite: replaces the invite list. At most 25 of them may be people not invited yet. | [optional]
**notify_invitees** | Option<**bool**> | Whether invitees get an email about the change. Defaults to true. | [optional]
**occurrence_start_time** | Option<**String**> | Start of the occurrence that scope this or this_and_following refers to (RFC 3339). | [optional]
**recurrence** | Option<**String**> | RFC 5545 RRULE to repeat the call by, such as FREQ=WEEKLY;BYDAY=MO. | [optional]
**scope** | Option<**Scope**> | Which occurrences of a recurring call: all (the default without occurrence_start_time), this (the default with it), or this_and_following. (enum: all, this, this_and_following) | [optional]
**start_time** | Option<**String**> | When the call (or the chosen occurrence) starts now (RFC 3339). | [optional]
**time_zone** | Option<**String**> | IANA time zone the recurrence follows. | [optional]
**title** | Option<**String**> | Title invitees see. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


