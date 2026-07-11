use matten_data::Table;

fn main() -> Result<(), matten_data::MattenDataError> {
    let csv = "region,sales,cost\nnorth,100,40\nsouth,150,";
    let _tensor = Table::from_csv_str(csv)?
        .select_columns(["sales", "cost"])?
        .fill_missing(0.0)?
        .try_numeric()?
        .to_tensor()?;
    Ok(())
}
