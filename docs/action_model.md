# Action Model

## Core Actions

### Register Sales
Record a new sales transaction.
- **Input**: Amount, Date, SectionID.
- **Output**: SalesID.
- **Rules**:
    - Must belong to an open Term.
    - Section must exist.

### Transform Sales (Transfer)
Move sales from one section to another.
- **Input**: SalesID, TargetSectionID.
- **Effect**:
    - Original Sales marked as transferred (or negated).
    - New Sales created for TargetSectionID.
- **Use Case**: Reorganization or correction of attribution.

### Adjust Sales
Modify the amount of an existing sale.
- **Input**: SalesID, NewAmount or Delta.
- **Effect**:
    - Creates an adjustment entry linked to the original sale.
    - Does not delete the original record (audit trail).

### Allocate Sales
Distribute a sale's value across multiple sections.
- **Input**: SalesID, List of (TargetSectionID, Ratio).
- **Effect**:
    - Creates allocation records.
    - Sum of allocated amounts must equal original amount (or specified portion).

### Close Term (Aggregation)
Finalize the term and calculate totals.
- **Input**: TermID.
- **Effect**:
    - Locks the term for normal registration.
    - Calculates totals per Section.

### Correct Term
Handle "failed aggregation" or late changes after a term is effectively closed or calculated.
- **Input**: TermID, SectionID, OriginalAmount, CorrectAmount.
- **Effect**:
    - Creates a pair of entries:
        1. Negative Sales to offset the incorrect amount.
        2. Positive Sales to establish the correct amount.
    - Ensures the sum of corrections is 0 if it's a rebalancing, or reflects the net change.
    - **Note**: As per issue #1, "accounter makes negative sales and positive sales, that sums to 0" implies a reclassification or correction that balances out.
