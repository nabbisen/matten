use matten_data::Table;

fn solve() -> Result<matten::Tensor, matten_data::MattenDataError> {
    let csv = "\
region,sales,cost
north,100,40
south,150,
east,120,55";
    Table::from_csv_str(csv)?
        .select_columns(["sales", "cost"])?
        .fill_missing(0.0)?
        .try_numeric()?
        .to_tensor()
}
