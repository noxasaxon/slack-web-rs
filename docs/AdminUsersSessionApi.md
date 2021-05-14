# \AdminUsersSessionApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**admin_users_session_invalidate**](AdminUsersSessionApi.md#admin_users_session_invalidate) | **post** /admin.users.session.invalidate | 
[**admin_users_session_reset**](AdminUsersSessionApi.md#admin_users_session_reset) | **post** /admin.users.session.reset | 



## admin_users_session_invalidate

> ::std::collections::HashMap<String, serde_json::Value> admin_users_session_invalidate(token, team_id, session_id)


Invalidate a single session for a user by session_id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.users:write` | [required] |
**team_id** | **String** | ID of the team that the session belongs to | [required] |
**session_id** | **i32** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_users_session_reset

> ::std::collections::HashMap<String, serde_json::Value> admin_users_session_reset(token, user_id, mobile_only, web_only)


Wipes all valid sessions on all devices for a given user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.users:write` | [required] |
**user_id** | **String** | The ID of the user to wipe sessions for | [required] |
**mobile_only** | Option<**bool**> | Only expire mobile sessions (default: false) |  |
**web_only** | Option<**bool**> | Only expire web sessions (default: false) |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

