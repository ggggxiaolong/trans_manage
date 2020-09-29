pub struct SqlHelper {
    pub sql: String,
    has_where: bool,
}

impl SqlHelper {
    pub fn query(table: &str, columns: &str) -> Self {
        let mut helper = SqlHelper {
            sql: String::new(),
            has_where: false,
        };
        helper.sql.push_str("SELECT ");
        helper.sql.push_str(columns);
        helper.sql.push_str(" FROM ");
        helper.sql.push_str(table);
        helper
    }

    pub fn and_where_eq(&mut self, column_name: &str, mask: &str) -> &mut Self {
        if self.has_where {
            self.sql.push_str(" AND ");
        } else {
            self.has_where = true
        }
        self.sql.push_str(" WHERE ");
        self.sql.push_str(column_name);
        self.sql.push_str(" = ");
        self.sql.push_str(mask);
        self
    }

    pub fn and_where_not_eq(&mut self, column_name: &str, mask: &str) -> &mut Self {
        if self.has_where {
            self.sql.push_str(" AND ");
        } else {
            self.has_where = true
        }
        self.sql.push_str(" WHERE ");
        self.sql.push_str(column_name);
        self.sql.push_str(" <> ");
        self.sql.push_str(mask);
        self
    }

    pub fn and_where_like(&mut self, column_name: &str, mask: &str) -> &mut Self {
        if self.has_where {
            self.sql.push_str(" AND ");
        } else {
            self.has_where = true
        }
        self.sql.push_str(" WHERE ");
        self.sql.push_str(column_name);
        self.sql.push_str(" like %");
        self.sql.push_str(mask);
        self.sql.push('%');
        self
    }

    pub fn and_where_like_option(&mut self, column_name: &str, mask: Option<String>) -> &mut Self {
        match mask {
            None => self,
            Some(mask) => {
                if mask.len() == 0 {
                    return self;
                }
                if self.has_where {
                    self.sql.push_str(" AND ");
                } else {
                    self.has_where = true
                }
                self.sql.push_str(" WHERE ");
                self.sql.push_str(column_name);
                self.sql.push_str(" like %");
                self.sql.push_str(&mask);
                self.sql.push('%');
                self
            }
        }
    }

    pub fn page(&mut self, page: i32, page_size: i32) -> &mut Self {
        self.sql.push_str(" LIMIT ");
        self.sql.push_str(&page_size.to_string());
        self.sql.push_str(" OFFSET ");
        self.sql.push_str(&(page_size * page).to_string());
        self
    }

    pub fn sql(&self) -> String {
        self.sql.clone()
    }
}