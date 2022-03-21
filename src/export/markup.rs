use crate::benchmark::benchmark_result::BenchmarkResult;
use crate::output::format::format_duration_value;
use crate::util::units::Unit;

#[derive(Clone)]
pub enum MarkupType {
    /// Markdown table
    Markdown,
}

pub fn markup_table_data(kind: &MarkupType, data: &Vec<String>) -> String {
    return match kind {
        MarkupType::Markdown => format!("| {} |\n", data.join(" | ")),
    };
}

pub fn markup_table_line(kind: &MarkupType, size: usize) -> String {
    return match kind {
        MarkupType::Markdown => format!("|:---|{}\n", "---:|".repeat(size - 1)),
    };
}

pub fn markup_results_unit(results: &[BenchmarkResult], unit: Option<Unit>) -> Unit {
    return if let Some(unit) = unit {
        // Use the given unit for all entries.
        unit
    } else if let Some(first_result) = results.first() {
        // Use the first BenchmarkResult entry to determine the unit for all entries.
        format_duration_value(first_result.mean, None).1
    } else {
        // Default to `Second`.
        Unit::Second
    };
}

/// Check Markdown-based data row formatting
#[test]
fn test_markup_table_data_markdown() {
    let kind = MarkupType::Markdown;
    let data: Vec<_> = vec!["a", "b", "c"].into_iter().map(String::from).collect();

    let markup_actual = markup_table_data(&kind, &data);
    let markup_expected = format!("| a | b | c |\n");

    assert_eq!(markup_expected, markup_actual);
}

/// Check Markdown-based horizontal line formatting
#[test]
fn test_markup_table_line_markdown() {
    let kind = MarkupType::Markdown;
    let size = 5;

    let markup_actual = markup_table_line(&kind, size);
    let markup_expected = format!("|:---|---:|---:|---:|---:|\n");

    assert_eq!(markup_expected, markup_actual);
}

/// Check unit resolving for timing results and given unit 's'
#[test]
fn test_markup_table_results_unit_given_s() {
    use std::collections::BTreeMap;
    let results = vec![
        BenchmarkResult {
            command: String::from("sleep 2"),
            mean: 2.0050,
            stddev: Some(0.0020),
            median: 2.0050,
            user: 0.0009,
            system: 0.0012,
            min: 2.0020,
            max: 2.0080,
            times: Some(vec![2.0, 2.0, 2.0]),
            exit_codes: vec![Some(0), Some(0), Some(0)],
            parameters: BTreeMap::new(),
        },
        BenchmarkResult {
            command: String::from("sleep 0.1"),
            mean: 0.1057,
            stddev: Some(0.0016),
            median: 0.1057,
            user: 0.0009,
            system: 0.0011,
            min: 0.1023,
            max: 0.1080,
            times: Some(vec![0.1, 0.1, 0.1]),
            exit_codes: vec![Some(0), Some(0), Some(0)],
            parameters: BTreeMap::new(),
        },
    ];
    let unit = Some(Unit::Second);

    let markup_actual = markup_results_unit(&results, unit);
    let markup_expected = Unit::Second;

    assert_eq!(markup_expected, markup_actual);
}

/// Check unit resolving for timing results and given unit 'ms'
#[test]
fn test_markup_table_results_unit_given_ms() {
    use std::collections::BTreeMap;
    let results = vec![
        BenchmarkResult {
            command: String::from("sleep 2"),
            mean: 2.0050,
            stddev: Some(0.0020),
            median: 2.0050,
            user: 0.0009,
            system: 0.0012,
            min: 2.0020,
            max: 2.0080,
            times: Some(vec![2.0, 2.0, 2.0]),
            exit_codes: vec![Some(0), Some(0), Some(0)],
            parameters: BTreeMap::new(),
        },
        BenchmarkResult {
            command: String::from("sleep 0.1"),
            mean: 0.1057,
            stddev: Some(0.0016),
            median: 0.1057,
            user: 0.0009,
            system: 0.0011,
            min: 0.1023,
            max: 0.1080,
            times: Some(vec![0.1, 0.1, 0.1]),
            exit_codes: vec![Some(0), Some(0), Some(0)],
            parameters: BTreeMap::new(),
        },
    ];
    let unit = Some(Unit::MilliSecond);

    let markup_actual = markup_results_unit(&results, unit);
    let markup_expected = Unit::MilliSecond;

    assert_eq!(markup_expected, markup_actual);
}

/// Check unit resolving for timing results using the first result entry as 's'
#[test]
fn test_markup_table_results_unit_first_s() {
    use std::collections::BTreeMap;
    let results = vec![
        BenchmarkResult {
            command: String::from("sleep 2"),
            mean: 2.0050,
            stddev: Some(0.0020),
            median: 2.0050,
            user: 0.0009,
            system: 0.0012,
            min: 2.0020,
            max: 2.0080,
            times: Some(vec![2.0, 2.0, 2.0]),
            exit_codes: vec![Some(0), Some(0), Some(0)],
            parameters: BTreeMap::new(),
        },
        BenchmarkResult {
            command: String::from("sleep 0.1"),
            mean: 0.1057,
            stddev: Some(0.0016),
            median: 0.1057,
            user: 0.0009,
            system: 0.0011,
            min: 0.1023,
            max: 0.1080,
            times: Some(vec![0.1, 0.1, 0.1]),
            exit_codes: vec![Some(0), Some(0), Some(0)],
            parameters: BTreeMap::new(),
        },
    ];
    let unit = None;

    let markup_actual = markup_results_unit(&results, unit);
    let markup_expected = Unit::Second;

    assert_eq!(markup_expected, markup_actual);
}

/// Check unit resolving for timing results using the first result entry as 'ms'
#[test]
fn test_markup_table_results_unit_first_ms() {
    use std::collections::BTreeMap;
    let results = vec![
        BenchmarkResult {
            command: String::from("sleep 0.1"),
            mean: 0.1057,
            stddev: Some(0.0016),
            median: 0.1057,
            user: 0.0009,
            system: 0.0011,
            min: 0.1023,
            max: 0.1080,
            times: Some(vec![0.1, 0.1, 0.1]),
            exit_codes: vec![Some(0), Some(0), Some(0)],
            parameters: BTreeMap::new(),
        },
        BenchmarkResult {
            command: String::from("sleep 2"),
            mean: 2.0050,
            stddev: Some(0.0020),
            median: 2.0050,
            user: 0.0009,
            system: 0.0012,
            min: 2.0020,
            max: 2.0080,
            times: Some(vec![2.0, 2.0, 2.0]),
            exit_codes: vec![Some(0), Some(0), Some(0)],
            parameters: BTreeMap::new(),
        },
    ];
    let unit = None;

    let markup_actual = markup_results_unit(&results, unit);
    let markup_expected = Unit::MilliSecond;

    assert_eq!(markup_expected, markup_actual);
}

/// Check unit resolving for not timing results and no given unit defaulting to 's'
#[test]
fn test_markup_table_results_unit_default_s() {
    let results: Vec<BenchmarkResult> = vec![];
    let unit = None;

    let markup_actual = markup_results_unit(&results, unit);
    let markup_expected = Unit::Second;

    assert_eq!(markup_expected, markup_actual);
}
