# Installation

This section explains how to install the CLI tool as well as installing TGI from source. **The strongly recommended approach is to use Docker, as it does not require much setup. Check [the Quick Tour](./quicktour) to learn how to run TGI with Docker.**

## Install CLI

You can use TGI command-line interface (CLI) to download weights, serve and quantize models, or get information on serving parameters. 

To install the CLI, you will need to first clone the TGI repository and then run the following commands to install the necessary dependencies and configure the build environment:

```bash
git clone https://github.com/huggingface/text-generation-inference.git && cd text-generation-inference
make install
```

If you would like to serve models with custom kernels, run

```bash
BUILD_EXTENSIONS=True make install
```

## Local Installation from Source

Before you start, you will need to setup your environment, install the necessary dependencies, and install Text Generation Inference. Text Generation Inference is tested on **Python 3.9+**. Additionally, you may need to install Protoc and the necessary build tools.

Text Generation Inference is available on pypi, conda and GitHub. 

To install and launch locally, first [install Rust](https://rustup.rs/) and create a Python virtual environment with at least
Python 3.9, e.g. using conda:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

conda create -n text-generation-inference python=3.9
conda activate text-generation-inference
```

You may also need to install Protoc.

On Linux:

```bash
PROTOC_ZIP=protoc-21.12-linux-x86_64.zip
curl -OL https://github.com/protocolbuffers/protobuf/releases/download/v21.12/$PROTOC_ZIP
sudo unzip -o $PROTOC_ZIP -d /usr/local bin/protoc
sudo unzip -o $PROTOC_ZIP -d /usr/local 'include/*'
rm -f $PROTOC_ZIP
```

On MacOS, using Homebrew:

```bash
brew install protobuf
```

Then, run the following commands to install Text Generation Inference, including the necessary build tools:

```bash
git clone https://github.com/huggingface/text-generation-inference.git && cd text-generation-inference
BUILD_EXTENSIONS=True make install
```

<Tip warning={true}>

On some machines, you may also need to install the OpenSSL libraries and gcc. If you're on a Linux machine, run the following command to install the required build tools:

```bash
sudo apt-get install libssl-dev gcc -y
```

</Tip>

Once installation is done, simply run:

```bash
make run-falcon-7b-instruct
```

This will serve Falcon 7B Instruct model from the port 8080, which we can query.
