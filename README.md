# rost [![Build Status](https://travis-ci.org/cristianoliveira/rost.svg?branch=master)](https://travis-ci.org/cristianoliveira/rost)
A simple host manager in Rust.

### Demo
![demo](https://raw.githubusercontent.com/cristianoliveira/rost/master/demorost.mov.gif)

## Installation
To install it from source, clone this repo and inside this folder do
```bash
# inside rost folder
sudo make install
```
It will build rost and send to /usr/local/bin/rost

## Usage
```bash
rost -h
```

### Add host
```bash
sudo rost add 10.0.0.1  myrouter
```
### Delete host
It delete all host that match the pattern.
```bash
sudo rost del myrouter
```

### List host
```bash
sudo rost list
```

## Tests
Running tests:
```bash
make test
```

## Contributing
 - Fork it!
 - Create your feature branch: `git checkout -b my-new-feature`
 - Commit your changes: `git commit -am 'Add some feature'`
 - Push to the branch: `git push origin my-new-feature`
 - Submit a pull request

**Pull Request should have unit tests**

## Licence 
This project was made under MIT licence.
