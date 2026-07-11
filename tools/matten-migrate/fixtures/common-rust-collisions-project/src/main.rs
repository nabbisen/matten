use matten_data::Table;
use std::path::Path;

fn main() -> Result<(), matten_data::MattenDataError> {
    let csv = "name,value\nnorth,1";
    let _table = Table::from_csv_str(csv)?;
    let _joined = Path::new("reports").join("daily.csv");
    log::trace!("loaded fixture path");
    Ok(())
}
