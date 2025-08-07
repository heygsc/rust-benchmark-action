# rust-benchmark-action

## m4p mac mini

```
Benchmarking long_str_100_replace: Collecting 100 samples in estimated 6.0706 s (500
long_str_100_replace    time:   [12.145 ms 12.183 ms 12.225 ms]
                        change: [−4.3684% −3.6684% −3.0066%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) high mild
  2 (2.00%) high severe

Benchmarking short_str_10k_replace: Collecting 100 samples in estimated 5.8524 s (30
short_str_10k_replace   time:   [181.87 µs 185.02 µs 188.10 µs]
                        change: [−3.0236% −0.4471% +2.2451%] (p = 0.74 > 0.05)
                        No change in performance detected.
```
