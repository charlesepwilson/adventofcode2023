use crate::utils::Solves;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

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
            let conditions_mapping: Vec<_> =
                instructions.split(',').map(|x| x.to_string()).collect();
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

fn apply_workflows(
    workflows: HashMap<String, Vec<String>>,
    data: Vec<HashMap<String, u64>>,
) -> Vec<HashMap<String, u64>> {
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
                    if *value > test_value {
                        return target.to_string();
                    }
                }
            } else if let Some((label, test_value_str)) = test.split_once('<') {
                if let Some(value) = data.get(label) {
                    let test_value: u64 = test_value_str.parse().unwrap();
                    if *value < test_value {
                        return target.to_string();
                    }
                }
            }
        } else {
            return option.to_string();
        }
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
        if self.comparison == '>' {
            comparison = '<';
            test_value = self.test_value + 1;
        } else {
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
        let mut wf = Self {
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
    let mut reverse_lookup_cache = HashMap::new();
    let mut condition_sets = Vec::new();
    for (og_label, og_wf) in workflows
        .iter()
        .filter(|(_, w)| w.iter().any(|x| x.contains(ACCEPTED)))
    {
        let endpoint_indices: Vec<usize> = og_wf
            .iter()
            .enumerate()
            .filter(|(_, op)| op.contains(ACCEPTED))
            .map(|(i, _)| i)
            .collect();
        for i in endpoint_indices {
            let mut conditions = Vec::new();
            const DUMMY_GOAL: &str = "---"; //  on finding initial endpoints we use the index instead of the goal
            let mut goal = DUMMY_GOAL;
            let (mut label, mut wf) = (og_label, og_wf);
            let mut seen_labels = HashSet::new(); // Prevent infinite loops for inaccessible endpoints
            let found_start = loop {
                if seen_labels.contains(label) {
                    break false; // Back where we've already been; will never get back to "in"
                } else {
                    seen_labels.insert(label);
                }
                for (j, option) in wf.iter().enumerate() {
                    let wf_option: WorkflowOption = option.parse().unwrap();
                    if wf_option.condition.is_none() {
                        break;
                    }
                    if wf_option.target.contains(goal) || ((goal == DUMMY_GOAL) && (i == j)) {
                        conditions.push(wf_option.condition.unwrap());
                        break;
                    } else {
                        conditions.push(wf_option.condition.unwrap().invert());
                    }
                }
                if label.as_str() == START {
                    break true;
                }
                goal = label.as_str();
                let reverse_lookup =
                    find_workflow_to_reach_label(label, &workflows, &mut reverse_lookup_cache);
                if reverse_lookup.is_none() {
                    break false;
                }
                (label, wf) = reverse_lookup.unwrap();
            };
            if found_start {
                condition_sets.push(conditions);
            }
        }
    }
    condition_sets
}

fn find_workflow_to_reach_label<'a>(
    goal: &String,
    workflows: &'a HashMap<String, Vec<String>>,
    reverse_lookup_cache: &mut HashMap<String, String>,
) -> Option<(&'a String, &'a Vec<String>)> {
    if let Some(label) = reverse_lookup_cache.get(goal) {
        return workflows.get_key_value(label);
    }
    for (label, wf) in workflows.iter() {
        if wf
            .iter()
            .any(|x| x.split_once(':').unwrap_or(("", x.as_str())).1 == goal)
        {
            reverse_lookup_cache.insert(goal.clone(), label.clone());
            return Some((label, wf));
        }
    }
    None
}

const MIN_VALUE: u64 = 1; // inclusive
const MAX_VALUE: u64 = 4001; // exclusive

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
        } else {
            bounds.1 = min(condition.test_value, bounds.1);
        }
    }

    let total = boundaries
        .values()
        .map(|(lower, upper)| upper.saturating_sub(*lower))
        .product();

    total
}
