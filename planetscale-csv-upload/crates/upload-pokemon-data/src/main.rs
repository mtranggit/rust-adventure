fn main() -> Result<(), csv::Error> {
    let mut reader = csv::Reader::from_path("./crates/upload-pokemon-data/pokemon.csv")?;
    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}
