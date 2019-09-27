// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================
#![allow(warnings)]

use futures::future;
use futures::Future;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};
use std::error::Error;
use std::fmt;

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde_json;
/// <p>Details of the common errors that all actions return.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct FailureInfo {
    /// <p>The code of the common error. Valid values include <code>InternalServiceException</code>, <code>InvalidParameterException</code>, and any valid error code returned by the AWS service that hosts the resource that you want to tag.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The message of the common error.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The HTTP status code of the common error.</p>
    #[serde(rename = "StatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetResourcesInput {
    /// <p>A string that indicates that additional data is available. Leave this value empty for your initial request. If the response includes a <code>PaginationToken</code>, use that string for this value to request an additional page of data.</p>
    #[serde(rename = "PaginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    /// <p>The constraints on the resources that you want returned. The format of each resource type is <code>service[:resourceType]</code>. For example, specifying a resource type of <code>ec2</code> returns all Amazon EC2 resources (which includes EC2 instances). Specifying a resource type of <code>ec2:instance</code> returns only EC2 instances. </p> <p>The string for each service name and resource type is the same as that embedded in a resource's Amazon Resource Name (ARN). Consult the <i>AWS General Reference</i> for the following:</p> <ul> <li> <p>For a list of service name strings, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces">AWS Service Namespaces</a>.</p> </li> <li> <p>For resource type strings, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arns-syntax">Example ARNs</a>.</p> </li> <li> <p>For more information about ARNs, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p> </li> </ul> <p>You can specify multiple resource types by using an array. The array can include up to 100 items. Note that the length constraint requirement applies to each resource type filter. </p>
    #[serde(rename = "ResourceTypeFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type_filters: Option<Vec<String>>,
    /// <p>A limit that restricts the number of resources returned by GetResources in paginated output. You can set ResourcesPerPage to a minimum of 1 item and the maximum of 100 items. </p>
    #[serde(rename = "ResourcesPerPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources_per_page: Option<i64>,
    /// <p><p>A list of TagFilters (keys and values). Each TagFilter specified must contain a key with values as optional. A request can include up to 50 keys, and each key can include up to 20 values. </p> <p>Note the following when deciding how to use TagFilters:</p> <ul> <li> <p>If you <i>do</i> specify a TagFilter, the response returns only those resources that are currently associated with the specified tag. </p> </li> <li> <p>If you <i>don&#39;t</i> specify a TagFilter, the response includes all resources that were ever associated with tags. Resources that currently don&#39;t have associated tags are shown with an empty tag set, like this: <code>&quot;Tags&quot;: []</code>.</p> </li> <li> <p>If you specify more than one filter in a single request, the response returns only those resources that satisfy all specified filters.</p> </li> <li> <p>If you specify a filter that contains more than one value for a key, the response returns resources that match any of the specified values for that key.</p> </li> <li> <p>If you don&#39;t specify any values for a key, the response returns resources that are tagged with that key irrespective of the value.</p> <p>For example, for filters: filter1 = {key1, {value1}}, filter2 = {key2, {value2,value3,value4}} , filter3 = {key3}:</p> <ul> <li> <p>GetResources( {filter1} ) returns resources tagged with key1=value1</p> </li> <li> <p>GetResources( {filter2} ) returns resources tagged with key2=value2 or key2=value3 or key2=value4</p> </li> <li> <p>GetResources( {filter3} ) returns resources tagged with any tag containing key3 as its tag key, irrespective of its value</p> </li> <li> <p>GetResources( {filter1,filter2,filter3} ) returns resources tagged with ( key1=value1) and ( key2=value2 or key2=value3 or key2=value4) and (key3, irrespective of the value)</p> </li> </ul> </li> </ul></p>
    #[serde(rename = "TagFilters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filters: Option<Vec<TagFilter>>,
    /// <p>A limit that restricts the number of tags (key and value pairs) returned by GetResources in paginated output. A resource with no tags is counted as having one tag (one key and value pair).</p> <p> <code>GetResources</code> does not split a resource and its associated tags across pages. If the specified <code>TagsPerPage</code> would cause such a break, a <code>PaginationToken</code> is returned in place of the affected resource and its tags. Use that token in another request to get the remaining data. For example, if you specify a <code>TagsPerPage</code> of <code>100</code> and the account has 22 resources with 10 tags each (meaning that each resource has 10 key and value pairs), the output will consist of 3 pages, with the first page displaying the first 10 resources, each with its 10 tags, the second page displaying the next 10 resources each with its 10 tags, and the third page displaying the remaining 2 resources, each with its 10 tags.</p> <p>You can set <code>TagsPerPage</code> to a minimum of 100 items and the maximum of 500 items.</p>
    #[serde(rename = "TagsPerPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_per_page: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetResourcesOutput {
    /// <p>A string that indicates that the response contains more data than can be returned in a single response. To receive additional data, specify this string for the <code>PaginationToken</code> value in a subsequent request.</p>
    #[serde(rename = "PaginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    /// <p>A list of resource ARNs and the tags (keys and values) associated with each.</p>
    #[serde(rename = "ResourceTagMappingList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tag_mapping_list: Option<Vec<ResourceTagMapping>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetTagKeysInput {
    /// <p>A string that indicates that additional data is available. Leave this value empty for your initial request. If the response includes a PaginationToken, use that string for this value to request an additional page of data.</p>
    #[serde(rename = "PaginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetTagKeysOutput {
    /// <p>A string that indicates that the response contains more data than can be returned in a single response. To receive additional data, specify this string for the <code>PaginationToken</code> value in a subsequent request.</p>
    #[serde(rename = "PaginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    /// <p>A list of all tag keys in the AWS account.</p>
    #[serde(rename = "TagKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetTagValuesInput {
    /// <p>The key for which you want to list all existing values in the specified region for the AWS account.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>A string that indicates that additional data is available. Leave this value empty for your initial request. If the response includes a PaginationToken, use that string for this value to request an additional page of data.</p>
    #[serde(rename = "PaginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetTagValuesOutput {
    /// <p>A string that indicates that the response contains more data than can be returned in a single response. To receive additional data, specify this string for the <code>PaginationToken</code> value in a subsequent request.</p>
    #[serde(rename = "PaginationToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_token: Option<String>,
    /// <p>A list of all tag values for the specified key in the AWS account.</p>
    #[serde(rename = "TagValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<Vec<String>>,
}

