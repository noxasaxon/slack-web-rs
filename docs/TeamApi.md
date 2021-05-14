# \TeamApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**team_access_logs**](TeamApi.md#team_access_logs) | **get** /team.accessLogs | 
[**team_billable_info**](TeamApi.md#team_billable_info) | **get** /team.billableInfo | 
[**team_info**](TeamApi.md#team_info) | **get** /team.info | 
[**team_integration_logs**](TeamApi.md#team_integration_logs) | **get** /team.integrationLogs | 
[**team_profile_get**](TeamApi.md#team_profile_get) | **get** /team.profile.get | 



## team_access_logs

> ::std::collections::HashMap<String, serde_json::Value> team_access_logs(token, before, count, page)


Gets the access logs for the current team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin` | [required] |
**before** | Option<**String**> | End of time range of logs to include in results (inclusive). |  |
**count** | Option<**String**> |  |  |
**page** | Option<**String**> |  |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_billable_info

> ::std::collections::HashMap<String, serde_json::Value> team_billable_info(token, user)


Gets billable users information for the current team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin` | [required] |
**user** | Option<**String**> | A user to retrieve the billable information for. Defaults to all users. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_info

> ::std::collections::HashMap<String, serde_json::Value> team_info(token, team)


Gets information about the current team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `team:read` | [required] |
**team** | Option<**String**> | Team to get info on, if omitted, will return information about the current team. Will only return team that the authenticated token is allowed to see through external shared channels |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_integration_logs

> ::std::collections::HashMap<String, serde_json::Value> team_integration_logs(token, app_id, change_type, count, page, service_id, user)


Gets the integration logs for the current team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin` | [required] |
**app_id** | Option<**String**> | Filter logs to this Slack app. Defaults to all logs. |  |
**change_type** | Option<**String**> | Filter logs with this change type. Defaults to all logs. |  |
**count** | Option<**String**> |  |  |
**page** | Option<**String**> |  |  |
**service_id** | Option<**String**> | Filter logs to this service. Defaults to all logs. |  |
**user** | Option<**String**> | Filter logs generated by this user’s actions. Defaults to all logs. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## team_profile_get

> ::std::collections::HashMap<String, serde_json::Value> team_profile_get(token, visibility)


Retrieve a team's profile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `users.profile:read` | [required] |
**visibility** | Option<**String**> | Filter by visibility. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
