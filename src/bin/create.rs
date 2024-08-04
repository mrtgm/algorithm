use anyhow::{self, Context, Result};
use clap::Parser;
use std::{
    env,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Seek, Write},
    path::Path,
    str::FromStr,
};
use strum;

#[derive(Debug, strum::EnumString)]
enum Template {
    Function,
    AT,
}

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long, required = true)]
    category: String,
    #[arg(short, long, required = true)]
    file: String,
    #[arg(short, long, default_value = "Function")]
    mode: String,
}

fn create_template(mode: Template, filename: &str) -> String {
    match mode {
        Template::Function => {
            format!(
                r#"use std::fmt::Debug;

fn {filename}<T: Copy + Debug + Ord>(arr: &mut [T]) {{
}}

#[cfg(test)]
mod test {{
    use super::*;

    #[test]
    fn test_{filename}() {{
        let mut arr = [5, 2, 4, 6, 1, 3];
        {filename}(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }}
}}
        "#
            )
        }
        Template::AT => {
            format!(
                r#"#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num_traits::pow;
#[allow(unused_imports)]
use proconio::{{
    input,
    marker::{{Chars, Usize1}},
            }};
#[allow(unused_imports)]
use std::cmp::{{max, min}};
#[allow(unused_imports)]
use std::collections::{{HashMap, HashSet, VecDeque}};

fn main() {{
    input! {{
            }}
            }}

#[cfg(test)]
mod test {{
    use super::*;

    #[test]
    fn {filename}() {{
        main();
            }}
            }} "#
            )
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let category_path = Path::new("./src/bin/").join(&args.category);
    let category_file_path = category_path.with_extension("rs");
    let file_path = category_path.join(&args.file).with_extension("rs");
    let mode = Template::from_str(&args.mode).context("Invalid mode")?;

    let template = create_template(mode, &args.file);

    let category_file_template = format!(
        r#"mod {category} {{
        pub(super) mod {file};
}}
fn main() {{}}
"#,
        category = args.category,
        file = args.file,
    );

    if category_path.exists() {
        if file_path.exists() {
            println!("{} already exists", file_path.display());
        } else {
            if category_file_path.exists() {
                let mut category_file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open(&category_file_path)?; // OpenOptions で両方可能

                let reader = BufReader::new(&category_file);
                let mut v: Vec<String> = reader.lines().flat_map(|v| v).collect(); // Iter<Result<T>> を flat_map と collect で vec に

                category_file.rewind().unwrap(); //読んだ後はポインタを巻き戻し

                if let Some(target_line) = v.iter().enumerate().find(|(_, line)| "}" == *line) {
                    let (i, _) = target_line;
                    v.insert(i, format!("    pub(super) mod {};", args.file).to_string());
                    for l in v {
                        writeln!(category_file, "{}", l).with_context(|| "write fail")?;
                        // category_file
                        //     .write((l + "\n").as_bytes())
                        //     .with_context(|| "write fail")?;
                    }
                } else {
                    return Err(anyhow::anyhow!("}} not found"));
                }
            } else {
                let mut category_file = File::create(category_file_path)?;
                category_file.write_all(category_file_template.as_bytes())?;
            }

            let mut file = File::create(file_path)?;
            file.write_all(template.as_bytes())?;
        }
    } else {
        env::set_current_dir("./src/bin")?;
        std::fs::create_dir(&args.category)?;

        let mut category_file = File::create(category_file_path)?;
        category_file.write_all(category_file_template.as_bytes())?;

        let mut file = File::create(file_path)?;
        file.write_all(template.as_bytes())?;
    }

    Ok(())
}
