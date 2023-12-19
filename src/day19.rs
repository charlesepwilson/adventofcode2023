use std::collections::HashMap;
use crate::utils::Solves;

pub struct Solution;

impl Solves for Solution {
    const DAY: u32 = 19;
    type ParsedInput = (HashMap<String, Vec<String>>, Vec<HashMap<String, u32>>);
    type Output = u32;

    fn parse_input(dir: &str) -> Self::ParsedInput {
        let mut workflows = HashMap::new();
        let all_lines: Vec<_> = Self::read_file(dir).collect();
        let mut sections = all_lines.split(|line| line.is_empty());
        let workflows_section = sections.next().unwrap();
        for line in workflows_section {
            let (name, mut instructions) = line.split_once("{").unwrap();
            instructions = instructions.trim_matches('}');
            let conditions_mapping: Vec<_> = instructions.split(',').map(|x| x.to_string()).collect();
            workflows.insert(name.to_string(), conditions_mapping);
        }
        let data = sections.next().unwrap();
        let mut data_vec = Vec::new();
        for line in data {
            let mut data_map = HashMap::new();
            for attribute in line.trim_matches('{').trim_matches('}').split(',') {
                let (label, value_str) = attribute.split_once('=').unwrap();
                let value: u32 = value_str.parse().unwrap();
                data_map.insert(label.to_string(), value);
            }
            data_vec.push(data_map);
        }
        (workflows, data_vec)
    }

    fn part1(dir: &str) -> Self::Output {
        let (workflows, data) = Self::parse_input(dir);
        let accepted = apply_workflows(workflows, data);
        score(accepted)
    }

    fn part2(dir: &str) -> Self::Output {
        let input = Self::parse_input(dir);
        0
    }
}

fn apply_workflows(workflows: HashMap<String, Vec<String>>, data: Vec<HashMap<String, u32>>) -> Vec<HashMap<String, u32>> {
    let mut accepted_data = Vec::new();
    for d in data {
        let mut next_label = "in";
        let mut label;
        while let Some(wf) = workflows.get(next_label) {
            label = apply_workflow(wf, &d);
            next_label = label.as_str();
        }
        if next_label == "A" {
            accepted_data.push(d);
        }
    }
    accepted_data
}

fn apply_workflow(workflow: &Vec<String>, data: &HashMap<String, u32>) -> String {
    for option in workflow {
        if let Some((test, target)) = option.split_once(':') {
            if let Some((label, test_value_str)) = test.split_once('>') {
                if let Some(value) = data.get(label) {
                    let test_value: u32 = test_value_str.parse().unwrap();
                    if *value > test_value {return target.to_string();}
                }
            }
            else if let Some((label, test_value_str)) = test.split_once('<') {
                if let Some(value) = data.get(label) {
                    let test_value: u32 = test_value_str.parse().unwrap();
                    if *value < test_value {return target.to_string();}
                }
            }
        }
        else { return option.to_string(); }
    }
    panic!()
}

fn score(data: Vec<HashMap<String, u32>>) -> u32 {
    let mut total = 0;
    for item in data {
        for value in item.values() {
            total += *value;
        }
    }
    total
}
