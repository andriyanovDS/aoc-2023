mod day1;

fn main() -> anyhow::Result<()> {
    let challenge_day = std::env::args()
        .nth(1)
        .expect("Usage: <challenge day> <challenge part>");

    let challenge_part = std::env::args()
        .nth(2)
        .expect("Usage: <challenge day> <challenge part>");

    match (challenge_day.as_str(), challenge_part.as_str()) {
        ("1", "1") => day1::first_part(),
        ("1", "2") => day1::second_part(),
        _ => panic!("Challge was not done yet!"),
    }
}
