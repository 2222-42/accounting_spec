use crate::domain::entity::{Sales, Section, Term, TermStatus};
use crate::domain::repository::{SalesRepository, SectionRepository, TermRepository};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Default)]
pub struct InMemorySectionRepository {
    storage: HashMap<Uuid, Section>,
}

impl InMemorySectionRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

impl SectionRepository for InMemorySectionRepository {
    fn save(&mut self, section: Section) -> Result<(), String> {
        self.storage.insert(section.id, section);
        Ok(())
    }

    fn find_by_id(&self, id: &Uuid) -> Option<Section> {
        self.storage.get(id).cloned()
    }
}

#[derive(Default)]
pub struct InMemoryTermRepository {
    storage: HashMap<Uuid, Term>,
}

impl InMemoryTermRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TermRepository for InMemoryTermRepository {
    fn save(&mut self, term: Term) -> Result<(), String> {
        self.storage.insert(term.id, term);
        Ok(())
    }

    fn find_by_id(&self, id: &Uuid) -> Option<Term> {
        self.storage.get(id).cloned()
    }

    // Performance Note: Linear search. In production, consider an index or caching the open term.
    fn find_open_term(&self) -> Option<Term> {
        self.storage
            .values()
            .find(|t| t.status == TermStatus::Open)
            .cloned()
    }
}

#[derive(Default)]
pub struct InMemorySalesRepository {
    storage: HashMap<Uuid, Sales>,
}

impl InMemorySalesRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

impl SalesRepository for InMemorySalesRepository {
    fn save(&mut self, sales: Sales) -> Result<(), String> {
        self.storage.insert(sales.id, sales);
        Ok(())
    }

    fn find_by_id(&self, id: &Uuid) -> Option<Sales> {
        self.storage.get(id).cloned()
    }

    // Performance Note: Linear scan. In production, add an index on term_id.
    fn find_by_term(&self, term_id: &Uuid) -> Vec<Sales> {
        self.storage
            .values()
            .filter(|s| s.term_id == *term_id)
            .cloned()
            .collect()
    }

    // Performance Note: Linear scan. In production, add a composite index on (section_id, term_id).
    fn find_by_section_and_term(&self, section_id: &Uuid, term_id: &Uuid) -> Vec<Sales> {
        self.storage
            .values()
            .filter(|s| s.section_id == *section_id && s.term_id == *term_id)
            .cloned()
            .collect()
    }
}
