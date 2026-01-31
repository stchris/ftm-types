use ftm_types::generated::*;

fn main() {
    let mut entity = Person::new("john-smith");
    entity.birth_date = Some(vec!["1979-08-23".to_string()]);

    let mut passport = Passport::builder()
        .id("C716818".to_string())
        .number(vec!["C716818".to_string()])
        .build();

    passport.holder = Some(vec![entity.id]);
}
