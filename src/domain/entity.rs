use super::value_object::Money;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SectionType {
    Department,
    Division,
    Section,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    pub id: Uuid,
    pub name: String,
    pub section_type: SectionType,
    pub parent_id: Option<Uuid>,
}

impl Section {
    pub fn new(name: String, section_type: SectionType, parent_id: Option<Uuid>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            section_type,
            parent_id,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TermStatus {
    Open,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Term {
    pub id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub status: TermStatus,
}

impl Term {
    pub fn new(start_date: NaiveDate, end_date: NaiveDate) -> Self {
        Self {
            id: Uuid::new_v4(),
            start_date,
            end_date,
            status: TermStatus::Open,
        }
    }

    pub fn close(&mut self) {
        self.status = TermStatus::Closed;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SalesType {
    Normal,
    Adjustment,
    Correction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sales {
    pub id: Uuid,
    pub amount: Money,
    pub date: NaiveDateTime,
    pub section_id: Uuid,
    pub term_id: Uuid,
    pub sales_type: SalesType,
    pub related_sales_id: Option<Uuid>, // For adjustments/allocations
}

impl Sales {
    pub fn new(
        amount: Money,
        date: NaiveDateTime,
        section_id: Uuid,
        term_id: Uuid,
        sales_type: SalesType,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            amount,
            date,
            section_id,
            term_id,
            sales_type,
            related_sales_id: None,
        }
    }
}
