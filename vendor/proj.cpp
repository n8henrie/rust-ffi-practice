// proj.cpp
#include "proj.h"
#include <map>

void Configuration::setMap() { (*this)["foo"] = "bar"; }
std::string Configuration::getMap() { return (*this)["foo"]; }
