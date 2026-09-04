# GetUsageCreditsResponseData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**free_balance** | **f64** | USD remaining across unexpired free grants. | 
**free_grants** | [**Vec<models::GetUsageCreditsResponseDataFreeGrants>**](GetUsageCreditsResponseDataFreeGrants.md) | Unexpired free grants, soonest expiry first. | 
**prepaid_balance** | **f64** | Purchased (prepaid) USD balance. May be negative when paid usage has overdrawn this bucket. | 
**total_balance** | **f64** | Spendable USD remaining, clamped at 0.00: total_balance = max(0, prepaid_balance + free_balance). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


