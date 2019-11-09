use serde::Serialize;

// NOTE: This file was automatically generated.

#[derive(Debug, Serialize)]
pub struct ContextTagKeys {
    application_version: Option<String>,
    device_id: Option<String>,
    device_locale: Option<String>,
    device_model: Option<String>,
    device_oem_name: Option<String>,
    device_os_version: Option<String>,
    device_type: Option<String>,
    location_ip: Option<String>,
    location_country: Option<String>,
    location_province: Option<String>,
    location_city: Option<String>,
    operation_id: Option<String>,
    operation_name: Option<String>,
    operation_parent_id: Option<String>,
    operation_synthetic_source: Option<String>,
    operation_correlation_vector: Option<String>,
    session_id: Option<String>,
    session_is_first: Option<String>,
    user_account_id: Option<String>,
    user_id: Option<String>,
    user_auth_user_id: Option<String>,
    cloud_role: Option<String>,
    cloud_role_ver: Option<String>,
    cloud_role_instance: Option<String>,
    cloud_location: Option<String>,
    internal_sdk_version: Option<String>,
    internal_agent_version: Option<String>,
    internal_node_name: Option<String>,
}

impl ContextTagKeys {
    /// Create a new [ContextTagKeys](trait.ContextTagKeys.html) instance with default values set by the schema.
    pub fn new() -> Self {
        Self {
            application_version: Some(String::from("ai.application.ver")),
            device_id: Some(String::from("ai.device.id")),
            device_locale: Some(String::from("ai.device.locale")),
            device_model: Some(String::from("ai.device.model")),
            device_oem_name: Some(String::from("ai.device.oemName")),
            device_os_version: Some(String::from("ai.device.osVersion")),
            device_type: Some(String::from("ai.device.type")),
            location_ip: Some(String::from("ai.location.ip")),
            location_country: Some(String::from("ai.location.country")),
            location_province: Some(String::from("ai.location.province")),
            location_city: Some(String::from("ai.location.city")),
            operation_id: Some(String::from("ai.operation.id")),
            operation_name: Some(String::from("ai.operation.name")),
            operation_parent_id: Some(String::from("ai.operation.parentId")),
            operation_synthetic_source: Some(String::from("ai.operation.syntheticSource")),
            operation_correlation_vector: Some(String::from("ai.operation.correlationVector")),
            session_id: Some(String::from("ai.session.id")),
            session_is_first: Some(String::from("ai.session.isFirst")),
            user_account_id: Some(String::from("ai.user.accountId")),
            user_id: Some(String::from("ai.user.id")),
            user_auth_user_id: Some(String::from("ai.user.authUserId")),
            cloud_role: Some(String::from("ai.cloud.role")),
            cloud_role_ver: Some(String::from("ai.cloud.roleVer")),
            cloud_role_instance: Some(String::from("ai.cloud.roleInstance")),
            cloud_location: Some(String::from("ai.cloud.location")),
            internal_sdk_version: Some(String::from("ai.internal.sdkVersion")),
            internal_agent_version: Some(String::from("ai.internal.agentVersion")),
            internal_node_name: Some(String::from("ai.internal.nodeName")),
        }
    }

    /// Application version. Information in the application context fields is always about the application that is sending the telemetry.
    pub fn with_application_version(&mut self, application_version: Option<String>) -> &mut Self {
        self.application_version = application_version;
        self
    }

    /// Unique client device id. Computer name in most cases.
    pub fn with_device_id(&mut self, device_id: Option<String>) -> &mut Self {
        self.device_id = device_id;
        self
    }

    /// Device locale using <language>-<REGION> pattern, following RFC 5646. Example 'en-US'.
    pub fn with_device_locale(&mut self, device_locale: Option<String>) -> &mut Self {
        self.device_locale = device_locale;
        self
    }

    /// Model of the device the end user of the application is using. Used for client scenarios. If this field is empty then it is derived from the user agent.
    pub fn with_device_model(&mut self, device_model: Option<String>) -> &mut Self {
        self.device_model = device_model;
        self
    }

    /// Client device OEM name taken from the browser.
    pub fn with_device_oem_name(&mut self, device_oem_name: Option<String>) -> &mut Self {
        self.device_oem_name = device_oem_name;
        self
    }

    /// Operating system name and version of the device the end user of the application is using. If this field is empty then it is derived from the user agent. Example 'Windows 10 Pro 10.0.10586.0'
    pub fn with_device_os_version(&mut self, device_os_version: Option<String>) -> &mut Self {
        self.device_os_version = device_os_version;
        self
    }

    /// The type of the device the end user of the application is using. Used primarily to distinguish JavaScript telemetry from server side telemetry. Examples: 'PC', 'Phone', 'Browser'. 'PC' is the default value.
    pub fn with_device_type(&mut self, device_type: Option<String>) -> &mut Self {
        self.device_type = device_type;
        self
    }

    /// The IP address of the client device. IPv4 and IPv6 are supported. Information in the location context fields is always about the end user. When telemetry is sent from a service, the location context is about the user that initiated the operation in the service.
    pub fn with_location_ip(&mut self, location_ip: Option<String>) -> &mut Self {
        self.location_ip = location_ip;
        self
    }

    /// The country of the client device. If any of Country, Province, or City is specified, those values will be preferred over geolocation of the IP address field. Information in the location context fields is always about the end user. When telemetry is sent from a service, the location context is about the user that initiated the operation in the service.
    pub fn with_location_country(&mut self, location_country: Option<String>) -> &mut Self {
        self.location_country = location_country;
        self
    }

