# GetRuleCountsResponseData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**all_project_client_apps** | Option<[**Vec<models::GetRuleCountsResponseDataAllProjectClientApps>**](GetRuleCountsResponseDataAllProjectClientApps.md)> | Per-client-app rule counts across the project. | [optional]
**cap_per_client_app** | **String** | Maximum number of rules allowed per client application. | 
**cap_per_project** | **String** | Maximum number of rules allowed across the project. | 
**client_app_rules_count** | [**models::GetRuleCountsResponseDataClientAppRulesCount**](GetRuleCountsResponseDataClientAppRulesCount.md) |  | 
**project_rules_count** | **String** | Number of rules across every client application in the project. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


