use crate::domain::entity::{Sales, SalesType, Section, Term};
use crate::domain::repository::{SalesRepository, SectionRepository, TermRepository};
use crate::domain::value_object::Money;
use chrono::NaiveDateTime;
use uuid::Uuid;

pub struct AccountingService<S, T, L>
where
    S: SectionRepository,
    T: TermRepository,
    L: SalesRepository,
{
    section_repo: S,
    term_repo: T,
    sales_repo: L,
}

impl<S, T, L> AccountingService<S, T, L>
where
    S: SectionRepository,
    T: TermRepository,
    L: SalesRepository,
{
    pub fn new(section_repo: S, term_repo: T, sales_repo: L) -> Self {
        Self {
            section_repo,
            term_repo,
            sales_repo,
        }
    }

    pub fn create_section(&mut self, section: Section) -> Result<Uuid, String> {
        let id = section.id;
        self.section_repo.save(section)?;
        Ok(id)
    }

    pub fn create_term(&mut self, term: Term) -> Result<Uuid, String> {
        let id = term.id;
        self.term_repo.save(term)?;
        Ok(id)
    }

    pub fn register_sales(
        &mut self,
        amount: Money,
        date: NaiveDateTime,
        section_id: Uuid,
    ) -> Result<Uuid, String> {
        // 1. Validate Section
        if self.section_repo.find_by_id(&section_id).is_none() {
            return Err("Section not found".to_string());
        }

        // 2. Validate Term (Must be open)
        let term = self
            .term_repo
            .find_open_term()
            .ok_or("No open term found")?;

        if date.date() < term.start_date || date.date() > term.end_date {
            return Err("Date is outside of the current term".to_string());
        }

        if amount.amount().is_zero() {
            return Err("Sales amount cannot be zero".to_string());
        }

        // 3. Create Sales
        let sales = Sales::new(amount, date, section_id, term.id, SalesType::Normal);
        let id = sales.id;
        self.sales_repo.save(sales)?;

        Ok(id)
    }

    pub fn transform_sales(
        &mut self,
        sales_id: Uuid,
        target_section_id: Uuid,
        date: NaiveDateTime,
    ) -> Result<Uuid, String> {
        let original_sales = self
            .sales_repo
            .find_by_id(&sales_id)
            .ok_or("Sales not found")?;

        if self.section_repo.find_by_id(&target_section_id).is_none() {
            return Err("Target section not found".to_string());
        }

        // Create negative sales for source
        let mut negative_sales = Sales::new(
            -original_sales.amount,
            date,
            original_sales.section_id,
            original_sales.term_id,
            SalesType::Adjustment,
        );
        negative_sales.related_sales_id = Some(sales_id);
        self.sales_repo.save(negative_sales)?;

        // Create positive sales for target
        let mut positive_sales = Sales::new(
            original_sales.amount,
            date,
            target_section_id,
            original_sales.term_id,
            SalesType::Adjustment,
        );
        positive_sales.related_sales_id = Some(sales_id);
        let new_id = positive_sales.id;
        self.sales_repo.save(positive_sales)?;

        Ok(new_id)
    }

    pub fn close_term(&mut self, term_id: Uuid) -> Result<(), String> {
        let mut term = self
            .term_repo
            .find_by_id(&term_id)
            .ok_or("Term not found")?;
        term.close();
        self.term_repo.save(term)?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn correct_term(
        &mut self,
        term_id: Uuid,
        section_id: Uuid,
        original_amount: Money,
        correct_amount: Money,
        date: NaiveDateTime,
    ) -> Result<(), String> {
        let term = self
            .term_repo
            .find_by_id(&term_id)
            .ok_or("Term not found")?;

        if self.section_repo.find_by_id(&section_id).is_none() {
            return Err("Section not found".to_string());
        }

        if date.date() < term.start_date || date.date() > term.end_date {
            return Err("Date is outside of the term".to_string());
        }

        // Even if closed, corrections are allowed but marked as Correction type

        // 1. Create reversal entry (negative of original)
        let reversal = Sales::new(
            -original_amount,
            date,
            section_id,
            term.id,
            SalesType::Correction,
        );
        self.sales_repo.save(reversal)?;

        // 2. Create correction entry (new correct amount)
        let correction = Sales::new(
            correct_amount,
            date,
            section_id,
            term.id,
            SalesType::Correction,
        );
        self.sales_repo.save(correction)?;

        Ok(())
    }

    pub fn rebalance_term(
        &mut self,
        term_id: Uuid,
        source_section_id: Uuid,
        target_section_id: Uuid,
        amount: Money,
        date: NaiveDateTime,
    ) -> Result<(), String> {
        let term = self
            .term_repo
            .find_by_id(&term_id)
            .ok_or("Term not found")?;

        if self.section_repo.find_by_id(&source_section_id).is_none() {
            return Err("Source section not found".to_string());
        }
        if self.section_repo.find_by_id(&target_section_id).is_none() {
            return Err("Target section not found".to_string());
        }

        if date.date() < term.start_date || date.date() > term.end_date {
            return Err("Date is outside of the term".to_string());
        }

        // Negative for source
        let source_correction = Sales::new(
            -amount,
            date,
            source_section_id,
            term.id,
            SalesType::Correction,
        );
        self.sales_repo.save(source_correction)?;

        // Positive for target
        let target_correction = Sales::new(
            amount,
            date,
            target_section_id,
            term.id,
            SalesType::Correction,
        );
        self.sales_repo.save(target_correction)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entity::SectionType;
    use crate::infrastructure::in_memory::{
        InMemorySalesRepository, InMemorySectionRepository, InMemoryTermRepository,
    };
    use chrono::NaiveDate;
    use rust_decimal::Decimal;
    use std::str::FromStr;

    #[test]
    fn test_register_sales_success() {
        let mut service = AccountingService::new(
            InMemorySectionRepository::new(),
            InMemoryTermRepository::new(),
            InMemorySalesRepository::new(),
        );

        let section = Section::new("Test Section".to_string(), SectionType::Section, None);
        let section_id = service.create_section(section).unwrap();

        let term = Term::new(
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(),
        );
        service.create_term(term).unwrap();

        let amount = Money::new(Decimal::from_str("100.00").unwrap());
        let date = NaiveDate::from_ymd_opt(2025, 6, 1)
            .unwrap()
            .and_hms_opt(10, 0, 0)
            .unwrap();

        let result = service.register_sales(amount, date, section_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_register_sales_outside_term() {
        let mut service = AccountingService::new(
            InMemorySectionRepository::new(),
            InMemoryTermRepository::new(),
            InMemorySalesRepository::new(),
        );

        let section = Section::new("Test Section".to_string(), SectionType::Section, None);
        let section_id = service.create_section(section).unwrap();

        let term = Term::new(
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(),
        );
        service.create_term(term).unwrap();

        let amount = Money::new(Decimal::from_str("100.00").unwrap());
        let date = NaiveDate::from_ymd_opt(2024, 12, 31)
            .unwrap()
            .and_hms_opt(10, 0, 0)
            .unwrap(); // Outside

        let result = service.register_sales(amount, date, section_id);
        assert!(result.is_err());
    }
    #[test]
    fn test_correct_term_success() {
        let mut service = AccountingService::new(
            InMemorySectionRepository::new(),
            InMemoryTermRepository::new(),
            InMemorySalesRepository::new(),
        );

        let section = Section::new("Test Section".to_string(), SectionType::Section, None);
        let section_id = service.create_section(section).unwrap();

        let term = Term::new(
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(),
        );
        let term_id = service.create_term(term).unwrap();
        service.close_term(term_id).unwrap();

        let original_amount = Money::new(Decimal::from_str("100.00").unwrap());
        let correct_amount = Money::new(Decimal::from_str("150.00").unwrap());
        let date = NaiveDate::from_ymd_opt(2025, 6, 1)
            .unwrap()
            .and_hms_opt(10, 0, 0)
            .unwrap();

        let result =
            service.correct_term(term_id, section_id, original_amount, correct_amount, date);
        assert!(result.is_ok());
    }

    #[test]
    fn test_correct_term_invalid_section() {
        let mut service = AccountingService::new(
            InMemorySectionRepository::new(),
            InMemoryTermRepository::new(),
            InMemorySalesRepository::new(),
        );

        let term = Term::new(
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(),
        );
        let term_id = service.create_term(term).unwrap();
        let invalid_section_id = Uuid::new_v4();

        let original_amount = Money::new(Decimal::from_str("100.00").unwrap());
        let correct_amount = Money::new(Decimal::from_str("150.00").unwrap());
        let date = NaiveDate::from_ymd_opt(2025, 6, 1)
            .unwrap()
            .and_hms_opt(10, 0, 0)
            .unwrap();

        let result = service.correct_term(
            term_id,
            invalid_section_id,
            original_amount,
            correct_amount,
            date,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_rebalance_term_success() {
        let mut service = AccountingService::new(
            InMemorySectionRepository::new(),
            InMemoryTermRepository::new(),
            InMemorySalesRepository::new(),
        );

        let section_a = Section::new("Section A".to_string(), SectionType::Section, None);
        let section_a_id = service.create_section(section_a).unwrap();
        let section_b = Section::new("Section B".to_string(), SectionType::Section, None);
        let section_b_id = service.create_section(section_b).unwrap();

        let term = Term::new(
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(),
        );
        let term_id = service.create_term(term).unwrap();

        let amount = Money::new(Decimal::from_str("100.00").unwrap());
        let date = NaiveDate::from_ymd_opt(2025, 6, 1)
            .unwrap()
            .and_hms_opt(10, 0, 0)
            .unwrap();

        let result = service.rebalance_term(term_id, section_a_id, section_b_id, amount, date);
        assert!(result.is_ok());
    }
}
