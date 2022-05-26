#include <iostream>
#include "rollbar.h"

int main() {
    char *message = rollbar::greet("there");
    std::cout << message;
    return 0;
}
