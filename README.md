# Solution timings

Advent of code 2025 solutions (every time is displayed in microseconds):

| Day | Parse time | Prep time | Part 1 time | Part 2 time | Tot time |
|-----|------------|-----------|-------------|-------------|----------|
| 1   | 190        | 0         | 16          | 17          | 223      |
| 2   | 58         | 174       | 0           | 0           | 232      |
| 3   | 43         | 0         | 35          | 141         | 219      |
| 4   | 295        | 0         | 33          | 314         | 642      |
| 5   | 45         | 7         | 25          | 0           | 77       |
| 6   | 0          | 0         | 119         | 319         | 438      |

# Parsing methods

| day | input type                  | parsing                                              |
|-----|-----------------------------|------------------------------------------------------|
| 1   | `L68` `L30`                 | nom                                                  |
| 2   | `11-22,95-115`              | nom                                                  |
| 3   |                             |                                                      |
| 4   | A map of chars              | Iter over lines and get the char and add a border    |
| 5   | Ranges and values           | nom                                                  |
| 6   | Values and operators        | nom                                                  |
| 7   | a map of chars with a start | Iter over the lines and detect the starting position |
| 8   |                             |                                                      |
| 9   |                             |                                                      |
| 10  |                             |                                                      |
| 11  |                             |                                                      |
| 12  |                             |                                                      |
