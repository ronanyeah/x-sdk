# ScheduleCallRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | What the call is about. | [optional]
**duration_minutes** | Option<**i32**> | How long the call lasts, in minutes. Defaults to 30; at most 31 days. | [optional]
**invitee_ids** | Option<**Vec<String>**> | X user ids of the people to invite. | [optional]
**notify_invitees** | Option<**bool**> | Whether invitees get an email about the call. Defaults to true. | [optional]
**recurrence** | Option<**String**> | RFC 5545 RRULE to repeat the call by, such as FREQ=WEEKLY;BYDAY=MO; the RRULE: prefix is optional. | [optional]
**start_time** | **String** | When the call starts (RFC 3339); not in the past. | 
**time_zone** | Option<**String**> | IANA time zone the recurrence follows, such as America/Los_Angeles. | [optional]
**title** | Option<**String**> | Title invitees see. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


