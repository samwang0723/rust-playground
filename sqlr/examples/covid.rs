use anyhow::Result;
use sqlr::query;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";

    let sql = format!(
        "SELECT count(location) c, total_cases, new_cases, total_deaths, new_deaths FROM {} where new_deaths >= 20 ORDER BY new_cases DESC",
        url
   );

    let df = query(sql).await?;
    println!("{:?}", df);

    Ok(())
}
