use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Grammar;

fn main() -> anyhow::Result<()> {
    let parsed_data = Grammar::parse(Rule::field, "-273.15")?;
    for record in parsed_data {
        println!("Parsed Record: {:?}", record);
        for inner in record.into_inner() {
            match inner.as_rule() {
                Rule::number => {
                    println!("Number: {}", inner.as_str());
                }
                _ => {}
            }
        }
    }

    Ok(())
}
