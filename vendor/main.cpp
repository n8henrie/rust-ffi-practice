// main.cpp

#include "proj.h"
#include <iostream>

int main() {
    config.setMap();
    std::cout << config.getMap() << std::endl;
    return 0;
}
