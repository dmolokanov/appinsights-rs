use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};

/// Contains all tags for telemetry to submit.
#[derive(Clone, Default)]
pub struct ContextTags(BTreeMap<String, String>);

impl ContextTags {
    /// Combines all tags from two bags. It can override some tags with values found
    /// in the second tags bag.
    pub fn combine(a: ContextTags, b: ContextTags) -> Self {
        let items = a.0.into_iter().chain(b.0).collect();
        Self(items)
    }
}

impl From<ContextTags> for BTreeMap<String, String> {
    fn from(tags: ContextTags) -> Self {
        tags.0
    }
}

impl Deref for ContextTags {
    type Target = BTreeMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ContextTags {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Macros to generate well-known context tags.
#[macro_export]
macro_rules! tags {
    ( $(#[$attr_factory:meta])* $factory:ident, $(#[$attr:meta])* $name:ident { $( $(#[$attr_method:meta])* $method:ident : $key:expr),* } ) => {
        impl ContextTags{
            $(#[$attr_factory])*
            pub fn $factory(&self) -> $name<'_> {
                $name::new(&self.0)
            }

            paste::item! {
                $(#[$attr_factory])*
                pub fn [<$factory _mut>](&mut self) -> [<$name Mut>]<'_> {
                    [<$name Mut>]::new(&mut self.0)
                }
            }
        }

        $(#[$attr])*
        pub struct $name<'a> {
            items: &'a std::collections::BTreeMap<String, String>,
        }

        impl<'a> $name<'a> {
            /// Returns a new instance of immutable tag helper type.
            fn new(items: &'a std::collections::BTreeMap<String, String>) -> Self {
                Self { items }
            }

            $(
                $(#[$attr_method])*
                pub fn $method(&'a self) -> Option<&'a str> {
                    self.items.get($key).map(|x| x.as_ref())
                }
            )*
        }

        paste::item! {
            $(#[$attr])*
            pub struct [<$name Mut>]<'a> {
                items: &'a mut std::collections::BTreeMap<String, String>,
            }

            impl<'a> [<$name Mut>]<'a> {
                /// Returns a new instance of mutable tag helper type.
                fn new(items: &'a mut std::collections::BTreeMap<String, String>) -> Self {
                    Self { items }
                }
                $(
                    $(#[$attr_method])*
                    pub fn [<set_ $method>](&'a mut self, value: String) {
                        self.items.insert($key.into(), value);
                    }
                )*
            }
        }
    };
}

tags!(
    /// Returns tag helper type that provides access to context fields grouped under 'location'.
    application,
    /// Tag helper type that provides access to context fields grouped under 'location'.
    ApplicationTags {
        /// Application version. Information in the application context fields is always about the application that is sending the telemetry.
        version: "ai.application.ver"
    }
);

tags!(
    /// Returns tag helper type that provides access to context fields grouped under 'device'.
    device,
    /// Tag helper type that provides access to context fields grouped under 'device'.
    DeviceTags {
        /// Unique client device id. Computer name in most cases.
        id: "ai.device.id",
        /// Device locale using <language>-<REGION> pattern, following RFC 5646. Example 'en-US'.
        locale: "ai.device.locale",
        /// Model of the device the end user of the application is using. Used for client scenarios. If this field is empty then it is derived from the user agent.
        model: "ai.device.model",
        /// Client device OEM name taken from the browser.
        oem_name: "ai.device.oemName",
        /// Operating system name and version of the device the end user of the application is using. If this field is empty then it is derived from the user agent. Example 'Windows 10 Pro 10.0.10586.0'
        os_version: "ai.device.osVersion",
        /// The type of the device the end user of the application is using. Used primarily to distinguish JavaScript telemetry from server side telemetry. Examples: 'PC', 'Phone', 'Browser'. 'PC' is the default value.
        r#type: "ai.device.type"
    }
);

tags!(
    /// Returns tag helper type that provides access to context fields grouped under 'location'.
    location,
    /// Tag helper type that provides access to context fields grouped under 'location'.
    LocationTags {
        /// The IP address of the client device. IPv4 and IPv6 are supported. Information in the location context fields is always about the end user. When telemetry is sent from a service, the location context is about the user that initiated the operation in the service.
        ip: "ai.location.ip",
        /// The country of the client device. If any of Country, Province, or City is specified, those values will be preferred over geolocation of the IP address field. Information in the location context fields is always about the end user. When telemetry is sent from a service, the location context is about the user that initiated the operation in the service.
        country: "ai.location.country",
        /// The province/state of the client device. If any of Country, Province, or City is specified, those values will be preferred over geolocation of the IP address field. Information in the location context fields is always about the end user. When telemetry is sent from a service, the location context is about the user that initiated the operation in the service.
        province: "ai.location.province",
        /// The city of the client device. If any of Country, Province, or City is specified, those values will be preferred over geolocation of the IP address field. Information in the location context fields is always about the end user. When telemetry is sent from a service, the location context is about the user that initiated the operation in the service.
        city: "ai.location.city"
    }
);

tags!(
    /// Returns tag helper type that provides access to context fields grouped under 'operation'.
    operation,
    /// Tag helper type that provides access to context fields grouped under 'operation'.
    OperationTags {
        /// A unique identifier for the operation instance. The operation.id is created by either a request or a page view. All other telemetry sets this to the value for the containing request or page view. Operation.id is used for finding all the telemetry items for a specific operation instance.
        id: "ai.operation.id",
        /// The name (group) of the operation. The operation.name is created by either a request or a page view. All other telemetry items set this to the value for the containing request or page view. Operation.name is used for finding all the telemetry items for a group of operations (i.e. 'GET Home/Index').
        name: "ai.operation.name",
        /// The unique identifier of the telemetry item's immediate parent.
        parent_id: "ai.operation.parentId",
        /// Name of synthetic source. Some telemetry from the application may represent a synthetic traffic. It may be web crawler indexing the web site, site availability tests or traces from diagnostic libraries like Application Insights SDK itself.
        synthetic_source: "ai.operation.syntheticSource",
        /// The correlation vector is a light weight vector clock which can be used to identify and order related events across clients and services.
        correlation_vector: "ai.operation.correlationVector"
    }
);

tags!(
    /// Returns tag helper type that provides access to context fields grouped under 'session'.
    session,
    /// Tag helper type that provides access to context fields grouped under 'session'.
    SessionTags {
        /// Session ID - the instance of the user's interaction with the app. Information in the session context fields is always about the end user. When telemetry is sent from a service, the session context is about the user that initiated the operation in the service.
        id: "ai.session.id",
        /// Boolean value indicating whether the session identified by ai.session.id is first for the user or not.
        is_first: "ai.session.isFirst"
    }
);

tags!(
    /// Returns tag helper type that provides access to context fields grouped under 'user'.
    user,
    /// Tag helper type that provides access to context fields grouped under 'user'.
    UserTags {
        /// In multi-tenant applications this is the account ID or name which the user is acting with. Examples may be subscription ID for Azure portal or blog name blogging platform.
        account_id: "ai.user.accountId",
        /// Anonymous user id. Represents the end user of the application. When telemetry is sent from a service, the user context is about the user that initiated the operation in the service.
        id: "ai.user.id",
        /// Authenticated user id. The opposite of ai.user.id, this represents the user with a friendly name. Since it's PII information it is not collected by default by most SDKs.
        auth_user_id: "ai.user.authUserId"
    }
);

tags!(
    /// Returns tag helper type that provides access to context fields grouped under 'cloud'.
    cloud,
    /// Tag helper type that provides access to context fields grouped under 'cloud'.
    CloudTags {
        /// Name of the role the application is a part of. Maps directly to the role name in azure.
        role: "ai.cloud.role",
        /// Version of the role the application is a part of.
        role_ver: "ai.cloud.roleVer",
        /// Name of the instance where the application is running. Computer name for on-premisis, instance name for Azure.
        role_instance: "ai.cloud.roleInstance",
        /// Location of the role the application is a part of.
        location: "ai.cloud.location"
    }
);

tags!(
    /// Returns tag helper type that provides access to context fields grouped under 'internal'.
    internal,
    /// Tag helper type that provides access to context fields grouped under 'internal'.
    InternalTags {
        /// SDK version. See https://github.com/Microsoft/ApplicationInsights-Home/blob/master/SDK-AUTHORING.md#sdk-version-specification for information.
        sdk_version: "ai.internal.sdkVersion",
        /// Agent version. Used to indicate the version of StatusMonitor installed on the computer if it is used for data collection.
        agent_version: "ai.internal.agentVersion",
        /// This is the node name used for billing purposes. Use it to override the standard detection of nodes.
        node_name: "ai.internal.nodeName"
    }
);

#[cfg(test)]
tags!(
    /// Returns example wrapper
    example,
    /// Example tags
    ExampleTags {
        /// foo
        foo: "foo",
        /// bar
        bar: "bar"
    }
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_updates_example_tags() {
        let mut tags = ContextTags::default();

        tags.example_mut().set_foo("foo".into());
        tags.example_mut().set_bar("bar".into());

        assert_eq!(tags.example().foo(), Some("foo"));
        assert_eq!(tags.example().bar(), Some("bar"));
    }

    #[test]
    fn it_updates_example_tags_even_when_example_shared() {
        let mut tags = ContextTags::default();

        let mut example = tags.example_mut();
        example.set_foo("foo".into());
        // example.set_bar("bar".into()); // TODO figure out how to deal with this case

        let example = tags.example();
        assert_eq!(example.foo(), Some("foo"));
        // assert_eq!(example.bar(), Some("bar"));
    }
}
