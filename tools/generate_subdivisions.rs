use std::{
    collections::{BTreeMap, BTreeSet},
    env,
    error::Error,
    fmt::Write as _,
    fs,
    path::PathBuf,
};

const CLDR_VERSION: &str = "48.2";

#[derive(Debug)]
struct Subdivision {
    code: String,
    name: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args_os().skip(1);
    let validity_path = PathBuf::from(
        args.next()
            .ok_or("usage: generate_subdivisions <validity.xml> <en.xml> <output.rs>")?,
    );
    let names_path = PathBuf::from(
        args.next()
            .ok_or("usage: generate_subdivisions <validity.xml> <en.xml> <output.rs>")?,
    );
    let output_path = PathBuf::from(
        args.next()
            .ok_or("usage: generate_subdivisions <validity.xml> <en.xml> <output.rs>")?,
    );

    if args.next().is_some() {
        return Err("generate_subdivisions accepts exactly three arguments".into());
    }

    let validity = fs::read_to_string(validity_path)?;
    let names = fs::read_to_string(names_path)?;
    let regular_codes = parse_regular_codes(&validity)?;
    let english_names = parse_english_names(&names)?;

    let mut subdivisions = regular_codes
        .into_iter()
        .map(|identifier| {
            let name = english_names
                .get(&identifier)
                .ok_or_else(|| format!("missing English name for {identifier}"))?;
            Ok(Subdivision {
                code: iso_code(&identifier)?,
                name: name.clone(),
            })
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;
    subdivisions.sort_unstable_by(|left, right| left.code.cmp(&right.code));

    let generated = render(&subdivisions)?;
    fs::write(output_path, generated)?;
    Ok(())
}

fn parse_regular_codes(xml: &str) -> Result<BTreeSet<String>, Box<dyn Error>> {
    let (_, after_status) = xml
        .split_once("idStatus='regular'")
        .ok_or("regular subdivision section not found")?;
    let (_, after_count_start) = after_status
        .split_once("<!--")
        .ok_or("regular subdivision count not found")?;
    let (count, _) = after_count_start
        .split_once(" items")
        .ok_or("regular subdivision count is incomplete")?;
    let expected_count = count.trim().parse::<usize>()?;
    let (_, after_opening_tag) = after_status
        .split_once('>')
        .ok_or("regular subdivision opening tag is incomplete")?;
    let (body, _) = after_opening_tag
        .split_once("</id>")
        .ok_or("regular subdivision closing tag not found")?;
    let body = strip_xml_comments(body)?;
    let mut codes = BTreeSet::new();

    for token in body.split_whitespace() {
        if let Some((start, end)) = token.split_once('~') {
            for code in expand_range(start, end)? {
                codes.insert(code);
            }
        } else {
            codes.insert(token.to_owned());
        }
    }

    if codes.len() != expected_count {
        return Err(format!(
            "expected {expected_count} regular subdivisions, found {}",
            codes.len()
        )
        .into());
    }

    Ok(codes)
}

fn strip_xml_comments(mut input: &str) -> Result<String, Box<dyn Error>> {
    let mut output = String::with_capacity(input.len());

    while let Some((before, after_start)) = input.split_once("<!--") {
        output.push_str(before);
        let (_, after_end) = after_start
            .split_once("-->")
            .ok_or("unterminated XML comment")?;
        input = after_end;
    }
    output.push_str(input);

    Ok(output)
}

fn expand_range(start: &str, abbreviated_end: &str) -> Result<Vec<String>, Box<dyn Error>> {
    if abbreviated_end.len() != 1 || start.is_empty() {
        return Err(format!("unsupported CLDR range {start}~{abbreviated_end}").into());
    }

    let (start_byte, prefix) = start
        .as_bytes()
        .split_last()
        .ok_or("CLDR range start is empty")?;
    let prefix = core::str::from_utf8(prefix)?;
    let end_byte = abbreviated_end
        .as_bytes()
        .first()
        .copied()
        .ok_or("CLDR range end is empty")?;
    let both_digits = start_byte.is_ascii_digit() && end_byte.is_ascii_digit();
    let both_letters = start_byte.is_ascii_lowercase() && end_byte.is_ascii_lowercase();

    if !(both_digits || both_letters) || *start_byte > end_byte {
        return Err(format!("invalid CLDR range {start}~{abbreviated_end}").into());
    }

    Ok((*start_byte..=end_byte)
        .map(|last| format!("{prefix}{}", char::from(last)))
        .collect())
}

fn parse_english_names(xml: &str) -> Result<BTreeMap<String, String>, Box<dyn Error>> {
    let mut names = BTreeMap::new();

    for line in xml.lines() {
        let Some((_, after_type)) = line.split_once("<subdivision type=\"") else {
            continue;
        };
        let (identifier, after_identifier) = after_type
            .split_once('"')
            .ok_or("subdivision type attribute is incomplete")?;
        let (_, after_opening_tag) = after_identifier
            .split_once('>')
            .ok_or("subdivision opening tag is incomplete")?;
        let (name, _) = after_opening_tag
            .split_once("</subdivision>")
            .ok_or("subdivision element is incomplete")?;
        let name = unescape_xml(name)?;

        if names.insert(identifier.to_owned(), name).is_some() {
            return Err(format!("duplicate English name for {identifier}").into());
        }
    }

    Ok(names)
}

fn unescape_xml(value: &str) -> Result<String, Box<dyn Error>> {
    let without_known_entities = value
        .replace("&amp;", "")
        .replace("&quot;", "")
        .replace("&apos;", "")
        .replace("&lt;", "")
        .replace("&gt;", "");

    if without_known_entities.contains('&') {
        return Err(format!("unsupported XML entity in {value}").into());
    }

    Ok(value
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">"))
}

fn iso_code(identifier: &str) -> Result<String, Box<dyn Error>> {
    if identifier.len() < 3 || !identifier.is_ascii() {
        return Err(format!("invalid CLDR subdivision identifier {identifier}").into());
    }

    let (country, subdivision) = identifier.split_at(2);
    Ok(format!(
        "{}-{}",
        country.to_ascii_uppercase(),
        subdivision.to_ascii_uppercase()
    ))
}

fn render(subdivisions: &[Subdivision]) -> Result<String, Box<dyn Error>> {
    let mut output = String::new();
    writeln!(
        output,
        "// Generated from Unicode CLDR {CLDR_VERSION}. Do not edit manually."
    )?;
    writeln!(output, "// SPDX-License-Identifier: Unicode-3.0")?;
    writeln!(output)?;
    writeln!(
        output,
        "pub(super) const CLDR_VERSION: &str = {CLDR_VERSION:?};"
    )?;
    writeln!(output)?;
    writeln!(
        output,
        "pub(super) static SUBDIVISIONS: [Subdivision; {}] = [",
        subdivisions.len()
    )?;

    for subdivision in subdivisions {
        writeln!(
            output,
            "    Subdivision {{ code: {:?}, name: {:?} }},",
            subdivision.code, subdivision.name
        )?;
    }

    writeln!(output, "];")?;
    Ok(output)
}
