# Accounting Specification

## Overview
This specification defines the rules and logic for the accounting system, focusing on sales management, organizational hierarchy, and term-based aggregation.

## Business Rules

### 1. Sales Registration
- All sales must be associated with a valid **Section** and **Term**.
- Sales amount can be positive or negative (returns/corrections).
- Sales date must fall within the associated Term's start and end dates.

### 2. Organizational Structure
- Sales belong to the lowest level Section (e.g., a specific team or unit).
- Aggregation can be performed at higher levels (Division, Department) by summing sales of child sections.

### 3. Allocation and Adjustment
- **Allocation**: A sale in one section can be allocated to multiple other sections. The total allocated amount must match the source amount.
- **Adjustment**: Corrections to sales figures should be made via adjustment entries, preserving the original record.

### 4. Term Management
- **Open Term**: Sales can be freely registered and modified.
- **Closed Term**: Sales cannot be modified directly. Corrections must be made via specific "Correction" actions.

## Edge Cases & Special Handling

### End of Term Corrections (Failed Aggregation)
As described in the requirements, there are cases where sales are "failed to be aggregated" or need late adjustment after calculation.

**Scenario**: A term is closed or aggregation has run, but a discrepancy is found.
**Resolution**:
1. The Accounter identifies the incorrect amount.
2. Two entries are created to "balance" the books without altering closed records:
    - **Negative Entry**: Negates the incorrect portion or transfers it out.
    - **Positive Entry**: Re-enters the correct amount or transfers it in.
3. **Sum to 0**: If this is a reclassification (e.g., wrong section), the sum of the negative and positive entries across the system is 0.

**Example**:
- Sale of $100 recorded in Section A (Incorrect).
- Correction:
    - Entry 1: -$100 in Section A.
    - Entry 2: +$100 in Section B (Correct Section).
- Net change to system: $0.
- Section A Net: $0.
- Section B Net: $100.

## Aggregation Logic
- **Term Total** = Sum(Normal Sales) + Sum(Adjustments) + Sum(Corrections).
- Aggregation must respect the hierarchy: `Department Total = Sum(Division Totals)`.

## Formal Verification & Constraints

To ensure the correctness of the accounting system, we define the following invariants which should be formally verified (e.g., using TLA+ or property-based testing).

### Invariants

1.  **Conservation of Money (Double Entry Principle)**
    *   For any `rebalance_term` or `transform_sales` operation, the sum of all created/modified entries must equal the original amount (or sum to 0 if it's a correction).
    *   `Sum(New Entries) == Sum(Old Entries)`

2.  **Term Integrity**
    *   `Term.start_date <= Term.end_date`
    *   Sales cannot be registered to a `Closed` term (unless via specific Correction workflow).
    *   `Sales.date` must be within `[Term.start_date, Term.end_date]`.

3.  **Section Hierarchy**
    *   A Section cannot be its own parent (no cycles).
    *   Sales are only registered to leaf sections (SectionType::Section), not aggregate levels (Department/Division), unless specified otherwise.

4.  **Sales Validity**
    *   `Sales.amount` must be a valid decimal.
    *   `AllocationRatio` must be between 0 and 1 inclusive.
