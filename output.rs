use crate::{CliError, CliResponse, ResponseContent};

pub fn write_response(res: &CliResponse, writer: &mut impl std::io::Write) -> std::io::Result<()> {
    let Some(content) = res.content() else {
        writeln!(writer, "✓ Success")?;
        return Ok(());
    };

    match content {
        ResponseContent::Message(msg) => {
            writeln!(writer, "✓ {}", msg)?;
        }
        ResponseContent::Record { record, .. } => {
            writeln!(writer, "✓ Record created:")?;
            writeln!(writer, "  ID: {}", record.id)?;
            writeln!(writer, "  Amount: {}", record.amount)?;
            // More formatting later
        }
        ResponseContent::List { records, .. } => {
            for record in records {
                writeln!(writer, "{:?}", record)?;
            }
        }
        ResponseContent::Total(total) => {
            writeln!(writer, "Opening Balance: {} {}", total.opening_balance, total.currency)?;
            writeln!(writer, "Total Income: {} {}", total.income_total, total.currency)?;
            writeln!(writer, "Total Expenses: {} {}", total.expenses_total, total.currency)?;
            let net_balance = total.opening_balance + total.income_total - total.expenses_total;
            writeln!(writer, "Net Balance: {} {}", net_balance, total.currency)?;
        }
        _ => {}
    }
    Ok(())
}

pub fn write_error(err: &CliError, writer: &mut impl std::io::Write) -> std::io::Result<()> {
    match err {
        CliError::FileNotFound(msg) => writeln!(writer, "Error: File not found: {}", msg),
        CliError::InvalidJson(msg) => writeln!(writer, "Error: Invalid JSON: {}", msg),
        CliError::ValidationError(kind) => {
            match kind {
                crate::ValidationErrorKind::AmountTooSmall { amount } => {
                    writeln!(writer, "Error: Amount must be greater than 0, got {}", amount)
                }
                crate::ValidationErrorKind::SubcategoryNotFound { name } => {
                    writeln!(writer, "Error: Subcategory '{}' not found", name)
                }
                crate::ValidationErrorKind::RecordNotFound { id } => {
                    writeln!(writer, "Error: Record with ID {} not found", id)
                }
                _ => writeln!(writer, "Error: Validation failed"),
            }
        }
        CliError::FileAlreadyExists => {
            writeln!(writer, "Error: Tracker already initialized. Use 'fintrack clear' to start fresh.")
        }
        _ => writeln!(writer, "Error: {}", err),
    }
}
