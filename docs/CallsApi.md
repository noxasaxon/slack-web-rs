# \CallsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**calls_add**](CallsApi.md#calls_add) | **post** /calls.add | 
[**calls_end**](CallsApi.md#calls_end) | **post** /calls.end | 
[**calls_info**](CallsApi.md#calls_info) | **get** /calls.info | 
[**calls_participants_add**](CallsApi.md#calls_participants_add) | **post** /calls.participants.add | 
[**calls_participants_remove**](CallsApi.md#calls_participants_remove) | **post** /calls.participants.remove | 
[**calls_update**](CallsApi.md#calls_update) | **post** /calls.update | 



## calls_add

> ::std::collections::HashMap<String, serde_json::Value> calls_add(token, external_unique_id, join_url, external_display_id, desktop_app_join_url, date_start, title, created_by, users)


Registers a new Call.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `calls:write` | [required] |
**external_unique_id** | **String** | An ID supplied by the 3rd-party Call provider. It must be unique across all Calls from that service. | [required] |
**join_url** | **String** | The URL required for a client to join the Call. | [required] |
**external_display_id** | Option<**String**> | An optional, human-readable ID supplied by the 3rd-party Call provider. If supplied, this ID will be displayed in the Call object. |  |
**desktop_app_join_url** | Option<**String**> | When supplied, available Slack clients will attempt to directly launch the 3rd-party Call with this URL. |  |
**date_start** | Option<**i32**> | Call start time in UTC UNIX timestamp format |  |
**title** | Option<**String**> | The name of the Call. |  |
**created_by** | Option<**String**> | The valid Slack user ID of the user who created this Call. When this method is called with a user token, the `created_by` field is optional and defaults to the authed user of the token. Otherwise, the field is required. |  |
**users** | Option<**String**> | The list of users to register as participants in the Call. [Read more on how to specify users here](/apis/calls#users). |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calls_end

> ::std::collections::HashMap<String, serde_json::Value> calls_end(token, id, duration)


Ends a Call.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `calls:write` | [required] |
**id** | **String** | `id` returned when registering the call using the [`calls.add`](/methods/calls.add) method. | [required] |
**duration** | Option<**i32**> | Call duration in seconds |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calls_info

> ::std::collections::HashMap<String, serde_json::Value> calls_info(token, id)


Returns information about a Call.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `calls:read` | [required] |
**id** | **String** | `id` of the Call returned by the [`calls.add`](/methods/calls.add) method. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calls_participants_add

> ::std::collections::HashMap<String, serde_json::Value> calls_participants_add(token, id, users)


Registers new participants added to a Call.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `calls:write` | [required] |
**id** | **String** | `id` returned by the [`calls.add`](/methods/calls.add) method. | [required] |
**users** | **String** | The list of users to add as participants in the Call. [Read more on how to specify users here](/apis/calls#users). | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calls_participants_remove

> ::std::collections::HashMap<String, serde_json::Value> calls_participants_remove(token, id, users)


Registers participants removed from a Call.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `calls:write` | [required] |
**id** | **String** | `id` returned by the [`calls.add`](/methods/calls.add) method. | [required] |
**users** | **String** | The list of users to remove as participants in the Call. [Read more on how to specify users here](/apis/calls#users). | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## calls_update

> ::std::collections::HashMap<String, serde_json::Value> calls_update(token, id, title, join_url, desktop_app_join_url)


Updates information about a Call.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `calls:write` | [required] |
**id** | **String** | `id` returned by the [`calls.add`](/methods/calls.add) method. | [required] |
**title** | Option<**String**> | The name of the Call. |  |
**join_url** | Option<**String**> | The URL required for a client to join the Call. |  |
**desktop_app_join_url** | Option<**String**> | When supplied, available Slack clients will attempt to directly launch the 3rd-party Call with this URL. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

