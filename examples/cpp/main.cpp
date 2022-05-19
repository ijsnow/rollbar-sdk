#include "sdk/cpp/rollbar.h"

int main() {
    struct config c = {.endpoint="https://api.rollbar.com", .access_token=get_access_token()};
    configure(config);
    warning("oh wow from cpp");
    shutdown();
    return 0;
}

std::string get_access_token() {
    const char * val = std::getenv("POST_TOKEN");
    if (val == nullptr) {
        return "";
    }
    else {
        return val;
    }
}

