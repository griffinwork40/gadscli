#![allow(dead_code)]

/// Fluent builder for GAQL queries
pub struct QueryBuilder {
    select: Vec<String>,
    from: Option<String>,
    where_clauses: Vec<String>,
    order_by: Vec<(String, bool)>, // (field, is_desc)
    limit: Option<i32>,
    parameters: Option<String>,
}

impl QueryBuilder {
    pub fn new() -> Self {
        Self {
            select: Vec::new(),
            from: None,
            where_clauses: Vec::new(),
            order_by: Vec::new(),
            limit: None,
            parameters: None,
        }
    }

    pub fn select(mut self, fields: &[&str]) -> Self {
        self.select.extend(fields.iter().map(|f| f.to_string()));
        self
    }

    pub fn from(mut self, resource: &str) -> Self {
        self.from = Some(resource.to_string());
        self
    }

    pub fn where_clause(mut self, condition: &str) -> Self {
        self.where_clauses.push(condition.to_string());
        self
    }

    /// Add a WHERE condition only if the value is Some
    pub fn where_if(self, field: &str, op: &str, value: Option<&str>) -> Self {
        match value {
            Some(v) => self.where_clause(&format!("{} {} '{}'", field, op, v)),
            None => self,
        }
    }

    /// Add a WHERE != condition
    pub fn where_not(self, field: &str, value: &str) -> Self {
        self.where_clause(&format!("{} != '{}'", field, value))
    }

    pub fn order_by(mut self, field: &str, desc: bool) -> Self {
        self.order_by.push((field.to_string(), desc));
        self
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn limit_if(self, limit: Option<i32>) -> Self {
        match limit {
            Some(l) => self.limit(l),
            None => self,
        }
    }

    pub fn parameters(mut self, params: &str) -> Self {
        self.parameters = Some(params.to_string());
        self
    }

    pub fn build(self) -> crate::error::Result<String> {
        if self.select.is_empty() {
            return Err(crate::error::GadsError::Validation(
                "SELECT clause is required".into(),
            ));
        }
        if self.from.is_none() {
            return Err(crate::error::GadsError::Validation(
                "FROM clause is required".into(),
            ));
        }

        let mut query = format!(
            "SELECT {} FROM {}",
            self.select.join(", "),
            self.from.unwrap()
        );

        if !self.where_clauses.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&self.where_clauses.join(" AND "));
        }

        if !self.order_by.is_empty() {
            query.push_str(" ORDER BY ");
            let orders: Vec<String> = self
                .order_by
                .iter()
                .map(|(f, desc)| {
                    if *desc {
                        format!("{} DESC", f)
                    } else {
                        f.clone()
                    }
                })
                .collect();
            query.push_str(&orders.join(", "));
        }

        if let Some(limit) = self.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }

        if let Some(params) = &self.parameters {
            query.push_str(&format!(" PARAMETERS {}", params));
        }

        Ok(query)
    }
}

impl Default for QueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}
