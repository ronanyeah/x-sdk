# ChatSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allow_dms_from** | Option<**String**> | Who can send the user messages: `all` (anyone), `verified` (verified accounts only), or `following` (accounts the user follows). | [optional]
**always_allow_dms_from_subscribers** | Option<**bool**> | Whether the user's subscribers can always message them regardless of allow_dms_from. | [optional]
**av_settings** | Option<[**models::ChatSettingsAvSettings**](ChatSettingsAvSettings.md)> |  | [optional]
**dm_quality_filter** | Option<**String**> | Low-quality message request filter: `enabled` (low-quality requests go to a separate folder) or `disabled` (all requests stay in the main inbox). | [optional]
**dm_receipt_settings** | Option<**String**> | Read receipts: `all_enabled` (send and receive read receipts) or `all_disabled` (neither). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


