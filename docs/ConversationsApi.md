# \ConversationsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**conversations_archive**](ConversationsApi.md#conversations_archive) | **post** /conversations.archive | 
[**conversations_close**](ConversationsApi.md#conversations_close) | **post** /conversations.close | 
[**conversations_create**](ConversationsApi.md#conversations_create) | **post** /conversations.create | 
[**conversations_history**](ConversationsApi.md#conversations_history) | **get** /conversations.history | 
[**conversations_info**](ConversationsApi.md#conversations_info) | **get** /conversations.info | 
[**conversations_invite**](ConversationsApi.md#conversations_invite) | **post** /conversations.invite | 
[**conversations_join**](ConversationsApi.md#conversations_join) | **post** /conversations.join | 
[**conversations_kick**](ConversationsApi.md#conversations_kick) | **post** /conversations.kick | 
[**conversations_leave**](ConversationsApi.md#conversations_leave) | **post** /conversations.leave | 
[**conversations_list**](ConversationsApi.md#conversations_list) | **get** /conversations.list | 
[**conversations_mark**](ConversationsApi.md#conversations_mark) | **post** /conversations.mark | 
[**conversations_members**](ConversationsApi.md#conversations_members) | **get** /conversations.members | 
[**conversations_open**](ConversationsApi.md#conversations_open) | **post** /conversations.open | 
[**conversations_rename**](ConversationsApi.md#conversations_rename) | **post** /conversations.rename | 
[**conversations_replies**](ConversationsApi.md#conversations_replies) | **get** /conversations.replies | 
[**conversations_set_purpose**](ConversationsApi.md#conversations_set_purpose) | **post** /conversations.setPurpose | 
[**conversations_set_topic**](ConversationsApi.md#conversations_set_topic) | **post** /conversations.setTopic | 
[**conversations_unarchive**](ConversationsApi.md#conversations_unarchive) | **post** /conversations.unarchive | 



## conversations_archive

> ::std::collections::HashMap<String, serde_json::Value> conversations_archive(token, channel)


Archives a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**channel** | Option<**String**> | ID of conversation to archive |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_close

> ::std::collections::HashMap<String, serde_json::Value> conversations_close(token, channel)


Closes a direct message or multi-person direct message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**channel** | Option<**String**> | Conversation to close. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_create

> ::std::collections::HashMap<String, serde_json::Value> conversations_create(token, name, is_private)


Initiates a public or private channel-based conversation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**name** | Option<**String**> | Name of the public or private channel to create |  |
**is_private** | Option<**bool**> | Create a private channel instead of a public one |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_history

> ::std::collections::HashMap<String, serde_json::Value> conversations_history(token, channel, latest, oldest, inclusive, limit, cursor)


Fetches a conversation's history of messages and events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:history` |  |
**channel** | Option<**String**> | Conversation ID to fetch history for. |  |
**latest** | Option<**f32**> | End of time range of messages to include in results. |  |
**oldest** | Option<**f32**> | Start of time range of messages to include in results. |  |
**inclusive** | Option<**bool**> | Include messages with latest or oldest timestamp in results only when either timestamp is specified. |  |
**limit** | Option<**i32**> | The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the users list hasn't been reached. |  |
**cursor** | Option<**String**> | Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first \"page\" of the collection. See [pagination](/docs/pagination) for more detail. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_info

> ::std::collections::HashMap<String, serde_json::Value> conversations_info(token, channel, include_locale, include_num_members)


Retrieve information about a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:read` |  |
**channel** | Option<**String**> | Conversation ID to learn more about |  |
**include_locale** | Option<**bool**> | Set this to `true` to receive the locale for this conversation. Defaults to `false` |  |
**include_num_members** | Option<**bool**> | Set to `true` to include the member count for the specified conversation. Defaults to `false` |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_invite

> ::std::collections::HashMap<String, serde_json::Value> conversations_invite(token, channel, users)


Invites users to a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**channel** | Option<**String**> | The ID of the public or private channel to invite user(s) to. |  |
**users** | Option<**String**> | A comma separated list of user IDs. Up to 1000 users may be listed. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_join

> ::std::collections::HashMap<String, serde_json::Value> conversations_join(token, channel)


Joins an existing conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `channels:write` |  |
**channel** | Option<**String**> | ID of conversation to join |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_kick

> ::std::collections::HashMap<String, serde_json::Value> conversations_kick(token, channel, user)


Removes a user from a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**channel** | Option<**String**> | ID of conversation to remove user from. |  |
**user** | Option<**String**> | User ID to be removed. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_leave

> ::std::collections::HashMap<String, serde_json::Value> conversations_leave(token, channel)


Leaves a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**channel** | Option<**String**> | Conversation to leave |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_list

> ::std::collections::HashMap<String, serde_json::Value> conversations_list(token, exclude_archived, types, limit, cursor)


Lists all channels in a Slack team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:read` |  |
**exclude_archived** | Option<**bool**> | Set to `true` to exclude archived channels from the list |  |
**types** | Option<**String**> | Mix and match channel types by providing a comma-separated list of any combination of `public_channel`, `private_channel`, `mpim`, `im` |  |
**limit** | Option<**i32**> | The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the list hasn't been reached. Must be an integer no larger than 1000. |  |
**cursor** | Option<**String**> | Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first \"page\" of the collection. See [pagination](/docs/pagination) for more detail. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_mark

> ::std::collections::HashMap<String, serde_json::Value> conversations_mark(token, channel, ts)


Sets the read cursor in a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**channel** | Option<**String**> | Channel or conversation to set the read cursor for. |  |
**ts** | Option<**f32**> | Unique identifier of message you want marked as most recently seen in this conversation. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_members

> ::std::collections::HashMap<String, serde_json::Value> conversations_members(token, channel, limit, cursor)


Retrieve members of a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:read` |  |
**channel** | Option<**String**> | ID of the conversation to retrieve members for |  |
**limit** | Option<**i32**> | The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the users list hasn't been reached. |  |
**cursor** | Option<**String**> | Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first \"page\" of the collection. See [pagination](/docs/pagination) for more detail. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_open

> ::std::collections::HashMap<String, serde_json::Value> conversations_open(token, channel, users, return_im)


Opens or resumes a direct message or multi-person direct message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**channel** | Option<**String**> | Resume a conversation by supplying an `im` or `mpim`'s ID. Or provide the `users` field instead. |  |
**users** | Option<**String**> | Comma separated lists of users. If only one user is included, this creates a 1:1 DM.  The ordering of the users is preserved whenever a multi-person direct message is returned. Supply a `channel` when not supplying `users`. |  |
**return_im** | Option<**bool**> | Boolean, indicates you want the full IM channel definition in the response. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_rename

> ::std::collections::HashMap<String, serde_json::Value> conversations_rename(token, channel, name)


Renames a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**channel** | Option<**String**> | ID of conversation to rename |  |
**name** | Option<**String**> | New name for conversation. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_replies

> ::std::collections::HashMap<String, serde_json::Value> conversations_replies(token, channel, ts, latest, oldest, inclusive, limit, cursor)


Retrieve a thread of messages posted to a conversation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:history` |  |
**channel** | Option<**String**> | Conversation ID to fetch thread from. |  |
**ts** | Option<**f32**> | Unique identifier of a thread's parent message. `ts` must be the timestamp of an existing message with 0 or more replies. If there are no replies then just the single message referenced by `ts` will return - it is just an ordinary, unthreaded message. |  |
**latest** | Option<**f32**> | End of time range of messages to include in results. |  |
**oldest** | Option<**f32**> | Start of time range of messages to include in results. |  |
**inclusive** | Option<**bool**> | Include messages with latest or oldest timestamp in results only when either timestamp is specified. |  |
**limit** | Option<**i32**> | The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the users list hasn't been reached. |  |
**cursor** | Option<**String**> | Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first \"page\" of the collection. See [pagination](/docs/pagination) for more detail. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_set_purpose

> ::std::collections::HashMap<String, serde_json::Value> conversations_set_purpose(token, channel, purpose)


Sets the purpose for a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**channel** | Option<**String**> | Conversation to set the purpose of |  |
**purpose** | Option<**String**> | A new, specialer purpose |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_set_topic

> ::std::collections::HashMap<String, serde_json::Value> conversations_set_topic(token, channel, topic)


Sets the topic for a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**channel** | Option<**String**> | Conversation to set the topic of |  |
**topic** | Option<**String**> | The new topic string. Does not support formatting or linkification. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_unarchive

> ::std::collections::HashMap<String, serde_json::Value> conversations_unarchive(token, channel)


Reverses conversation archival.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**channel** | Option<**String**> | ID of conversation to unarchive |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

