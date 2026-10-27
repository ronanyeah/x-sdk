# ScheduleCallResponseData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**calendar** | Option<**String**> | Where the call lives: x (X Calls), or outlook for a Microsoft Outlook calendar the user linked to X, whose calls are changed there. | [optional]
**call_id** | Option<**String**> | The X call people join; also its join secret. | [optional]
**description** | Option<**String**> | What the call is about. | [optional]
**end_time** | Option<**String**> | End of this occurrence (RFC 3339). | [optional]
**id** | Option<**String**> | Unique identifier of the scheduled call: what PUT, DELETE and rsvp take. Calls on linked calendars have none. | [optional]
**invitees** | Option<[**Vec<models::ScheduleCallResponseDataInvitees>**](ScheduleCallResponseDataInvitees.md)> | Everyone invited, except the organizer. Not set for linked calendars. | [optional]
**join_url** | Option<**String**> | Link people open to join the call on X. | [optional]
**occurrence_start_time** | Option<**String**> | For a recurring call, this occurrence's original start (RFC 3339): pass it as occurrence_start_time to change, cancel or answer just this occurrence. | [optional]
**organizer_id** | Option<**String**> | X user id of the call's organizer. | [optional]
**recurrence** | Option<**String**> | RFC 5545 RRULE the call repeats by, such as FREQ=WEEKLY;BYDAY=MO. | [optional]
**start_time** | Option<**String**> | Start of this occurrence (RFC 3339). | [optional]
**status** | Option<**String**> | scheduled, or cancelled for a cancelled occurrence. | [optional]
**time_zone** | Option<**String**> | IANA time zone the recurrence follows. | [optional]
**title** | Option<**String**> | Title of the call. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


