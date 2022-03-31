use rollbar::Config;

fn main() {
    let access_token = std::env::var("ROLLBAR_POST_ITEM_TOKEN")
        .expect("token to be set as ROLLBAR_POST_ITEM_TOKEN");

    let config = Config::builder().access_token(access_token).build();

    let mut filter = env_logger::filter::Builder::new();
    filter.parse("logger=info");

    rollbar::set_logger_with_filter(config, filter.build()).expect("set logger");

    log::info!("this should show up in rollbar");
}