/// <p>A list of resource ARNs and the tags (keys and values) that are associated with each.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ResourceTagMapping {
    /// <p>The ARN of the resource.</p>
    #[serde(rename = "ResourceARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>The tags that have been applied to one or more AWS resources.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>The metadata that you apply to AWS resources to help you categorize and organize them. Each tag consists of a key and an optional value, both of which you define. For more information, see <a href="http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Using_Tags.html#tag-basics">Tag Basics</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Tag {
    /// <p>One part of a key-value pair that make up a tag. A key is a general label that acts like a category for more specific tag values.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The optional part of a key-value pair that make up a tag. A value acts as a descriptor within a tag category (key).</p>
    #[serde(rename = "Value")]
    pub value: String,
}

/// <p>A list of tags (keys and values) that are used to specify the associated resources.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagFilter {
    /// <p>One part of a key-value pair that make up a tag. A key is a general label that acts like a category for more specific tag values.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The optional part of a key-value pair that make up a tag. A value acts as a descriptor within a tag category (key).</p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagResourcesInput {
    /// <p>A list of ARNs. An ARN (Amazon Resource Name) uniquely identifies a resource. You can specify a minimum of 1 and a maximum of 20 ARNs (resources) to tag. An ARN can be set to a maximum of 1600 characters. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a> in the <i>AWS General Reference</i>.</p>
    #[serde(rename = "ResourceARNList")]
    pub resource_arn_list: Vec<String>,
    /// <p>The tags that you want to add to the specified resources. A tag consists of a key and a value that you define.</p>
    #[serde(rename = "Tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TagResourcesOutput {
    /// <p>Details of resources that could not be tagged. An error code, status code, and error message are returned for each failed item.</p>
    #[serde(rename = "FailedResourcesMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_resources_map: Option<::std::collections::HashMap<String, FailureInfo>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagResourcesInput {
    /// <p>A list of ARNs. An ARN (Amazon Resource Name) uniquely identifies a resource. You can specify a minimum of 1 and a maximum of 20 ARNs (resources) to untag. An ARN can be set to a maximum of 1600 characters. For more information, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a> in the <i>AWS General Reference</i>.</p>
    #[serde(rename = "ResourceARNList")]
    pub resource_arn_list: Vec<String>,
    /// <p>A list of the tag keys that you want to remove from the specified resources.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UntagResourcesOutput {
    /// <p>Details of resources that could not be untagged. An error code, status code, and error message are returned for each failed item.</p>
    #[serde(rename = "FailedResourcesMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_resources_map: Option<::std::collections::HashMap<String, FailureInfo>>,
}

