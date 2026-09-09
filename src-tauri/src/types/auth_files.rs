use serde::{Deserialize, Serialize};

// Auth file entry from Management API
// Fields `priority` and `note` added in CLIProxyAPI v6.8.55+ (GET /auth-files response)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthFile {
    pub id: String,
    pub name: String,
    pub provider: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub status: String,
    #[serde(alias = "status_message", skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(default)]
    pub disabled: bool,
    #[serde(default)]
    pub unavailable: bool,
    #[serde(default, alias = "runtime_only")]
    pub runtime_only: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modtime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(alias = "account_type", skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(alias = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(alias = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(alias = "last_refresh", skip_serializing_if = "Option::is_none")]
    pub last_refresh: Option<String>,
    /// Successful request count. CLIProxyAPI reports `success`; ProxyPal serializes camelCase.
    #[serde(
        alias = "success",
        alias = "success_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub success_count: Option<u64>,
    /// Failed request count. CLIProxyAPI reports `failed`; ProxyPal serializes camelCase.
    #[serde(
        alias = "failed",
        alias = "failure_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub failure_count: Option<u64>,
    /// Priority for routing order (lower = higher priority). CLIProxyAPI v6.8.55+
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    /// User-defined note/description for this auth entry. CLIProxyAPI v6.8.55+
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::AuthFile;

    #[test]
    fn deserializes_snake_case_management_auth_file_fields() {
        let value = serde_json::json!({
            "id": "codex-account.json",
            "name": "codex-account.json",
            "provider": "openai",
            "status": "active",
            "status_message": "ready",
            "runtime_only": true,
            "account_type": "plus",
            "created_at": "2026-07-10T12:00:00Z",
            "updated_at": "2026-07-10T12:01:00Z",
            "last_refresh": "2026-07-10T12:02:00Z",
            "success_count": 4,
            "failure_count": 1
        });

        let file: AuthFile = serde_json::from_value(value).unwrap();

        assert_eq!(file.status_message.as_deref(), Some("ready"));
        assert!(file.runtime_only);
        assert_eq!(file.account_type.as_deref(), Some("plus"));
        assert_eq!(file.created_at.as_deref(), Some("2026-07-10T12:00:00Z"));
        assert_eq!(file.updated_at.as_deref(), Some("2026-07-10T12:01:00Z"));
        assert_eq!(file.last_refresh.as_deref(), Some("2026-07-10T12:02:00Z"));
        assert_eq!(file.success_count, Some(4));
        assert_eq!(file.failure_count, Some(1));
    }

    #[test]
    fn deserializes_upstream_success_and_failed_counts() {
        // CLIProxyAPI /v0/management/auth-files reports `success` and `failed`.
        let value = serde_json::json!({
            "id": "codex-account.json",
            "name": "codex-account.json",
            "provider": "openai",
            "status": "ready",
            "success": 12,
            "failed": 3
        });

        let file: AuthFile = serde_json::from_value(value).unwrap();

        assert_eq!(file.success_count, Some(12));
        assert_eq!(file.failure_count, Some(3));

        // The frontend contract stays camelCase.
        let serialized = serde_json::to_value(&file).unwrap();
        assert_eq!(serialized["successCount"], 12);
        assert_eq!(serialized["failureCount"], 3);
    }

    #[test]
    fn deserializes_camel_case_management_auth_file_fields() {
        let value = serde_json::json!({
            "id": "claude-account.json",
            "name": "claude-account.json",
            "provider": "claude",
            "status": "active",
            "statusMessage": "ready",
            "runtimeOnly": true,
            "accountType": "pro",
            "createdAt": "2026-07-10T12:00:00Z",
            "updatedAt": "2026-07-10T12:01:00Z",
            "lastRefresh": "2026-07-10T12:02:00Z",
            "successCount": 5,
            "failureCount": 2
        });

        let file: AuthFile = serde_json::from_value(value).unwrap();

        assert_eq!(file.status_message.as_deref(), Some("ready"));
        assert!(file.runtime_only);
        assert_eq!(file.account_type.as_deref(), Some("pro"));
        assert_eq!(file.created_at.as_deref(), Some("2026-07-10T12:00:00Z"));
        assert_eq!(file.updated_at.as_deref(), Some("2026-07-10T12:01:00Z"));
        assert_eq!(file.last_refresh.as_deref(), Some("2026-07-10T12:02:00Z"));
        assert_eq!(file.success_count, Some(5));
        assert_eq!(file.failure_count, Some(2));
    }
}
