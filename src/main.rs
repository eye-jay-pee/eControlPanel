mod applab;

fn main() -> applab::errors::Result {
    let hight_res = applab::people::attributes::height::Height::new(5, 5);
    match hight_res {
        Ok(h) => {
            let height: applab::people::attributes::height::Height = h;
            let user = applab::people::Person::new("McLOVIN", "", height);
            applab::TestApp::new(user).launch()
        }
        Err(e) => Err(applab::errors::AppLabError::from(
            applab::people::errors::PeopleError::from(
                applab::people::attributes::errors::AttributeError::from(e),
            ),
        )),
    }
}