    /// The province/state of the client device. If any of Country, Province, or City is specified, those values will be preferred over geolocation of the IP address field. Information in the location context fields is always about the end user. When telemetry is sent from a service, the location context is about the user that initiated the operation in the service.
    pub fn with_location_province(&mut self, location_province: Option<String>) -> &mut Self {
        self.location_province = location_province;
        self
    }

    /// The city of the client device. If any of Country, Province, or City is specified, those values will be preferred over geolocation of the IP address field. Information in the location context fields is always about the end user. When telemetry is sent from a service, the location context is about the user that initiated the operation in the service.
    pub fn with_location_city(&mut self, location_city: Option<String>) -> &mut Self {
        self.location_city = location_city;
        self
    }

    /// A unique identifier for the operation instance. The operation.id is created by either a request or a page view. All other telemetry sets this to the value for the containing request or page view. Operation.id is used for finding all the telemetry items for a specific operation instance.
    pub fn with_operation_id(&mut self, operation_id: Option<String>) -> &mut Self {
        self.operation_id = operation_id;
        self
    }

    /// The name (group) of the operation. The operation.name is created by either a request or a page view. All other telemetry items set this to the value for the containing request or page view. Operation.name is used for finding all the telemetry items for a group of operations (i.e. 'GET Home/Index').
    pub fn with_operation_name(&mut self, operation_name: Option<String>) -> &mut Self {
        self.operation_name = operation_name;
        self
    }

    /// The unique identifier of the telemetry item's immediate parent.
    pub fn with_operation_parent_id(&mut self, operation_parent_id: Option<String>) -> &mut Self {
        self.operation_parent_id = operation_parent_id;
        self
    }

    /// Name of synthetic source. Some telemetry from the application may represent a synthetic traffic. It may be web crawler indexing the web site, site availability tests or traces from diagnostic libraries like Application Insights SDK itself.
    pub fn with_operation_synthetic_source(&mut self, operation_synthetic_source: Option<String>) -> &mut Self {
        self.operation_synthetic_source = operation_synthetic_source;
        self
    }

    /// The correlation vector is a light weight vector clock which can be used to identify and order related events across clients and services.
    pub fn with_operation_correlation_vector(&mut self, operation_correlation_vector: Option<String>) -> &mut Self {
        self.operation_correlation_vector = operation_correlation_vector;
        self
    }

    /// Session ID - the instance of the user's interaction with the app. Information in the session context fields is always about the end user. When telemetry is sent from a service, the session context is about the user that initiated the operation in the service.
    pub fn with_session_id(&mut self, session_id: Option<String>) -> &mut Self {
        self.session_id = session_id;
        self
    }

    /// Boolean value indicating whether the session identified by ai.session.id is first for the user or not.
    pub fn with_session_is_first(&mut self, session_is_first: Option<String>) -> &mut Self {
        self.session_is_first = session_is_first;
        self
    }

    /// In multi-tenant applications this is the account ID or name which the user is acting with. Examples may be subscription ID for Azure portal or blog name blogging platform.
    pub fn with_user_account_id(&mut self, user_account_id: Option<String>) -> &mut Self {
        self.user_account_id = user_account_id;
        self
    }

    /// Anonymous user id. Represents the end user of the application. When telemetry is sent from a service, the user context is about the user that initiated the operation in the service.
    pub fn with_user_id(&mut self, user_id: Option<String>) -> &mut Self {
        self.user_id = user_id;
        self
    }

    /// Authenticated user id. The opposite of ai.user.id, this represents the user with a friendly name. Since it's PII information it is not collected by default by most SDKs.
    pub fn with_user_auth_user_id(&mut self, user_auth_user_id: Option<String>) -> &mut Self {
        self.user_auth_user_id = user_auth_user_id;
        self
    }

    /// Name of the role the application is a part of. Maps directly to the role name in azure.
    pub fn with_cloud_role(&mut self, cloud_role: Option<String>) -> &mut Self {
        self.cloud_role = cloud_role;
        self
    }

    pub fn with_cloud_role_ver(&mut self, cloud_role_ver: Option<String>) -> &mut Self {
        self.cloud_role_ver = cloud_role_ver;
        self
    }

    /// Name of the instance where the application is running. Computer name for on-premisis, instance name for Azure.
    pub fn with_cloud_role_instance(&mut self, cloud_role_instance: Option<String>) -> &mut Self {
        self.cloud_role_instance = cloud_role_instance;
        self
    }

    pub fn with_cloud_location(&mut self, cloud_location: Option<String>) -> &mut Self {
        self.cloud_location = cloud_location;
        self
    }

    /// SDK version. See https://github.com/Microsoft/ApplicationInsights-Home/blob/master/SDK-AUTHORING.md#sdk-version-specification for information.
    pub fn with_internal_sdk_version(&mut self, internal_sdk_version: Option<String>) -> &mut Self {
        self.internal_sdk_version = internal_sdk_version;
        self
    }

    /// Agent version. Used to indicate the version of StatusMonitor installed on the computer if it is used for data collection.
    pub fn with_internal_agent_version(&mut self, internal_agent_version: Option<String>) -> &mut Self {
        self.internal_agent_version = internal_agent_version;
        self
    }

    /// This is the node name used for billing purposes. Use it to override the standard detection of nodes.
    pub fn with_internal_node_name(&mut self, internal_node_name: Option<String>) -> &mut Self {
        self.internal_node_name = internal_node_name;
        self
    }
}
