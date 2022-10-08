# honeybadger

Flood a form with fake information designed for phishing sites to dilute down any legitimate submissions. Also my first project in Rust

## Doing the buildful

```
git clone git@github.com:andydixon/honeybadger.git honeybadger
cd honeybadger
cargo build
```

## Doing the runful

via cargo:
`cargo run  -- -u http://url/to/attack.htm -n 500 -d 250`

with a build:
`honeybadger -u http://url/to/attack.htm -n 500 -d 250`

Options:
```
    -u, --url URL       URL to attack
    -n, --num HITS      Number of hits
    -d, --delay DELAY   delay in msec per request
```
