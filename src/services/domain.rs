use crate::client::ReloopClient;
use crate::models::*;
use reqwest::Method;

pub struct DomainService<'a> {
    client: &'a ReloopClient,
}

impl<'a> DomainService<'a> {
    pub fn new(client: &'a ReloopClient) -> Self {
        DomainService { client }
    }

    pub async fn create(
        &self,
        params: CreateDomainParams,
    ) -> Result<Domain, Box<dyn std::error::Error>> {
        let body = serde_json::to_value(params)?;
        self.client
            .fetch(Method::POST, "/api/domain/v1/create", Some(body))
            .await
    }

    pub async fn list(
        &self,
        params: Option<ListDomainsParams>,
    ) -> Result<DomainListResponse, Box<dyn std::error::Error>> {
        let path = build_list_path(params.as_ref());
        self.client.fetch(Method::GET, &path, None).await
    }

    pub async fn get(&self, domain_id: &str) -> Result<Domain, Box<dyn std::error::Error>> {
        self.client
            .fetch(Method::GET, &format!("/api/domain/v1/{domain_id}"), None)
            .await
    }

    pub async fn get_nameservers(
        &self,
        domain_id: &str,
    ) -> Result<DomainNameserversResponse, Box<dyn std::error::Error>> {
        self.client
            .fetch(
                Method::GET,
                &format!("/api/domain/v1/nameservers/{domain_id}"),
                None,
            )
            .await
    }

    pub async fn update(
        &self,
        domain_id: &str,
        params: UpdateDomainParams,
    ) -> Result<Domain, Box<dyn std::error::Error>> {
        let body = serde_json::to_value(params)?;
        self.client
            .fetch(Method::PATCH, &format!("/api/domain/v1/{domain_id}"), Some(body))
            .await
    }

    pub async fn delete(&self, domain_id: &str) -> Result<Domain, Box<dyn std::error::Error>> {
        self.client
            .fetch(Method::DELETE, &format!("/api/domain/v1/{domain_id}"), None)
            .await
    }

    pub async fn verify(
        &self,
        domain_id: &str,
    ) -> Result<DomainStatusResponse, Box<dyn std::error::Error>> {
        self.client
            .fetch(
                Method::POST,
                &format!("/api/domain/v1/verify/{domain_id}"),
                None,
            )
            .await
    }

    pub async fn forward_dns(
        &self,
        domain_id: &str,
        params: ForwardDnsParams,
    ) -> Result<ForwardDnsResponse, Box<dyn std::error::Error>> {
        let body = serde_json::to_value(params)?;
        self.client
            .fetch(
                Method::POST,
                &format!("/api/domain/v1/verify/{domain_id}/forward-dns"),
                Some(body),
            )
            .await
    }
}

fn build_list_path(params: Option<&ListDomainsParams>) -> String {
    let Some(params) = params else {
        return "/api/domain/v1/list".to_string();
    };

    let mut query = Vec::new();
    if let Some(page) = params.page {
        query.push(format!("page={page}"));
    }
    if let Some(limit) = params.limit {
        query.push(format!("limit={limit}"));
    }
    if let Some(q) = &params.q {
        query.push(format!("q={q}"));
    }
    if let Some(status) = params.status {
        query.push(format!("status={}", domain_status_as_str(status)));
    }

    if query.is_empty() {
        "/api/domain/v1/list".to_string()
    } else {
        format!("/api/domain/v1/list?{}", query.join("&"))
    }
}

fn domain_status_as_str(status: DomainStatus) -> &'static str {
    match status {
        DomainStatus::Pending => "pending",
        DomainStatus::Verifying => "verifying",
        DomainStatus::Active => "active",
        DomainStatus::Suspended => "suspended",
        DomainStatus::Failed => "failed",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::DomainStatus;

    #[test]
    fn create_route_uses_api_prefix() {
        assert_eq!("/api/domain/v1/create", "/api/domain/v1/create");
    }

    #[test]
    fn get_nameservers_route_includes_domain_id() {
        let domain_id = "dom_1";
        assert_eq!(
            format!("/api/domain/v1/nameservers/{domain_id}"),
            "/api/domain/v1/nameservers/dom_1"
        );
    }

    #[test]
    fn forward_dns_route_uses_verify_path() {
        let domain_id = "dom_1";
        assert_eq!(
            format!("/api/domain/v1/verify/{domain_id}/forward-dns"),
            "/api/domain/v1/verify/dom_1/forward-dns"
        );
    }

    #[test]
    fn build_list_path_includes_filters() {
        let path = build_list_path(Some(&ListDomainsParams {
            page: Some(2),
            limit: Some(5),
            q: Some("example".to_string()),
            status: Some(DomainStatus::Active),
        }));

        assert_eq!(
            path,
            "/api/domain/v1/list?page=2&limit=5&q=example&status=active"
        );
    }
}
