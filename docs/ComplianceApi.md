# \ComplianceApi

All URIs are relative to *https://api.x.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_compliance_jobs**](ComplianceApi.md#create_compliance_jobs) | **POST** /2/compliance/jobs | Create Compliance Job
[**get_compliance_jobs**](ComplianceApi.md#get_compliance_jobs) | **GET** /2/compliance/jobs | Get Compliance Jobs
[**get_compliance_jobs_by_id**](ComplianceApi.md#get_compliance_jobs_by_id) | **GET** /2/compliance/jobs/{id} | Get Compliance Job by ID



## create_compliance_jobs

> models::CreateComplianceJobsResponse create_compliance_jobs(create_compliance_jobs_request, compliance_job_fields)
Create Compliance Job

Creates a new Compliance Job for the specified job type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_compliance_jobs_request** | [**CreateComplianceJobsRequest**](CreateComplianceJobsRequest.md) |  | [required] |
**compliance_job_fields** | Option<[**Vec<models::ComplianceJobField>**](Models__ComplianceJobField.md)> | A comma separated list of ComplianceJob fields to display. |  |

### Return type

[**models::CreateComplianceJobsResponse**](CreateComplianceJobsResponse.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_compliance_jobs

> models::GetComplianceJobsResponse get_compliance_jobs(r#type, status, compliance_job_fields)
Get Compliance Jobs

Retrieves a list of Compliance Jobs filtered by job type and optional status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**r#type** | **String** |  | [required] |
**status** | Option<**String**> |  |  |
**compliance_job_fields** | Option<[**Vec<models::ComplianceJobField>**](Models__ComplianceJobField.md)> | A comma separated list of ComplianceJob fields to display. |  |

### Return type

[**models::GetComplianceJobsResponse**](GetComplianceJobsResponse.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_compliance_jobs_by_id

> models::GetComplianceJobsByIdResponse get_compliance_jobs_by_id(id, compliance_job_fields)
Get Compliance Job by ID

Retrieves details of a specific Compliance Job by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**compliance_job_fields** | Option<[**Vec<models::ComplianceJobField>**](Models__ComplianceJobField.md)> | A comma separated list of ComplianceJob fields to display. |  |

### Return type

[**models::GetComplianceJobsByIdResponse**](GetComplianceJobsByIdResponse.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