/// Errors returned by GetResources
#[derive(Debug, PartialEq)]
pub enum GetResourcesError {
    /// <p>The request processing failed because of an unknown error, exception, or failure. You can retry the request.</p>
    InternalService(String),
    /// <p>A parameter is missing or a malformed string or invalid or out-of-range value was supplied for the request parameter.</p>
    InvalidParameter(String),
    /// <p>A <code>PaginationToken</code> is valid for a maximum of 15 minutes. Your request was denied because the specified <code>PaginationToken</code> has expired.</p>
    PaginationTokenExpired(String),
    /// <p>The request was denied to limit the frequency of submitted requests.</p>
    Throttled(String),
}

impl GetResourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetResourcesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(GetResourcesError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetResourcesError::InvalidParameter(err.msg))
                }
                "PaginationTokenExpiredException" => {
                    return RusotoError::Service(GetResourcesError::PaginationTokenExpired(err.msg))
                }
                "ThrottledException" => {
                    return RusotoError::Service(GetResourcesError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetResourcesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetResourcesError {
    fn description(&self) -> &str {
        match *self {
            GetResourcesError::InternalService(ref cause) => cause,
            GetResourcesError::InvalidParameter(ref cause) => cause,
            GetResourcesError::PaginationTokenExpired(ref cause) => cause,
            GetResourcesError::Throttled(ref cause) => cause,
        }
    }
}
/// Errors returned by GetTagKeys
#[derive(Debug, PartialEq)]
pub enum GetTagKeysError {
    /// <p>The request processing failed because of an unknown error, exception, or failure. You can retry the request.</p>
    InternalService(String),
    /// <p>A parameter is missing or a malformed string or invalid or out-of-range value was supplied for the request parameter.</p>
    InvalidParameter(String),
    /// <p>A <code>PaginationToken</code> is valid for a maximum of 15 minutes. Your request was denied because the specified <code>PaginationToken</code> has expired.</p>
    PaginationTokenExpired(String),
    /// <p>The request was denied to limit the frequency of submitted requests.</p>
    Throttled(String),
}

impl GetTagKeysError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTagKeysError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(GetTagKeysError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetTagKeysError::InvalidParameter(err.msg))
                }
                "PaginationTokenExpiredException" => {
                    return RusotoError::Service(GetTagKeysError::PaginationTokenExpired(err.msg))
                }
                "ThrottledException" => {
                    return RusotoError::Service(GetTagKeysError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetTagKeysError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTagKeysError {
    fn description(&self) -> &str {
        match *self {
            GetTagKeysError::InternalService(ref cause) => cause,
            GetTagKeysError::InvalidParameter(ref cause) => cause,
            GetTagKeysError::PaginationTokenExpired(ref cause) => cause,
            GetTagKeysError::Throttled(ref cause) => cause,
        }
    }
}
/// Errors returned by GetTagValues
#[derive(Debug, PartialEq)]
pub enum GetTagValuesError {
    /// <p>The request processing failed because of an unknown error, exception, or failure. You can retry the request.</p>
    InternalService(String),
    /// <p>A parameter is missing or a malformed string or invalid or out-of-range value was supplied for the request parameter.</p>
    InvalidParameter(String),
    /// <p>A <code>PaginationToken</code> is valid for a maximum of 15 minutes. Your request was denied because the specified <code>PaginationToken</code> has expired.</p>
    PaginationTokenExpired(String),
    /// <p>The request was denied to limit the frequency of submitted requests.</p>
    Throttled(String),
}

impl GetTagValuesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTagValuesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(GetTagValuesError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetTagValuesError::InvalidParameter(err.msg))
                }
                "PaginationTokenExpiredException" => {
                    return RusotoError::Service(GetTagValuesError::PaginationTokenExpired(err.msg))
                }
                "ThrottledException" => {
                    return RusotoError::Service(GetTagValuesError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetTagValuesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTagValuesError {
    fn description(&self) -> &str {
        match *self {
            GetTagValuesError::InternalService(ref cause) => cause,
            GetTagValuesError::InvalidParameter(ref cause) => cause,
            GetTagValuesError::PaginationTokenExpired(ref cause) => cause,
            GetTagValuesError::Throttled(ref cause) => cause,
        }
    }
}
/// Errors returned by TagResources
#[derive(Debug, PartialEq)]
pub enum TagResourcesError {
    /// <p>The request processing failed because of an unknown error, exception, or failure. You can retry the request.</p>
    InternalService(String),
    /// <p>A parameter is missing or a malformed string or invalid or out-of-range value was supplied for the request parameter.</p>
    InvalidParameter(String),
    /// <p>The request was denied to limit the frequency of submitted requests.</p>
    Throttled(String),
}

impl TagResourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourcesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(TagResourcesError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(TagResourcesError::InvalidParameter(err.msg))
                }
                "ThrottledException" => {
                    return RusotoError::Service(TagResourcesError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for TagResourcesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TagResourcesError {
    fn description(&self) -> &str {
        match *self {
            TagResourcesError::InternalService(ref cause) => cause,
            TagResourcesError::InvalidParameter(ref cause) => cause,
            TagResourcesError::Throttled(ref cause) => cause,
        }
    }
}
/// Errors returned by UntagResources
#[derive(Debug, PartialEq)]
pub enum UntagResourcesError {
    /// <p>The request processing failed because of an unknown error, exception, or failure. You can retry the request.</p>
    InternalService(String),
    /// <p>A parameter is missing or a malformed string or invalid or out-of-range value was supplied for the request parameter.</p>
    InvalidParameter(String),
    /// <p>The request was denied to limit the frequency of submitted requests.</p>
    Throttled(String),
}

impl UntagResourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourcesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalServiceException" => {
                    return RusotoError::Service(UntagResourcesError::InternalService(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UntagResourcesError::InvalidParameter(err.msg))
                }
                "ThrottledException" => {
                    return RusotoError::Service(UntagResourcesError::Throttled(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UntagResourcesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UntagResourcesError {
    fn description(&self) -> &str {
        match *self {
            UntagResourcesError::InternalService(ref cause) => cause,
            UntagResourcesError::InvalidParameter(ref cause) => cause,
            UntagResourcesError::Throttled(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS Resource Groups Tagging API API. AWS Resource Groups Tagging API clients implement this trait.
pub trait ResourceGroupsTaggingApi {
    /// <p><p>Returns all the tagged or previously tagged resources that are located in the specified region for the AWS account. You can optionally specify <i>filters</i> (tags and resource types) in your request, depending on what information you want returned. The response includes all tags that are associated with the requested resources.</p> <note> <p>You can check the <code>PaginationToken</code> response parameter to determine if a query completed. Queries can occasionally return fewer results on a page than allowed. The <code>PaginationToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display. </p> </note></p>
    fn get_resources(
        &self,
        input: GetResourcesInput,
    ) -> RusotoFuture<GetResourcesOutput, GetResourcesError>;

    /// <p>Returns all tag keys in the specified region for the AWS account.</p>
    fn get_tag_keys(
        &self,
        input: GetTagKeysInput,
    ) -> RusotoFuture<GetTagKeysOutput, GetTagKeysError>;

    /// <p>Returns all tag values for the specified key in the specified region for the AWS account.</p>
    fn get_tag_values(
        &self,
        input: GetTagValuesInput,
    ) -> RusotoFuture<GetTagValuesOutput, GetTagValuesError>;

    /// <p><p>Applies one or more tags to the specified resources. Note the following:</p> <ul> <li> <p>Not all resources can have tags. For a list of resources that support tagging, see <a href="http://docs.aws.amazon.com/ARG/latest/userguide/supported-resources.html">Supported Resources</a> in the <i>AWS Resource Groups User Guide</i>.</p> </li> <li> <p>Each resource can have up to 50 tags. For other limits, see <a href="http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Using_Tags.html#tag-restrictions">Tag Restrictions</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p> </li> <li> <p>You can only tag resources that are located in the specified region for the AWS account.</p> </li> <li> <p>To add tags to a resource, you need the necessary permissions for the service that the resource belongs to as well as permissions for adding tags. For more information, see <a href="http://docs.aws.amazon.com/ARG/latest/userguide/obtaining-permissions-for-tagging.html">Obtaining Permissions for Tagging</a> in the <i>AWS Resource Groups User Guide</i>.</p> </li> </ul></p>
    fn tag_resources(
        &self,
        input: TagResourcesInput,
    ) -> RusotoFuture<TagResourcesOutput, TagResourcesError>;

    /// <p><p>Removes the specified tags from the specified resources. When you specify a tag key, the action removes both that key and its associated value. The operation succeeds even if you attempt to remove tags from a resource that were already removed. Note the following:</p> <ul> <li> <p>To remove tags from a resource, you need the necessary permissions for the service that the resource belongs to as well as permissions for removing tags. For more information, see <a href="http://docs.aws.amazon.com/ARG/latest/userguide/obtaining-permissions-for-tagging.html">Obtaining Permissions for Tagging</a> in the <i>AWS Resource Groups User Guide</i>.</p> </li> <li> <p>You can only tag resources that are located in the specified region for the AWS account.</p> </li> </ul></p>
    fn untag_resources(
        &self,
        input: UntagResourcesInput,
    ) -> RusotoFuture<UntagResourcesOutput, UntagResourcesError>;
}
/// A client for the AWS Resource Groups Tagging API API.
#[derive(Clone)]
pub struct ResourceGroupsTaggingApiClient {
    client: Client,
    region: region::Region,
}

impl ResourceGroupsTaggingApiClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ResourceGroupsTaggingApiClient {
        Self::new_with_client(Client::shared(), region)
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ResourceGroupsTaggingApiClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        Self::new_with_client(
            Client::new_with(credentials_provider, request_dispatcher),
            region,
        )
    }

    pub fn new_with_client(
        client: Client,
        region: region::Region,
    ) -> ResourceGroupsTaggingApiClient {
        ResourceGroupsTaggingApiClient { client, region }
    }
}

impl ResourceGroupsTaggingApi for ResourceGroupsTaggingApiClient {
    /// <p><p>Returns all the tagged or previously tagged resources that are located in the specified region for the AWS account. You can optionally specify <i>filters</i> (tags and resource types) in your request, depending on what information you want returned. The response includes all tags that are associated with the requested resources.</p> <note> <p>You can check the <code>PaginationToken</code> response parameter to determine if a query completed. Queries can occasionally return fewer results on a page than allowed. The <code>PaginationToken</code> response parameter value is <code>null</code> <i>only</i> when there are no more results to display. </p> </note></p>
    fn get_resources(
        &self,
        input: GetResourcesInput,
    ) -> RusotoFuture<GetResourcesOutput, GetResourcesError> {
        let mut request = SignedRequest::new("POST", "tagging", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ResourceGroupsTaggingAPI_20170126.GetResources",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetResourcesOutput, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetResourcesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns all tag keys in the specified region for the AWS account.</p>
    fn get_tag_keys(
        &self,
        input: GetTagKeysInput,
    ) -> RusotoFuture<GetTagKeysOutput, GetTagKeysError> {
        let mut request = SignedRequest::new("POST", "tagging", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ResourceGroupsTaggingAPI_20170126.GetTagKeys",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetTagKeysOutput, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetTagKeysError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns all tag values for the specified key in the specified region for the AWS account.</p>
    fn get_tag_values(
        &self,
        input: GetTagValuesInput,
    ) -> RusotoFuture<GetTagValuesOutput, GetTagValuesError> {
        let mut request = SignedRequest::new("POST", "tagging", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ResourceGroupsTaggingAPI_20170126.GetTagValues",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetTagValuesOutput, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetTagValuesError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Applies one or more tags to the specified resources. Note the following:</p> <ul> <li> <p>Not all resources can have tags. For a list of resources that support tagging, see <a href="http://docs.aws.amazon.com/ARG/latest/userguide/supported-resources.html">Supported Resources</a> in the <i>AWS Resource Groups User Guide</i>.</p> </li> <li> <p>Each resource can have up to 50 tags. For other limits, see <a href="http://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Using_Tags.html#tag-restrictions">Tag Restrictions</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p> </li> <li> <p>You can only tag resources that are located in the specified region for the AWS account.</p> </li> <li> <p>To add tags to a resource, you need the necessary permissions for the service that the resource belongs to as well as permissions for adding tags. For more information, see <a href="http://docs.aws.amazon.com/ARG/latest/userguide/obtaining-permissions-for-tagging.html">Obtaining Permissions for Tagging</a> in the <i>AWS Resource Groups User Guide</i>.</p> </li> </ul></p>
    fn tag_resources(
        &self,
        input: TagResourcesInput,
    ) -> RusotoFuture<TagResourcesOutput, TagResourcesError> {
        let mut request = SignedRequest::new("POST", "tagging", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ResourceGroupsTaggingAPI_20170126.TagResources",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<TagResourcesOutput, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(TagResourcesError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Removes the specified tags from the specified resources. When you specify a tag key, the action removes both that key and its associated value. The operation succeeds even if you attempt to remove tags from a resource that were already removed. Note the following:</p> <ul> <li> <p>To remove tags from a resource, you need the necessary permissions for the service that the resource belongs to as well as permissions for removing tags. For more information, see <a href="http://docs.aws.amazon.com/ARG/latest/userguide/obtaining-permissions-for-tagging.html">Obtaining Permissions for Tagging</a> in the <i>AWS Resource Groups User Guide</i>.</p> </li> <li> <p>You can only tag resources that are located in the specified region for the AWS account.</p> </li> </ul></p>
    fn untag_resources(
        &self,
        input: UntagResourcesInput,
    ) -> RusotoFuture<UntagResourcesOutput, UntagResourcesError> {
        let mut request = SignedRequest::new("POST", "tagging", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "ResourceGroupsTaggingAPI_20170126.UntagResources",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UntagResourcesOutput, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UntagResourcesError::from_response(response))),
                )
            }
        })
    }
}
