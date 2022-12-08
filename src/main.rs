mod data;

use std::collections::{btree_map::Entry, HashMap};

use data::get_string_from_vedlegg;

#[derive(Debug, Clone)]
struct SegmentData {
    pub date: String,
    pub hour_span: [i32; 2],
    pub cost: f32,
    pub usage: i32,
}

fn parse_data_segment(segment: &str) -> SegmentData {
    let data_strings: Vec<&str> = segment.split("\t").collect();
    let date = data_strings[0].to_owned().replace("\n", "");

    let hour_span_vec: Vec<&str> = data_strings[1].split("-").collect();
    let hour_span = [
        hour_span_vec[0].to_owned().clone().parse::<i32>().unwrap(),
        hour_span_vec[1].to_owned().clone().parse::<i32>().unwrap(),
    ];

    let cost = data_strings[2].parse::<f32>().unwrap();

    let usage = data_strings[3].parse::<i32>().unwrap();

    SegmentData {
        date,
        hour_span,
        cost,
        usage,
    }
}

#[derive(Debug, Clone)]
struct DailySummary {
    pub date: String,
    pub cost: f32,
    pub usage: i32,
}

fn summerize_days(data: Vec<SegmentData>) -> Vec<DailySummary> {
    let mut data_by_dates: HashMap<String, Vec<SegmentData>> = HashMap::new();
    for segment in data {
        if data_by_dates.contains_key(&segment.date) {
            let tmp = data_by_dates.get_mut(&segment.date).unwrap();
            tmp.push(segment);
            continue;
        }

        let date = segment.date.clone();
        let date_vec = vec![segment];
        data_by_dates.insert(date, date_vec);
    }

    let mut result: Vec<DailySummary> = vec![];

    for data_vec in data_by_dates.values().into_iter() {
        let date = data_vec[0].date.clone();
        let mut cost: f32 = 0.0;
        let mut usage: i32 = 0;

        for data in data_vec {
            cost += data.cost * data.usage as f32;
            usage += data.usage;
        }

        result.push(DailySummary { date, cost, usage })
    }

    result
}

fn main() {
    let vedlegg_txt = get_string_from_vedlegg();

    let segments: Vec<&str> = vedlegg_txt.split("\r").collect();

    let parsed_segments: Vec<SegmentData> = segments
        .into_iter()
        .map(|segment| parse_data_segment(segment))
        .collect();

    let summarized_data = summerize_days(parsed_segments);

    let mut lowest_expense_day: DailySummary = summarized_data[0].clone();

    for entry in &summarized_data {
        if lowest_expense_day.cost > entry.cost {
            lowest_expense_day = entry.clone();
        }
    }

    summarized_data.iter().for_each(|summary| {
        println!("{:?}", &summary);
    });

    println!(
        "The day with the lowest cost was: {}",
        &lowest_expense_day.date
    );
    println!(
        "That day the cost was:            {}",
        &lowest_expense_day.cost
    );
    println!(
        "And the amount of power used was: {}",
        &lowest_expense_day.usage
    );
    println!(
        "{}-{}-{}",
        lowest_expense_day.date,
        lowest_expense_day.usage,
        lowest_expense_day.cost.round() as i32
    )
}
