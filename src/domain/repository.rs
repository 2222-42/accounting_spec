use super::entity::{Sales, Section, Term};
use uuid::Uuid;

pub trait SectionRepository {
    fn save(&mut self, section: Section) -> Result<(), String>;
    fn find_by_id(&self, id: &Uuid) -> Option<Section>;
}

pub trait TermRepository {
    fn save(&mut self, term: Term) -> Result<(), String>;
    fn find_by_id(&self, id: &Uuid) -> Option<Term>;
    fn find_open_term(&self) -> Option<Term>;
}

pub trait SalesRepository {
    fn save(&mut self, sales: Sales) -> Result<(), String>;
    fn find_by_id(&self, id: &Uuid) -> Option<Sales>;
    fn find_by_term(&self, term_id: &Uuid) -> Vec<Sales>;
    fn find_by_section_and_term(&self, section_id: &Uuid, term_id: &Uuid) -> Vec<Sales>;
}
