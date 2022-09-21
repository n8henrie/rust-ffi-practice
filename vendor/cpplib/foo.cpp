#include <iostream>
#include "bar.h"

int main() {
    std::cout << "before" << std::endl;
    int val = barfunc();

    std::cout << val << std::endl;
    std::cout << "and after" << std::endl;
}
