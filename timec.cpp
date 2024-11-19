#include <iostream>
#include <chrono>
#include <ctime>
#include <string>
#include <vector>
#include <map>
#include <cstdlib>

// Function to get the current time in a specific timezone
time_t getCurrentTime(const std::string& timezone) {
    time_t now = time(0);
    struct tm* ltm = localtime(&now);

    // Set the timezone using strptime function
    char buffer[20];
    strftime(buffer, sizeof(buffer), "%Z", ltm);
    setenv("TZ", buffer, 1);
    tzset();

    // Get the current time in the specified timezone
    now = time(0);
    ltm = localtime(&now);

    return now;
}

// Function to format a time as HH:MM AM/PM
void printTime(const std::string& timezone, const tm& t) {
    char buffer[20];
    strftime(buffer, sizeof(buffer), "%I:%M %p", &t);
    std::cout << "Current Time in " << timezone << ": " << buffer << std::endl;
}

int main() {
    // Get the list of timezones
    std::vector<std::string> timezones = {"America/New_York", "America/Los_Angeles", "Europe/Berlin", "Asia/Tokyo"};

    if (timezones.empty()) {
        std::cout << "No timezones found." << std::endl;
        return 1;
    }

    // Print the current time in each timezone
    for (const auto& tz : timezones) {
        tm t = {};
        t.tm_year = 0;
        t.tm_mon = 0;
        t.tm_mday = 1; // January 1st

        std::string buffer = tz;
        strncat(buffer, "/", sizeof(buffer));
        buffer += "1";

        if (strptime(buffer.c_str(), "%Z%j", &t) == NULL) {
            continue;
        }

        t.tm_year = t.tm_yday / 365 + 1900;
        t.tm_mon = t.tm_mday % 12;

        printTime(tz, t);
    }

    return 0;
}