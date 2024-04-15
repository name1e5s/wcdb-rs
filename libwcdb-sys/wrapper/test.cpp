#include "Database.hpp"
#include <iostream>

extern "C" void printVersion();

void printVersion() {
    std::cout << "WCDB Source Id: " << WCDB::Database::getSourceId() << std::endl;
}