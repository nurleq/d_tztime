for time.rs (rust)
```
Note that you need to install the `dotenv` and `chrono` crates in your project.

Also, the `tz_database.txt` file should be created with the following format:
```text
America/New_York America/Los_Angeles Europe/Berlin Asia/Tokyo
```
Each line represents a timezone name. You can find more information about timezones at
[iana.org](https://www.iana.org/assignments/time-zones).

Please note that this is a simple example and does not handle all the edge cases of timezones.


for timec.cpp
```
Note that this code uses the `strptime` function to set the timezone and the `localtime` function to get the current time in a
specific timezone. Also, it uses the `setenv` and `tzset` functions to set the timezone.

Please note that this is a simple example and does not handle all the edge cases of timezones.

Also, you need to install the necessary libraries (e.g., GNU C Library) for your system to compile and run this code.