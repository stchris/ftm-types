use ftm_types::generated::*;

// cargo run --example first_steps

fn main() {
    let entity = Person::builder()
        .id("john-smith".to_string())
        .name(vec!["John Smith".to_string()])
        .maybe_birth_date(Some(vec!["1979-08-23".to_string()]))
        .build();

    let passport = Passport::builder()
        .id("C716818".to_string())
        .holder(vec![entity.id.clone()])
        .number(vec!["C716818".to_string()])
        .build();

    dbg!(passport);
}
