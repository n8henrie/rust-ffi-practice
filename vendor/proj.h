// proj.h
#include <map>
#include <string>

class Configuration : std::map<std::string, std::string> {
  public:
    void setMap();
    std::string getMap();
};
Configuration config;
