extern crate restson;
extern crate csv;
#[macro_use]
extern crate serde_derive;

use std::error::Error;
use csv::Reader;
use restson::{RestClient,RestPath};

static CSV_FILENAME: &str = "./data/word_list.csv";
static ROWS_MAX: usize = 300;

#[derive(Serialize,Deserialize, Debug)]
#[serde(untagged)]
enum TermApiResponse {
    Array(Vec<TermApi>)
}

#[derive(Serialize,Deserialize, Debug)]
struct TermApi {
    uuid: String,
    scheduled_for: String,
    scheduling_interval: u32,
    term_a: String,
    term_b: String,
    meta: TermApiMeta
}

#[derive(Serialize,Deserialize, Debug)]
struct TermApiMeta {
    core_index: String,
    jlpt: String,
    new_opt_voc_index: String,
    opt_sen_index: String,
    opt_voc_index: String,
    sent_ko_index: String,
    sentence_cloze: String,
    sentence_expression: String,
    sentence_furigana: String,
    sentence_kana: String,
    sentence_meaning: String,
    vocab_expression: String,
    vocab_furigana: String,
    vocab_ko_index: String,
    vocab_pos: String
}

impl RestPath<()> for TermApiResponse {
    fn get_path(_: ()) -> Result<String,restson::Error> { Ok(String::from("/dev/terms")) }
}

#[derive(Deserialize, Debug)]
struct TermCsv {
    core_index: u32,
    vocab_ko_index: u32,
    sent_ko_index: u32,
    new_opt_voc_index: u32,
    opt_voc_index: u32,
    opt_sen_index: u32,
    jlpt: String,
    vocab_expression: String,
    vocab_kana: String,
    vocab_meaning: String,
    vocab_sound_local: String,
    vocab_pos: String,
    sentence_expression: String,
    sentence_kana: String,
    sentence_meaning: String,
    sentence_sound_local: String,
    sentence_image_local: String,
    vocab_furigana: String,
    sentence_furigana: String,
    sentence_cloze: String
}

fn main() {
    // Load expected terms from CSV
    let mut expected_terms: Vec<TermCsv> = Vec::new();
    read_csv(&mut expected_terms).expect("Could not read CSV");

    // Load current terms from API
    let mut client = RestClient::new("https://dyvnth6y4j.execute-api.us-east-1.amazonaws.com").unwrap();
    let data: TermApiResponse = client.get(()).unwrap();

    println!("{:?}", data);
}

fn read_csv(terms: &mut Vec<TermCsv>) -> Result<(), Box<dyn Error>> {
    let mut reader = Reader::from_path(CSV_FILENAME)?;

    for (i, row) in reader.records().enumerate() {
        let term: TermCsv = row?.deserialize(None)?;
        terms.push(term);

        if (i+1) == ROWS_MAX {
            break;
        }
    }

    Ok(())
}