# User

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**affiliation** | Option<[**models::UserAffiliation**](UserAffiliation.md)> |  | [optional]
**confirmed_email** | Option<**String**> |  | [optional]
**connection_status** | Option<**Vec<ConnectionStatus>**> | Returns detailed information about the relationship between two users. (enum: blocking, follow_request_received, follow_request_sent, followed_by, following, muting) | [optional]
**created_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Creation time of this User. | [optional]
**description** | Option<**String**> | The text of this User's profile description (also known as bio), if the User provided one. | [optional]
**entities** | Option<[**models::UserEntities**](UserEntities.md)> |  | [optional]
**id** | Option<**String**> | Unique identifier of this User. | [optional]
**is_identity_verified** | Option<**bool**> | Indicates if this User has completed identity verification. | [optional]
**location** | Option<**String**> | The location specified in the User's profile, if the User provided one. As this is a freeform value, it may not indicate a valid location. | [optional]
**most_recent_post_id** | Option<**String**> | Unique identifier of this User's most recent Post. | [optional]
**name** | Option<**String**> | The friendly name of this User, as shown on their profile. | [optional]
**parody** | Option<**bool**> | Indicates if this User is a parody account. | [optional]
**pinned_post_id** | Option<**String**> | Unique identifier of this User's pinned Post. | [optional]
**profile_banner_url** | Option<**String**> | The URL to the profile banner for this User. | [optional]
**profile_image_url** | Option<**String**> | The URL to the profile image for this User. | [optional]
**protected** | Option<**bool**> | Indicates if this User has chosen to protect their Posts (in other words, if this User's Posts are private). | [optional]
**public_metrics** | Option<[**models::UserPublicMetrics**](UserPublicMetrics.md)> |  | [optional]
**receives_your_dm** | Option<**bool**> | Indicates if you can send a DM to this User. | [optional]
**subscribes_to_you** | Option<**bool**> | Indicates if this User subscribes to you. | [optional]
**subscription** | Option<[**models::UserSubscription**](UserSubscription.md)> |  | [optional]
**subscription_type** | Option<**String**> | The X Blue subscription type of the user, e.g.: Basic, Premium, PremiumPlus or None. | [optional]
**url** | Option<**String**> | The URL specified in the User's profile. | [optional]
**username** | Option<**String**> | The X handle (screen name) of this User. | [optional]
**verified** | Option<**bool**> | Indicates if this User is a verified X User. | [optional]
**verified_followers_count** | Option<**i32**> | The number of verified followers of this User. | [optional]
**verified_type** | Option<**String**> | The X Blue verified type of the user, e.g.: blue, government, business or none. | [optional]
**withheld** | Option<[**models::UserWithheld**](UserWithheld.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


