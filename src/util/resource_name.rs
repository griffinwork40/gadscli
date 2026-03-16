#![allow(dead_code)]

/// Parse and build Google Ads resource names like `customers/123/campaigns/456`
pub struct ResourceName;

impl ResourceName {
    pub fn customer(customer_id: &str) -> String {
        format!("customers/{}", customer_id)
    }

    pub fn campaign(customer_id: &str, campaign_id: &str) -> String {
        format!("customers/{}/campaigns/{}", customer_id, campaign_id)
    }

    pub fn ad_group(customer_id: &str, ad_group_id: &str) -> String {
        format!("customers/{}/adGroups/{}", customer_id, ad_group_id)
    }

    pub fn ad_group_ad(customer_id: &str, ad_group_id: &str, ad_id: &str) -> String {
        format!(
            "customers/{}/adGroupAds/{}~{}",
            customer_id, ad_group_id, ad_id
        )
    }

    pub fn ad_group_criterion(
        customer_id: &str,
        ad_group_id: &str,
        criterion_id: &str,
    ) -> String {
        format!(
            "customers/{}/adGroupCriteria/{}~{}",
            customer_id, ad_group_id, criterion_id
        )
    }

    pub fn campaign_budget(customer_id: &str, budget_id: &str) -> String {
        format!("customers/{}/campaignBudgets/{}", customer_id, budget_id)
    }

    pub fn bidding_strategy(customer_id: &str, strategy_id: &str) -> String {
        format!(
            "customers/{}/biddingStrategies/{}",
            customer_id, strategy_id
        )
    }

    pub fn asset(customer_id: &str, asset_id: &str) -> String {
        format!("customers/{}/assets/{}", customer_id, asset_id)
    }

    pub fn label(customer_id: &str, label_id: &str) -> String {
        format!("customers/{}/labels/{}", customer_id, label_id)
    }

    pub fn conversion_action(customer_id: &str, action_id: &str) -> String {
        format!(
            "customers/{}/conversionActions/{}",
            customer_id, action_id
        )
    }

    pub fn recommendation(customer_id: &str, rec_id: &str) -> String {
        format!("customers/{}/recommendations/{}", customer_id, rec_id)
    }

    pub fn batch_job(customer_id: &str, job_id: &str) -> String {
        format!("customers/{}/batchJobs/{}", customer_id, job_id)
    }

    /// Extract the customer ID from a resource name
    pub fn extract_customer_id(resource_name: &str) -> Option<String> {
        let parts: Vec<&str> = resource_name.split('/').collect();
        if parts.len() >= 2 && parts[0] == "customers" {
            Some(parts[1].to_string())
        } else {
            None
        }
    }

    /// Extract the resource ID from a resource name (last segment)
    pub fn extract_id(resource_name: &str) -> Option<String> {
        resource_name.split('/').last().map(String::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_campaign_resource_name() {
        assert_eq!(
            ResourceName::campaign("123", "456"),
            "customers/123/campaigns/456"
        );
    }

    #[test]
    fn test_extract_customer_id() {
        assert_eq!(
            ResourceName::extract_customer_id("customers/123/campaigns/456"),
            Some("123".into())
        );
    }

    #[test]
    fn test_extract_id() {
        assert_eq!(
            ResourceName::extract_id("customers/123/campaigns/456"),
            Some("456".into())
        );
    }
}
