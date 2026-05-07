use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap_or_else(|e| {
        eprintln!("Failed to read stdin: {e}");
        std::process::exit(1);
    });

    let spec: serde_json::Value = serde_json::from_str(&input).unwrap_or_else(|e| {
        eprintln!("Invalid JSON: {e}");
        std::process::exit(1);
    });

    if let Ok(parsed) =
        serde_json::from_value::<bert_lib::intermediate::IntermediateSpec>(spec.clone())
    {
        let errors = bert_lib::intermediate::validate_intermediate(&parsed);
        if !errors.is_empty() {
            for e in &errors {
                eprintln!("validation: {e}");
            }
            std::process::exit(1);
        }
    }

    let mut gen = bert_lib::generator::BertModelGenerator::new(spec);
    let model = gen.generate();

    println!("{}", serde_json::to_string_pretty(&model).unwrap());
}
