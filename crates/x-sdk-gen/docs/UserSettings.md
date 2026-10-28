# UserSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allow_dms_from** | Option<**String**> | Who can send the user direct messages: `all` (anyone), `verified` (verified accounts only), or `following` (accounts the user follows). | [optional]
**allow_media_tagging** | Option<**String**> | Who may tag the user in media: `all` (anyone), `following` (accounts the user follows), or `none`. | [optional]
**always_allow_dms_from_subscribers** | Option<**bool**> | Whether the user's subscribers can always message them regardless of allow_dms_from. | [optional]
**discoverable_by_email** | Option<**bool**> | Whether people who have the user's email address can find them on X. | [optional]
**discoverable_by_phone** | Option<**bool**> | Whether people who have the user's phone number can find them on X. | [optional]
**display_sensitive_media** | Option<**bool**> | Whether media that may contain sensitive content is displayed to the user. | [optional]
**id** | Option<**String**> | Unique identifier of the user these settings belong to. | [optional]
**language** | Option<**String**> | The language of the user's account, as a BCP 47 tag (for example `en`, `pt-BR`). | [optional]
**location_tagging_enabled** | Option<**bool**> | Whether the user allows a precise location to be attached to their posts. | [optional]
**personalized_trends** | Option<**bool**> | Whether trends shown to the user are personalized. Read-only. | [optional]
**protected** | Option<**bool**> | Whether the user's posts are protected (visible only to approved followers). | [optional]
**quiet_hours** | Option<[**models::UserSettingsQuietHours**](UserSettingsQuietHours.md)> |  | [optional]
**time_zone** | Option<[**models::UserSettingsTimeZone**](UserSettingsTimeZone.md)> |  | [optional]
**use_cookie_personalization** | Option<**bool**> | Whether X may use the user's off-X web activity to personalize their experience. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


