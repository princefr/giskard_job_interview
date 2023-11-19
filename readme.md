[![MacOS](https://github.com/princefr/giskard_job_interview/actions/workflows/mac.yml/badge.svg)](https://github.com/princefr/giskard_job_interview/actions/workflows/mac.yml)
[![Ubuntu](https://github.com/princefr/giskard_job_interview/actions/workflows/ubuntu.yml/badge.svg)](https://github.com/princefr/giskard_job_interview/actions/workflows/ubuntu.yml)
[![Windows](https://github.com/princefr/giskard_job_interview/actions/workflows/windows.yml/badge.svg)](https://github.com/princefr/giskard_job_interview/actions/workflows/windows.yml)

# https://github.com/lioncowlionant/developer-test 
## Giskard test entry
This is a test entry for the Giskard project job application.

**The goal of this repository is to provide a solution to the problem described in the link above.**
**The solution is provided in the form of a bin and a server.**
**The bin is a command line tool that takes two json files as arguments and returns the odds of the ship reaching Endor.**
**The server is a web server that exposes a graphql api to get the odds of a ship reaching Endor.**
**The client is a web client that consumes the graphql api and displays the odds of a ship reaching Endor.**

***The bin and the server are written in rust.***
***The client is written in javascript (Vuejs)***
***The bin and the server are tested on MacOS, Ubuntu and Windows.***


### How to run the bin
1. Clone the repository
2. Run `chmod +x install.sh` to make the script executable
2. Run `./install.sh` to install bin and server dependencies
3. Run bin from anywhere (except for microsoft windows users)

***The bin and the server are automatically installed in your path (except windows) when you run the ./install.sh file***

```
$ give-me-the-odds example1/millennium-falcon.json example1/empire.json
0
```

### How to run the server (in the server directory)
If you followed the steps above, you can run the server or cargo run --bin server from the server directory
```
$ server 
```
```
$ cargo run --bin server 
```

### How to run the tests (in the server directory)
```
$ cargo test
```
Test are run when pushin to the repository, you can see the results in the badges above.



### How to run the client
please navigate to the client directory and follow the instructions in the readme.md file





