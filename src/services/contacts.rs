use crate::client::ReloopClient;
use crate::parameters::{build_query, for_query, for_request};
use reqwest::Method;
use serde_json::Value;

pub struct ContactsService<'a> {
    client: &'a ReloopClient,
}

impl<'a> ContactsService<'a> {
    pub fn new(client: &'a ReloopClient) -> Self {
        ContactsService { client }
    }

    pub fn groups(&self) -> ContactGroupsService<'a> {
        ContactGroupsService::new(self.client)
    }

    pub fn channels(&self) -> ContactChannelsService<'a> {
        ContactChannelsService::new(self.client)
    }

    pub async fn create(
        &self,
        parameters: Value,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        self.client
            .fetch_value(
                Method::POST,
                "/api/contacts/create",
                Some(for_request(parameters)),
            )
            .await
    }

    pub async fn get(&self, contact_id: &str) -> Result<Value, Box<dyn std::error::Error>> {
        self.client
            .fetch_value(
                Method::GET,
                &format!("/api/contacts/retrieve/{contact_id}"),
                None,
            )
            .await
    }

    pub async fn list(&self, options: Value) -> Result<Value, Box<dyn std::error::Error>> {
        if let Some(group_id) = options.get("group_id").or_else(|| options.get("groupId")) {
            let group_id = group_id
                .as_str()
                .ok_or("group_id must be a string")?
                .to_string();
            let mut filtered = options.clone();
            if let Value::Object(map) = &mut filtered {
                map.remove("group_id");
                map.remove("groupId");
            }
            return self.groups().list_contacts(&group_id, filtered).await;
        }

        let query = build_query(&for_query(options));
        self.client
            .fetch_value(Method::GET, &format!("/api/contacts/list{query}"), None)
            .await
    }

    pub async fn update(
        &self,
        contact_id: &str,
        parameters: Value,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        self.client
            .fetch_value(
                Method::PATCH,
                &format!("/api/contacts/{contact_id}"),
                Some(for_request(parameters)),
            )
            .await
    }

    pub async fn delete(&self, contact_id: &str) -> Result<Value, Box<dyn std::error::Error>> {
        self.client
            .fetch_value(
                Method::DELETE,
                &format!("/api/contacts/{contact_id}"),
                None,
            )
            .await
    }

    pub async fn create_property(
        &self,
        parameters: Value,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        self.client.fetch_value(
            Method::POST,
            "/api/contacts/v1/properties/create",
            Some(for_request(parameters)),
        ).await
    }

    pub async fn list_properties(
        &self,
        options: Value,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let query = build_query(&for_query(options));
        self.client.fetch_value(
            Method::GET,
            &format!("/api/contacts/v1/properties/list{query}"),
            None,
        ).await
    }

    pub async fn update_property(
        &self,
        property_id: &str,
        parameters: Value,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        self.client.fetch_value(
            Method::PATCH,
            &format!("/api/contacts/v1/properties/{property_id}"),
            Some(for_request(parameters)),
        ).await
    }

    pub async fn delete_property(
        &self,
        property_id: &str,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        self.client.fetch_value(
            Method::DELETE,
            &format!("/api/contacts/v1/properties/{property_id}"),
            None,
        ).await
    }

    pub async fn create_group(
        &self,
        parameters: Value,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        self.client.fetch_value(
            Method::POST,
            "/api/contacts/v1/groups/create",
            Some(for_request(parameters)),
        ).await
    }

    pub async fn list_groups(
        &self,
        options: Value,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let query = build_query(&for_query(options));
        self.client.fetch_value(
            Method::GET,
            &format!("/api/contacts/v1/groups/list{query}"),
            None,
        ).await
    }

    pub async fn get_group(&self, group_id: &str) -> Result<Value, Box<dyn std::error::Error>> {
        self.client.fetch_value(
            Method::GET,
            &format!("/api/contacts/v1/groups/{group_id}"),
            None,
        ).await
    }

    pub async fn update_group(
        &self,
        group_id: &str,
        parameters: Value,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        self.client.fetch_value(
            Method::PATCH,
            &format!("/api/contacts/v1/groups/{group_id}"),
            Some(for_request(parameters)),
        ).await
    }

    pub async fn delete_group(&self, group_id: &str) -> Result<Value, Box<dyn std::error::Error>> {
        self.client.fetch_value(
            Method::DELETE,
            &format!("/api/contacts/v1/groups/{group_id}"),
            None,
        ).await
    }
}

pub struct ContactGroupsService<'a> {
    client: &'a ReloopClient,
}

impl<'a> ContactGroupsService<'a> {
    pub fn new(client: &'a ReloopClient) -> Self {
        ContactGroupsService { client }
    }

    pub async fn add_contact(
        &self,
        group_id: &str,
        parameters: Value,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        self.client.fetch_value(
            Method::POST,
            &format!("/api/contacts/group/{group_id}"),
            Some(for_request(parameters)),
        ).await
    }

    pub async fn remove_contact(
        &self,
        group_id: &str,
        parameters: Value,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        self.client.fetch_value(
            Method::DELETE,
            &format!("/api/contacts/group/{group_id}"),
            Some(for_request(parameters)),
        ).await
    }

    pub async fn list_contacts(
        &self,
        group_id: &str,
        options: Value,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let query = build_query(&for_query(options));
        self.client.fetch_value(
            Method::GET,
            &format!("/api/contacts/v1/groups/{group_id}/contacts{query}"),
            None,
        ).await
    }
}

pub struct ContactChannelsService<'a> {
    client: &'a ReloopClient,
}

impl<'a> ContactChannelsService<'a> {
    pub fn new(client: &'a ReloopClient) -> Self {
        ContactChannelsService { client }
    }

    pub async fn create(
        &self,
        parameters: Value,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        self.client.fetch_value(
            Method::POST,
            "/api/contacts/v1/channels/create",
            Some(for_request(parameters)),
        ).await
    }

    pub async fn list(&self, options: Value) -> Result<Value, Box<dyn std::error::Error>> {
        let query = build_query(&for_query(options));
        self.client.fetch_value(
            Method::GET,
            &format!("/api/contacts/v1/channels/list{query}"),
            None,
        ).await
    }

    pub async fn get(&self, channel_id: &str) -> Result<Value, Box<dyn std::error::Error>> {
        self.client.fetch_value(
            Method::GET,
            &format!("/api/contacts/v1/channels/{channel_id}"),
            None,
        ).await
    }

    pub async fn update(
        &self,
        channel_id: &str,
        parameters: Value,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        self.client.fetch_value(
            Method::PATCH,
            &format!("/api/contacts/v1/channels/{channel_id}"),
            Some(for_request(parameters)),
        ).await
    }

    pub async fn delete(&self, channel_id: &str) -> Result<Value, Box<dyn std::error::Error>> {
        self.client.fetch_value(
            Method::DELETE,
            &format!("/api/contacts/v1/channels/{channel_id}"),
            None,
        ).await
    }

    pub async fn add_contact(
        &self,
        channel_id: &str,
        parameters: Value,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        self.client.fetch_value(
            Method::POST,
            &format!("/api/contacts/channel/{channel_id}"),
            Some(for_request(parameters)),
        ).await
    }

    pub async fn update_subscription(
        &self,
        channel_id: &str,
        parameters: Value,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        self.client.fetch_value(
            Method::PATCH,
            &format!("/api/contacts/channel/{channel_id}"),
            Some(for_request(parameters)),
        ).await
    }
}
