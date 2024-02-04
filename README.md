# Disclaimer
This repository is currently a pre-alpha version. It is aimed at my own personal research and not suitable for any purpose. There will be a mature alpha version that can be used by others. Until then the source code, the interfaces, the modules can change arbitrarily without notice.
# Introduction
ZuStDPipe (Zukunft Stateful Data Pipelines) is a library and ecosystem for lightweight highly modular stateful data pipelines for embedded systems to large scale clusters.

The key advantage is that you can address data flows instead of control flows. Data flows have multiple advantages compared to control-flows for data processing, such as better performance due to massive parallization, simpler description and maintenance.

The core of ZuStDPipe is highly modular and users may decide which module to include depending on the system as well as usage context. 
Due to the modularity it can run  small embedding devices up to large scale clusters consisting of millions of nodes. Furthermore, thanks to the modularity you can
integrate any stateful data pipelinees, such as search indexes for any type of document, such as text or images. However, they are also very suitable for structured data processing.

It is written in Rust for safe, multi-platform and low resource usage applications. The WebAssembly (WASM)/WASM System Interface (WASI) ecosystem provides the foundation to run them securely everywhere.

One important aspect of modularization is that it is easier to get rid of modules not needed anymore or to refactor the software. The objective here is that the software does not get bloated more and more, but with each release functionality can simply be removed. A hope is that in the future it is able to run on very small devices that are decades old and for which no more sustainable replacement is possible.

## Code
The code is available under:
* Codeberg (a non-commercial European hosted Git for Open Source): https://codeberg.org/ZuInnoTe/zustdpipe
* Github (an US hosted commercial Git platform): https://github.com/ZuInnoTe/zustdpipe

## License
You can choose to either use [EUPL-1.2](./LICENSE-EUPL-1.2) ([Web](https://spdx.org/licenses/EUPL-1.2.html)) or [Apache-2.0](./LICENSE-Apache-2.0) ([Web](https://spdx.org/licenses/Apache-2.0.html)) license.
# Use cases
## Search Encrypted Cloud Storage
Tools, such as [Cryptomator](https://cryptomator.org/) allow encrypting your files in the cloud and decrypting them locally on any device. However, searching those files locally 
takes a lot of time due to encryption and loading the files from the cloud. ZuStdPipe can be used to create an index of the files and store this index as well encrypted
in the cloud using said tools. Once the index is created, the files are searchable with high speed.
## Highly Scalable Search system
You can create a cluster of nodes running each of them ZuStDPipe locally with parts of the data. You can send a queries to each of the nodes and collect their answers.
A more advanced system "knows" also to which nodes to send the query and get immediate results. Additionally, relevancy ranking needs to be taken into account.
This will be part of zustdp-daemon.
## Search local in the browser
Many web applications nowadays only work if they are online. This is problematic in case of unreliable networks to continue work. ZuStDPipe can be packaged into a WASM application and thus provide search locally in documents processed by the web application.

# Components
## zustdpipe
ZuStDPipe is the core of ZuStDPipe and it provides a lightweight very stable layer that should change rarely. It is a library that is offered on any operating system.

ZuStDPipe is configuarable using configuration files that define "apps" which define one or more stateful data pipelines.
ZuStDPipe loads all functionality relateed to stateful data pipelins from WebAssembly (WASM) modules that can be implemented in any programming language. Data is exchanged using the Arrow format that is available in many different programming languages.

## zustdpipe-modules-common
This provides various common modules (libraries) for stateful data pipelines. The idea of common
is 1) to provide some high quality modules for common stateful data pipelin cases, especially test 2) to demonstrate the integration of various modules in ZuStDPipe and
3) to be a high quality testbed for ZuStDPipe
Examples for such modules are different pipeline steps, such as extracting from JSON, decompression or tokenization, different types of indexes, such as inverted text index, low latency NLP models or geospatial ones to specific search types, such as traditional text search, similarity for text, images and video or knowledge-graph based questioning and answers.
## zustdp-cli
zus-cli is a command line interface (CLI) that uses the library ZuStDPipe. It can be used for creating indexes, ingesting document into an index, searching an index locally and remotely (zustdpipe-daemon). zustdpipe-cli will have an interactive mode to avoid loading the indexes etc. for every new command from scratch. Nevertheless, for search apps needed to serve thousands of users or for remote searching, a cluster of zus-daemons is recommended.
## zustdp-daemon
zustdp-daemon is a lightweight background daemon that offers zustdpipe via a network. It has various providers, such as REST via HTTP2 (with various authentication possibilities). Those providers are modules that can optionally be integrated. The providers have a conservative default configuration and must not be dependent on various zustdpipe-modules - they can be used with any zustdpipe-module now and in the future.
Additionally, zustdpipe-daemon can dynamically manage their load and share this with other zustdpipe-daemon instances. This is done via messaging mechanisms to distribute load as well as caches to keep track of the load at the global network level. 
zustdpipe-daemon does not provide a user interface. As a common practice the user interface should be separated from the backend. This is part of zus-web. 
## zustdpipe-web
As mentioned before, zustdpipe-daemon should not run also a web user interface, this is provided by zustdpipe-web and can be deployed on any other isolated node or container.

## Bindings
ZuStDPipe is written in Rust. However, also other programming languages should be able benefit from ZuStdPipe and use the library:
* C
* Java
* Python
* ...

Specific wrappers will be created to enable those and that can target different machine architectures. Since modules are compiled to WASM they will be automatically available for any target architecture that supports Wasm.


## Ecosystem ZuStDPipe
It is expect that in the future a full ecosystem of modules will develop around ZuStDPipe and users can abritrarily combine them offering search possibilities not even envision here. The main goal of ZuStDPipe is being highly modular with a stable small core suitable for any device. You can add your own query engine, your own index type or own extraction modules.

The modules can be written in any language that is supported by WebAssembly (WASM). We encourage as a default language to use Rust to leverage its features of safeness and performance to meet the safety goals of ZuStDPipe. If you need to call libraries written in other languages then we recommend to create at least a small wrapper in Rust that calls the library. 
