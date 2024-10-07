![coapi-cropped](https://github.com/user-attachments/assets/9c0857c8-b400-4cec-acc3-25c2facc2277)

<details>
<summary>Table of Contents</summary>

- [coapi.rs](#coapi-rs)
- [Installation](#installation)
    -[Modes](#modes)    
- [Introduction](#introduction)
- [Usage](#usage)
- [Licesnse](#license)

</details>

<!-- <img align="left" src="https://github.com/Aavtic/ena/releases/download/tags/ena-logo.png" alt="drawing" width="200"/> -->

# COAPI-rs

COAPI-rs is is a Rust axum server which responds with the output of code given. The server is still in it's early stages and currently only support execution of [python] 
language. 

## Installation

### Modes
<details>
<summary>Modes of Installation</summary>
    
- [Manually Compiling](#docker)
- [Downloading-executable](#compile)

</details>


###  Docker
You can clone this repository and then use docker to run the server. 
```shell
git clone https://github.com/Aavtic/coapi.rs
```
Then you can build the docker container using this command
This may take some time to set up Rust and download all the dependencies.
```shell
docker build -t rust-app .
```
Now you can run the server using this command.
```shell
docker run --rm -p 8081:8081 rust-app
```
This will start a server on port `8081`. Make sure no other processes are active on that port.

### Compile

In-order to compile and run this, you will have to clone this repository and run the server using cargo.
```shell
cargo run
```

## Introduction

coapi-rs is a rust axum server which can be used to get output for code by sending http POST requests.


## Usage

Once the program is running You can send POST requests to the server with It's payload in JSON format.
The format of the Payload is simple and straight forward as shown below.

    {
	    "code": "print("Hello World!")",
		  "language": "Python"
    }
Make sure to add the `Content-Type` in the header to `application/json`.

## License

[MIT](./LICENSE)


[python]: https://www.python.org/downloads/


