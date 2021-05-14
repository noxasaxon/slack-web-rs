# \StarsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**stars_add**](StarsApi.md#stars_add) | **post** /stars.add | 
[**stars_list**](StarsApi.md#stars_list) | **get** /stars.list | 
[**stars_remove**](StarsApi.md#stars_remove) | **post** /stars.remove | 



## stars_add

> ::std::collections::HashMap<String, serde_json::Value> stars_add(token, channel, file, file_comment, timestamp)


Adds a star to an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `stars:write` | [required] |
**channel** | Option<**String**> | Channel to add star to, or channel where the message to add star to was posted (used with `timestamp`). |  |
**file** | Option<**String**> | File to add star to. |  |
**file_comment** | Option<**String**> | File comment to add star to. |  |
**timestamp** | Option<**String**> | Timestamp of the message to add star to. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stars_list

> ::std::collections::HashMap<String, serde_json::Value> stars_list(token, count, page, cursor, limit)


Lists stars for a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `stars:read` |  |
**count** | Option<**String**> |  |  |
**page** | Option<**String**> |  |  |
**cursor** | Option<**String**> | Parameter for pagination. Set `cursor` equal to the `next_cursor` attribute returned by the previous request's `response_metadata`. This parameter is optional, but pagination is mandatory: the default value simply fetches the first \"page\" of the collection. See [pagination](/docs/pagination) for more details. |  |
**limit** | Option<**i32**> | The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the list hasn't been reached. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stars_remove

> ::std::collections::HashMap<String, serde_json::Value> stars_remove(token, channel, file, file_comment, timestamp)


Removes a star from an item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `stars:write` | [required] |
**channel** | Option<**String**> | Channel to remove star from, or channel where the message to remove star from was posted (used with `timestamp`). |  |
**file** | Option<**String**> | File to remove star from. |  |
**file_comment** | Option<**String**> | File comment to remove star from. |  |
**timestamp** | Option<**String**> | Timestamp of the message to remove star from. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

