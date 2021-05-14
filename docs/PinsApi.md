# \PinsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**pins_add**](PinsApi.md#pins_add) | **post** /pins.add | 
[**pins_list**](PinsApi.md#pins_list) | **get** /pins.list | 
[**pins_remove**](PinsApi.md#pins_remove) | **post** /pins.remove | 



## pins_add

> ::std::collections::HashMap<String, serde_json::Value> pins_add(token, channel, timestamp)


Pins an item to a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `pins:write` | [required] |
**channel** | **String** | Channel to pin the item in. | [required] |
**timestamp** | Option<**String**> | Timestamp of the message to pin. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pins_list

> serde_json::Value pins_list(token, channel)


Lists items pinned to a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `pins:read` | [required] |
**channel** | **String** | Channel to get pinned items for. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pins_remove

> ::std::collections::HashMap<String, serde_json::Value> pins_remove(token, channel, timestamp)


Un-pins an item from a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `pins:write` | [required] |
**channel** | **String** | Channel where the item is pinned to. | [required] |
**timestamp** | Option<**String**> | Timestamp of the message to un-pin. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

