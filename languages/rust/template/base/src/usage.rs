pub fn calculate_cost_micros(
    prompt_texts: &[String],
    context_texts: &[String],
    tool_results: &str,
) -> u64 {
    ((prompt_texts
        .iter()
        .chain(context_texts.iter())
        .map(|text| text.len() as u64)
        .sum::<u64>())
        * 2)
        + ((tool_results.len() as u64) * 3)
}
