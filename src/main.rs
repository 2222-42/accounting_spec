mod application;
mod domain;
mod infrastructure;

use application::service::AccountingService;
use chrono::NaiveDate;
use domain::entity::{Section, SectionType, Term};
use domain::value_object::Money;
use infrastructure::in_memory::{
    InMemorySalesRepository, InMemorySectionRepository, InMemoryTermRepository,
};
use rust_decimal::Decimal;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Accounting System Demo...");

    // 1. Initialize Repositories
    let section_repo = InMemorySectionRepository::new();
    let term_repo = InMemoryTermRepository::new();
    let sales_repo = InMemorySalesRepository::new();

    // 2. Initialize Service
    let mut service = AccountingService::new(section_repo, term_repo, sales_repo);

    // 3. Create Sections
    let section_a = Section::new("Sales Dept A".to_string(), SectionType::Section, None);
    let section_b = Section::new("Sales Dept B".to_string(), SectionType::Section, None);

    let section_a_id = service.create_section(section_a)?;
    let section_b_id = service.create_section(section_b)?;
    println!(
        "Created Sections: A ({}), B ({})",
        section_a_id, section_b_id
    );

    // 4. Create Term
    let term = Term::new(
        NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        NaiveDate::from_ymd_opt(2025, 12, 31).unwrap(),
    );
    let term_id = service.create_term(term)?;
    println!("Created Term: {}", term_id);

    // 5. Register Sales
    let amount = Money::new(Decimal::from_str("1000.00")?);
    let date = NaiveDate::from_ymd_opt(2025, 6, 15)
        .unwrap()
        .and_hms_opt(10, 0, 0)
        .unwrap();

    let sales_id = service.register_sales(amount, date, section_a_id)?;
    println!("Registered Sales: {} of {}", sales_id, amount);

    // 6. Transform Sales (Transfer 500 from A to B)
    let transfer_date = NaiveDate::from_ymd_opt(2025, 6, 20)
        .unwrap()
        .and_hms_opt(14, 0, 0)
        .unwrap();
    // Note: The transform_sales method in service currently takes a sales_id and moves the WHOLE amount.
    // If we want to move partial, we'd need a different logic or split first.
    // For this demo, let's assume we transfer the whole sale to B.
    let new_sales_id = service.transform_sales(sales_id, section_b_id, transfer_date)?;
    println!(
        "Transferred Sales {} to Section B. New Sales ID: {}",
        sales_id, new_sales_id
    );

    // 7. Close Term
    service.close_term(term_id)?;
    println!("Closed Term: {}", term_id);

    // 8. Rebalance Term (Correction)
    // Move 100 from B back to A (oops, mistake)
    let correction_amount = Money::new(Decimal::from_str("100.00")?);
    let correction_date = NaiveDate::from_ymd_opt(2025, 12, 31)
        .unwrap()
        .and_hms_opt(23, 59, 59)
        .unwrap();

    service.rebalance_term(
        term_id,
        section_b_id,
        section_a_id,
        correction_amount,
        correction_date,
    )?;
    println!("Rebalanced Term: Moved {} from B to A", correction_amount);

    println!("Demo Completed Successfully.");
    Ok(())
}
