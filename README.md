# pcstat
Linux Page Cache Stats

### Install

```
$ cargo install --git https://github.com/Hanaasagi/pcstat-rs
```

### Usage

```
$ dd if=/dev/urandom of=sample bs=10M count=1
$ pcstat git:(master) ✗ cargo run -- -f sample
[
  {
    "name": "sample",
    "size": 10485760,
    "m_time": {
      "secs_since_epoch": 1557031318,
      "nanos_since_epoch": 911935474
    },
    "pages": 2560,
    "cached": 2560,
    "uncached": 0,
    "percent": 100.0
  }
]
$ pcstat git:(master) ✗ sync && echo 3 > /proc/sys/vm/drop_caches
$ pcstat git:(master) ✗ cargo run -- -f sample
[
  {
    "name": "sample",
    "size": 10485760,
    "m_time": {
      "secs_since_epoch": 1557031318,
      "nanos_since_epoch": 911935474
    },
    "pages": 2560,
    "cached": 0,
    "uncached": 2560,
    "percent": 0.0
  }
]
```

### License
[MIT License](https://github.com/Hanaasagi/pcstat-rs/blob/master/LICENSE) Copyright (c) 2019, Hanaasagi
