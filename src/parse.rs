use std::collections::HashMap;

use crate::reference::{FieldType, Reference, ReferenceType};

pub fn parse_bibtex(bibtex: String) -> Result<Vec<Reference>, String> {
    let mut references: Vec<Reference> = Vec::new();
    let mut fields: HashMap<FieldType, String> = HashMap::new();
    let mut key: String = String::new();
    let mut maybe_reference_type: Result<ReferenceType, String> =
        Err("Failed to parse reference type".to_owned());

    for line in bibtex.lines() {
        // Skip blank lines
        if line.trim().is_empty() {
            continue;
        }
        // @article{veerman2012,
        // ["@article", "veerman2012,"]
        // If the line starts with '@', we're parsing a key
        if line.starts_with('@') {
            let split = line.split('{').collect::<Vec<&str>>();
            maybe_reference_type = parse_reference_type(split[0]);
            key = split[1].split(',').collect::<Vec<&str>>()[0].to_string();
        // If the line starts with '}', we're done parsing this reference and can move on
        } else if line.trim_start().starts_with('}') {
            // Parsing the reference type ("@article", etc.) may have failed, in which case we return an Err
            match maybe_reference_type {
                Ok(ref reference_type) => {
                    let reference =
                        Reference::new(key.clone(), reference_type.to_owned(), fields.clone());
                    references.push(reference);
                    fields.clear();
                }
                Err(err) => return Err(err.to_owned()),
            }
        // Otherwise, we're parsing a field, which is the form "author = {hello}"
        } else {
            let field = line.split('=').collect::<Vec<&str>>();
            // If splitting on '=' failed, skip this field
            if field.len() < 2 {
                continue;
            }

            // Ignore fields we can't parse into a FieldType
            if let Ok(field_type) = FieldType::try_from(field[0].trim()) {
                let value = strip_optional_suffix(
                    strip_optional_suffix(strip_optional_prefix(field[1].trim(), "{"), ","),
                    "}",
                )
                .to_string();
                fields.insert(field_type, value);
            }
        }
    }

    Ok(references)
}

fn parse_reference_type(str: &str) -> Result<ReferenceType, String> {
    ReferenceType::try_from(strip_optional_prefix(str, "@"))
}

fn strip_optional_prefix<'a>(s: &'a str, prefix: &str) -> &'a str {
    if let Some(s1) = s.strip_prefix(prefix) {
        s1
    } else {
        s
    }
}

fn strip_optional_suffix<'a>(s: &'a str, suffix: &str) -> &'a str {
    if let Some(s1) = s.strip_suffix(suffix) {
        s1
    } else {
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bibtex() {
        {
            let test_bibtex = r#"@inproceedings{Alam&Riccardi2014,
              title     = {Predicting Personality Traits using Multimodal Information},
              author    = {Alam, Firoj and Riccardi, Giuseppe},
              year      = 2014,
              booktitle = {Proceedings of the 2014 ACM Multi Media on Workshop on Computational Personality Recognition},
              location  = {Orlando, Florida, USA},
              publisher = {Association for Computing Machinery},
              address   = {New York, NY, USA},
              series    = {WCPR '14},
              pages     = {15–18},
              doi       = {10.1145/2659522.2659531},
              isbn      = 9781450331296,
              url       = {https://doi.org/10.1145/2659522.2659531},
              abstract  = {Measuring personality traits has a long story in psychology where analysis has been done by asking sets of questions. These question sets (inventories) have been designed by investigating lexical terms that we use in our daily communications or by analyzing biological phenomena. Whether consciously or unconsciously we express our thoughts and behaviors when communicating with others, either verbally, non-verbally or using visual expressions. Recently, research in behavioral signal processing has focused on automatically measuring personality traits using different behavioral cues that appear in our daily communication. In this study, we present an approach to automatically recognize personality traits using a video-blog (vlog) corpus, consisting of transcription and extracted audio-visual features. We analyzed linguistic, psycholinguistic and emotional features in addition to the audio-visual features provided with the dataset. We also studied whether we can better predict a trait by identifying other traits. Using our best models we obtained very promising results compared to the official baseline.},
              numpages  = 4,
              keywords  = {behavioral signal processing, multimodal personality recognition}
          }"#;
            let mut fields = HashMap::new();
            fields.insert(
                FieldType::Title,
                String::from("Predicting Personality Traits using Multimodal Information"),
            );
            fields.insert(
                FieldType::Author,
                String::from("Alam, Firoj and Riccardi, Giuseppe"),
            );
            fields.insert(FieldType::Year, String::from("2014"));
            fields.insert(FieldType::Booktitle, String::from("Proceedings of the 2014 ACM Multi Media on Workshop on Computational Personality Recognition"));
            fields.insert(
                FieldType::Publisher,
                String::from("Association for Computing Machinery"),
            );
            fields.insert(FieldType::Address, String::from("New York, NY, USA"));
            fields.insert(FieldType::Series, String::from("WCPR '14"));
            fields.insert(FieldType::Pages, String::from("15–18"));
            fields.insert(FieldType::Doi, String::from("10.1145/2659522.2659531"));
            fields.insert(FieldType::Isbn, String::from("9781450331296"));
            fields.insert(
                FieldType::Url,
                String::from("https://doi.org/10.1145/2659522.2659531"),
            );
            fields.insert(FieldType::Abstract, String::from("Measuring personality traits has a long story in psychology where analysis has been done by asking sets of questions. These question sets (inventories) have been designed by investigating lexical terms that we use in our daily communications or by analyzing biological phenomena. Whether consciously or unconsciously we express our thoughts and behaviors when communicating with others, either verbally, non-verbally or using visual expressions. Recently, research in behavioral signal processing has focused on automatically measuring personality traits using different behavioral cues that appear in our daily communication. In this study, we present an approach to automatically recognize personality traits using a video-blog (vlog) corpus, consisting of transcription and extracted audio-visual features. We analyzed linguistic, psycholinguistic and emotional features in addition to the audio-visual features provided with the dataset. We also studied whether we can better predict a trait by identifying other traits. Using our best models we obtained very promising results compared to the official baseline."));
            fields.insert(
                FieldType::Keywords,
                String::from("behavioral signal processing, multimodal personality recognition"),
            );

            let expected = Reference::new(
                String::from("Alam&Riccardi2014"),
                ReferenceType::InProceedings,
                fields,
            );

            let parsed_bibtex = parse_bibtex(test_bibtex.to_string()).unwrap();
            let actual = parsed_bibtex
                .first()
                .expect("Failed to parse bibtex: parsing returned 0 references")
                .to_owned();

            assert_eq!(expected, actual);
        }
    }
}
