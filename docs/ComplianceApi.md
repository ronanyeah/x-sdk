# \ComplianceApi

All URIs are relative to *https://api.x.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_compliance_jobs**](ComplianceApi.md#create_compliance_jobs) | **POST** /2/compliance/jobs | Create Compliance Job
[**delete_compliance_jobs_by_id**](ComplianceApi.md#delete_compliance_jobs_by_id) | **DELETE** /2/compliance/jobs/{id} | Cancel Compliance Job
[**download_compliance_job_results**](ComplianceApi.md#download_compliance_job_results) | **GET** /2/compliance/jobs/{id}/download | Download Compliance Job Results
[**get_compliance_jobs**](ComplianceApi.md#get_compliance_jobs) | **GET** /2/compliance/jobs | Get Compliance Jobs
[**get_compliance_jobs_by_id**](ComplianceApi.md#get_compliance_jobs_by_id) | **GET** /2/compliance/jobs/{id} | Get Compliance Job by ID
[**upload_compliance_job_submission**](ComplianceApi.md#upload_compliance_job_submission) | **PUT** /2/compliance/jobs/{id}/upload | Upload Compliance Job Submission



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


## delete_compliance_jobs_by_id

> models::DeleteComplianceJobsByIdResponse delete_compliance_jobs_by_id(id, compliance_job_fields)
Cancel Compliance Job

Cancels a Compliance Job that has not finished processing; its status becomes failed and a new job of the same type can be created.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**compliance_job_fields** | Option<[**Vec<models::ComplianceJobField>**](Models__ComplianceJobField.md)> | A comma separated list of ComplianceJob fields to display. |  |

### Return type

[**models::DeleteComplianceJobsByIdResponse**](DeleteComplianceJobsByIdResponse.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_compliance_job_results

> std::path::PathBuf download_compliance_job_results(id, token, compliance_job_fields)
Download Compliance Job Results

Downloads the results file of a completed Compliance Job using the signed URL returned at job creation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**token** | **String** | Signed download token from the job's download_url. | [required] |
**compliance_job_fields** | Option<[**Vec<models::ComplianceJobField>**](Models__ComplianceJobField.md)> | A comma separated list of ComplianceJob fields to display. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, application/json, application/problem+json

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


## upload_compliance_job_submission

> models::UploadComplianceJobSubmissionResponse upload_compliance_job_submission(id, token, body, compliance_job_fields)
Upload Compliance Job Submission

Uploads the newline-delimited ID file for a Compliance Job using the signed URL returned at job creation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**token** | **String** | Signed upload token from the job's upload_url. | [required] |
**body** | **std::path::PathBuf** |  | [required] |
**compliance_job_fields** | Option<[**Vec<models::ComplianceJobField>**](Models__ComplianceJobField.md)> | A comma separated list of ComplianceJob fields to display. |  |

### Return type

[**models::UploadComplianceJobSubmissionResponse**](UploadComplianceJobSubmissionResponse.md)

### Authorization

[BearerToken](../README.md#BearerToken)

### HTTP request headers

- **Content-Type**: application/octet-stream
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

