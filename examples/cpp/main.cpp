#include <iostream>
#include "rollbar.h"

int main() {
    struct rollbar::ConfigCompat config;

    config.access_token = "b5938ecbdb984aa091234644b0686c3d";

    rollbar::Transport* transport = nullptr;

    int code = rollbar::create_transport(config, &transport);

    if (code != 0) {
        return 1;
    }

    rollbar::log(transport, rollbar::LevelCompat::Debug, "hello from cpp");

    rollbar::shutdown(transport);
}
