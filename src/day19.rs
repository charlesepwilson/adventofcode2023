use std::cmp::{max, min};
use std::collections::HashMap;
use std::str::FromStr;
use crate::utils::Solves;

pub struct Solution;

impl Solves for Solution {
    const DAY: u32 = 19;
    type ParsedInput = (HashMap<String, Vec<String>>, Vec<HashMap<String, u64>>);
    type Output = u64;

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
                let value: u64 = value_str.parse().unwrap();
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
        let (workflows, _) = Self::parse_input(dir);
        let constraints = find_constraints(workflows);
        constraints.into_iter().map(count_valid_possibilities).sum()
    }
}

const START: &str = "in";
const ACCEPTED: char = 'A';
const REJECTED: char = 'R';

fn apply_workflows(workflows: HashMap<String, Vec<String>>, data: Vec<HashMap<String, u64>>) -> Vec<HashMap<String, u64>> {
    let mut accepted_data = Vec::new();
    for d in data {
        let mut next_label = START;
        let mut label;
        while let Some(wf) = workflows.get(next_label) {
            label = apply_workflow(wf, &d);
            next_label = label.as_str();
        }
        if next_label.contains(ACCEPTED) {
            accepted_data.push(d);
        }
    }
    accepted_data
}

fn apply_workflow(workflow: &Vec<String>, data: &HashMap<String, u64>) -> String {
    for option in workflow {
        if let Some((test, target)) = option.split_once(':') {
            if let Some((label, test_value_str)) = test.split_once('>') {
                if let Some(value) = data.get(label) {
                    let test_value: u64 = test_value_str.parse().unwrap();
                    if *value > test_value {return target.to_string();}
                }
            }
            else if let Some((label, test_value_str)) = test.split_once('<') {
                if let Some(value) = data.get(label) {
                    let test_value: u64 = test_value_str.parse().unwrap();
                    if *value < test_value {return target.to_string();}
                }
            }
        }
        else { return option.to_string(); }
    }
    panic!()
}

fn score(data: Vec<HashMap<String, u64>>) -> u64 {
    let mut total = 0;
    for item in data {
        for value in item.values() {
            total += *value;
        }
    }
    total
}

#[derive(Debug, PartialEq, Eq)]
struct Condition {
    field: char,
    comparison: char,
    test_value: u64,
}

impl Condition {
    fn invert(self) -> Self {
        let comparison;
        let test_value;
        if self.comparison == '>'{
            comparison = '<';
            test_value = self.test_value + 1;
        }
        else {
            comparison = '>';
            test_value = self.test_value - 1;
        }
        Self {
            field: self.field,
            comparison,
            test_value,
        }

    }
}

#[derive(Debug, PartialEq, Eq)]
struct WorkflowOption {
    condition: Option<Condition>,
    target: String,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseWorkflowError;
impl FromStr for WorkflowOption {
    type Err = ParseWorkflowError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut wf = Self{
            condition: None,
            target: s.to_string(),
        };
        if let Some((test, target)) = s.split_once(':') {
            if let Some((_, test_value_str)) = test.split_once(&['>', '<']) {
                let mut char_iter = test.chars();
                wf.condition = Some(Condition {
                    field: char_iter.next().unwrap(),
                    comparison: char_iter.next().unwrap(),
                    test_value: test_value_str.parse().unwrap(),
                });
                wf.target = target.to_string();
            }
        }
        Ok(wf)
    }
}


fn find_constraints(workflows: HashMap<String, Vec<String>>) -> Vec<Vec<Condition>> {
    let mut condition_sets = Vec::new();
    for (mut label, mut wf) in workflows.iter().filter(|(_, w)| w.iter().any(|x| x.contains(ACCEPTED))) {
        let mut conditions = Vec::new();
        let g = ACCEPTED.to_string();
        let mut goal = g.as_str();
        loop {
            dbg!(label, wf);
            for option in wf.iter() {
                let wf_option: WorkflowOption = option.parse().unwrap();
                if wf_option.condition.is_none() { break; }
                if wf_option.target.contains(goal) {
                    conditions.push(wf_option.condition.unwrap());
                    break;
                }
                else {
                    conditions.push(wf_option.condition.unwrap().invert());
                }
            }
            if label.as_str() == START {break;}
            goal = label.as_str();
            (label, wf) = find_workflow_to_reach_label(label, &workflows);
        }
        dbg!(&conditions);
        condition_sets.push(conditions);
    }
    condition_sets
}

fn find_workflow_to_reach_label<'a>(goal: &String, workflows: &'a HashMap<String, Vec<String>>) -> (&'a String, &'a Vec<String>) {
    for (label, wf) in workflows.iter() {
        if wf.iter().any(|x| x.contains(goal)) {
            return (label, wf);
        }
    }
    panic!()
}

const MIN_VALUE: u64 = 1;  // inclusive
const MAX_VALUE: u64 = 4001;  // exclusive

fn count_valid_possibilities(conditions: Vec<Condition>) -> u64 {
    let mut boundaries = HashMap::from([
        ('x', (MIN_VALUE, MAX_VALUE)),
        ('m', (MIN_VALUE, MAX_VALUE)),
        ('a', (MIN_VALUE, MAX_VALUE)),
        ('s', (MIN_VALUE, MAX_VALUE)),
    ]);
    for condition in conditions {
        let bounds = boundaries.get_mut(&condition.field).unwrap();
        if condition.comparison == '>' {
            bounds.0 = max(condition.test_value + 1, bounds.0);
        }
        else {
            bounds.1 = min(condition.test_value, bounds.1);
        }
    }
    dbg!(&boundaries);
    boundaries.values().map(|(lower, upper)| upper.saturating_sub(*lower)).product()
}

// 150616375868000
// 167409079868000
// haven't accounted for the possibility of 2 endpoints in one workflow
