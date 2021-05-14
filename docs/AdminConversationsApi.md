# \AdminConversationsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**admin_conversations_archive**](AdminConversationsApi.md#admin_conversations_archive) | **post** /admin.conversations.archive | 
[**admin_conversations_convert_to_private**](AdminConversationsApi.md#admin_conversations_convert_to_private) | **post** /admin.conversations.convertToPrivate | 
[**admin_conversations_create**](AdminConversationsApi.md#admin_conversations_create) | **post** /admin.conversations.create | 
[**admin_conversations_delete**](AdminConversationsApi.md#admin_conversations_delete) | **post** /admin.conversations.delete | 
[**admin_conversations_disconnect_shared**](AdminConversationsApi.md#admin_conversations_disconnect_shared) | **post** /admin.conversations.disconnectShared | 
[**admin_conversations_get_conversation_prefs**](AdminConversationsApi.md#admin_conversations_get_conversation_prefs) | **get** /admin.conversations.getConversationPrefs | 
[**admin_conversations_get_teams**](AdminConversationsApi.md#admin_conversations_get_teams) | **get** /admin.conversations.getTeams | 
[**admin_conversations_invite**](AdminConversationsApi.md#admin_conversations_invite) | **post** /admin.conversations.invite | 
[**admin_conversations_rename**](AdminConversationsApi.md#admin_conversations_rename) | **post** /admin.conversations.rename | 
[**admin_conversations_search**](AdminConversationsApi.md#admin_conversations_search) | **get** /admin.conversations.search | 
[**admin_conversations_set_conversation_prefs**](AdminConversationsApi.md#admin_conversations_set_conversation_prefs) | **post** /admin.conversations.setConversationPrefs | 
[**admin_conversations_set_teams**](AdminConversationsApi.md#admin_conversations_set_teams) | **post** /admin.conversations.setTeams | 
[**admin_conversations_unarchive**](AdminConversationsApi.md#admin_conversations_unarchive) | **post** /admin.conversations.unarchive | 



## admin_conversations_archive

> ::std::collections::HashMap<String, serde_json::Value> admin_conversations_archive(token, channel_id)


Archive a public or private channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.conversations:write` | [required] |
**channel_id** | **String** | The channel to archive. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_conversations_convert_to_private

> ::std::collections::HashMap<String, serde_json::Value> admin_conversations_convert_to_private(token, channel_id)


Convert a public channel to a private channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.conversations:write` | [required] |
**channel_id** | **String** | The channel to convert to private. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_conversations_create

> ::std::collections::HashMap<String, serde_json::Value> admin_conversations_create(token, name, is_private, description, org_wide, team_id)


Create a public or private channel-based conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.conversations:write` | [required] |
**name** | **String** | Name of the public or private channel to create. | [required] |
**is_private** | **bool** | When `true`, creates a private channel instead of a public channel | [required] |
**description** | Option<**String**> | Description of the public or private channel to create. |  |
**org_wide** | Option<**bool**> | When `true`, the channel will be available org-wide. Note: if the channel is not `org_wide=true`, you must specify a `team_id` for this channel |  |
**team_id** | Option<**String**> | The workspace to create the channel in. Note: this argument is required unless you set `org_wide=true`. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_conversations_delete

> ::std::collections::HashMap<String, serde_json::Value> admin_conversations_delete(token, channel_id)


Delete a public or private channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.conversations:write` | [required] |
**channel_id** | **String** | The channel to delete. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_conversations_disconnect_shared

> ::std::collections::HashMap<String, serde_json::Value> admin_conversations_disconnect_shared(token, channel_id, leaving_team_ids)


Disconnect a connected channel from one or more workspaces.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.conversations:write` | [required] |
**channel_id** | **String** | The channel to be disconnected from some workspaces. | [required] |
**leaving_team_ids** | Option<**String**> | The team to be removed from the channel. Currently only a single team id can be specified. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_conversations_get_conversation_prefs

> ::std::collections::HashMap<String, serde_json::Value> admin_conversations_get_conversation_prefs(token, channel_id)


Get conversation preferences for a public or private channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.conversations:read` | [required] |
**channel_id** | **String** | The channel to get preferences for. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_conversations_get_teams

> ::std::collections::HashMap<String, serde_json::Value> admin_conversations_get_teams(token, channel_id, cursor, limit)


Get all the workspaces a given public or private channel is connected to within this Enterprise org.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.conversations:read` | [required] |
**channel_id** | **String** | The channel to determine connected workspaces within the organization for. | [required] |
**cursor** | Option<**String**> | Set `cursor` to `next_cursor` returned by the previous call to list items in the next page |  |
**limit** | Option<**i32**> | The maximum number of items to return. Must be between 1 - 1000 both inclusive. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_conversations_invite

> ::std::collections::HashMap<String, serde_json::Value> admin_conversations_invite(token, user_ids, channel_id)


Invite a user to a public or private channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.conversations:write` | [required] |
**user_ids** | **String** | The users to invite. | [required] |
**channel_id** | **String** | The channel that the users will be invited to. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_conversations_rename

> ::std::collections::HashMap<String, serde_json::Value> admin_conversations_rename(token, channel_id, name)


Rename a public or private channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.conversations:write` | [required] |
**channel_id** | **String** | The channel to rename. | [required] |
**name** | **String** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_conversations_search

> ::std::collections::HashMap<String, serde_json::Value> admin_conversations_search(token, team_ids, query, limit, cursor, search_channel_types, sort, sort_dir)


Search for public or private channels in an Enterprise organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.conversations:read` | [required] |
**team_ids** | Option<**String**> | Comma separated string of team IDs, signifying the workspaces to search through. |  |
**query** | Option<**String**> | Name of the the channel to query by. |  |
**limit** | Option<**i32**> | Maximum number of items to be returned. Must be between 1 - 20 both inclusive. Default is 10. |  |
**cursor** | Option<**String**> | Set `cursor` to `next_cursor` returned by the previous call to list items in the next page. |  |
**search_channel_types** | Option<**String**> | The type of channel to include or exclude in the search. For example `private` will search private channels, while `private_exclude` will exclude them. For a full list of types, check the [Types section](#types). |  |
**sort** | Option<**String**> | Possible values are `relevant` (search ranking based on what we think is closest), `name` (alphabetical), `member_count` (number of users in the channel), and `created` (date channel was created). You can optionally pair this with the `sort_dir` arg to change how it is sorted  |  |
**sort_dir** | Option<**String**> | Sort direction. Possible values are `asc` for ascending order like (1, 2, 3) or (a, b, c), and `desc` for descending order like (3, 2, 1) or (c, b, a) |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_conversations_set_conversation_prefs

> ::std::collections::HashMap<String, serde_json::Value> admin_conversations_set_conversation_prefs(token, channel_id, prefs)


Set the posting permissions for a public or private channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.conversations:write` | [required] |
**channel_id** | **String** | The channel to set the prefs for | [required] |
**prefs** | **String** | The prefs for this channel in a stringified JSON format. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_conversations_set_teams

> ::std::collections::HashMap<String, serde_json::Value> admin_conversations_set_teams(token, channel_id, team_id, target_team_ids, org_channel)


Set the workspaces in an Enterprise grid org that connect to a public or private channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.conversations:write` | [required] |
**channel_id** | **String** | The encoded `channel_id` to add or remove to workspaces. | [required] |
**team_id** | Option<**String**> | The workspace to which the channel belongs. Omit this argument if the channel is a cross-workspace shared channel. |  |
**target_team_ids** | Option<**String**> | A comma-separated list of workspaces to which the channel should be shared. Not required if the channel is being shared org-wide. |  |
**org_channel** | Option<**bool**> | True if channel has to be converted to an org channel |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_conversations_unarchive

> ::std::collections::HashMap<String, serde_json::Value> admin_conversations_unarchive(token, channel_id)


Unarchive a public or private channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.conversations:write` | [required] |
**channel_id** | **String** | The channel to unarchive. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

