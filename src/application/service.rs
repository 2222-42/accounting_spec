use crate::domain::entity::{Sales, SalesType, Section, Term, TermStatus};
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

    pub fn correct_term(
        &mut self,
        term_id: Uuid,
        section_id: Uuid,
        amount_delta: Money,
        date: NaiveDateTime,
    ) -> Result<(), String> {
        let term = self
            .term_repo
            .find_by_id(&term_id)
            .ok_or("Term not found")?;

        // Even if closed, corrections are allowed but marked as Correction type
        
        // Create correction entry
        let correction = Sales::new(
            amount_delta,
            date,
            section_id,
            term.id,
            SalesType::Correction,
        );
        self.sales_repo.save(correction)?;
        
        // Note: The balancing entry (to make sum 0) is assumed to be handled by another call 
        // or this function should take a target section to balance against if it's a transfer.
        // If it's just a pure adjustment (e.g. found new revenue), it might not sum to 0 globally 
        // but the spec said "accounter makes negative sales and positive sales, that sums to 0".
        // Let's assume this function handles one side, and the caller manages the balance, 
        // OR we implement a `rebalance_term` function. 
        // For simplicity, let's implement `rebalance_term` which takes source and target.
        
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
