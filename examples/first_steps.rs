use ftm_types::generated::*;

// cargo run --example first_steps

fn main() {
    let entity = Person::builder()
        .id("john-smith".to_string())
        .name("John Smith")
        .birth_date("1979-08-23")
        .build();

    let passport = Passport::builder()
        .id("C716818".to_string())
        .holder(entity.id.clone())
        .number("C716818")
        .build();

    dbg!(passport);
}
