mod applab;

fn main() -> applab::errors::Result {
    let height = applab::people::attributes::height::Height::new(5, 5)
        .map_err(|e| {
            applab::errors::AppLabError::from(
                applab::people::errors::PeopleError::from(
                    applab::people::attributes::errors::AttributeError::from(e),
                ),
            )
        })?;

    let user = applab::people::Person::new("McLOVIN", "", height);
    applab::TestApp::new(user).launch()
}
