#![allow(dead_code)]

use crate::client::GoogleAdsClient;
use crate::error::Result;
use crate::types::responses::SearchRow;

/// Iterator-like helper that handles pagination over search results.
pub struct PageIterator<'a> {
    client: &'a GoogleAdsClient,
    customer_id: String,
    query: String,
    page_size: Option<i32>,
    next_page_token: Option<String>,
    done: bool,
}

impl<'a> PageIterator<'a> {
    pub fn new(
        client: &'a GoogleAdsClient,
        customer_id: String,
        query: String,
        page_size: Option<i32>,
    ) -> Self {
        Self {
            client,
            customer_id,
            query,
            page_size,
            next_page_token: None,
            done: false,
        }
    }

    /// Fetch the next page of results. Returns None when exhausted.
    pub async fn next_page(&mut self) -> Result<Option<Vec<SearchRow>>> {
        if self.done {
            return Ok(None);
        }

        let response = self
            .client
            .search(
                &self.customer_id,
                &self.query,
                self.page_size,
                self.next_page_token.as_deref(),
            )
            .await?;

        match response.next_page_token {
            Some(token) if !token.is_empty() => {
                self.next_page_token = Some(token);
            }
            _ => {
                self.done = true;
            }
        }

        if response.results.is_empty() {
            self.done = true;
            return Ok(None);
        }

        Ok(Some(response.results))
    }

    /// Collect all pages into a single Vec
    pub async fn collect_all(&mut self) -> Result<Vec<SearchRow>> {
        let mut all = Vec::new();
        while let Some(page) = self.next_page().await? {
            all.extend(page);
        }
        Ok(all)
    }
}
