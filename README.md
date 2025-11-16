# oCMAPI - Container Management API

A project sprung out of the urge to learn rust and wanting to build a `API` for managing `podman` containers in my local environment.

## Functionality

The `API` is built atop the standard `podman` binary and acts as an extension to the functionality it has and makes it possible to remote manage your `podman` servers 
and this `API` was built for and works seamless with `Rootless` containers and do not require any other privileges to work.

The `API` has support for `SSL` out of the box with `Basic` authorization, `Salted` cookie data and `custom` cookie name.
All logging is done in `CEF` format to ensure a standardized logging output when needed.

The `API` has been tested with `openapi-fuzzer` project to minimize issues with each endpoint, but if you discover anything security related PM me here on GitHub with as much details as possible and i will look into it.

The `API` contains information about all the routes when accessing it via browser and going to the root(`/`) path, the information is made with the help of `Redocly`.

<img width="1145" height="278" alt="web" src="https://github.com/user-attachments/assets/fa65489d-21d6-4f1e-83a7-123b2a4dfb64"/>

The `API` has been validated against `OpenAPI` version `3.0.0` and the `yaml` file is baked into the information presented on the first page in the web GUI.

All requests and responses are `JSON` based for all `API` endpoints.

## Configuration
The configuration file has information on what is required or not, the program will let you now if any of them are missing information.

## Usage

The binary has been compiled on Ubuntu 24.04.3 LTS with Rust 1.89, but has been tested on Ubuntu 25.04 and with Podman 5.4.1.

Follow these steps and you should be up and running in no time.

Steps:
1. Download latest release.
2. Unpack in desired folder.
3. Edit `ocmapi.toml`, add your settings and save.
4. Run.

To run the API server manually to test your setting.

Alternative 1:
```
./ocmapi
```
Alternative 2:
```
./ocmapi --config="path/to/ocmapi.toml"
```

Can be that you do not have execute permissions on the binary, then you need to do the following.

```
chmod 770 ./ocmapi
```

This will give the user that extracted the files execute permissions.

## Startup

To make it start at server boot/reboot add the following line to your users crontab.

Make sure that every path in configuration file is absolute paths.

```
@reboot /path/to/file/ocmapi --config "/path/to/config/ocmapi.toml" &
```

## API Endpoints

#### Common

All interfaces related to systemwide information and configuration for podman resides under here.

Endpoints:
> /v1/common/getStats
> 
> /v1/common/getVersion
> 
> /v1/common/getInfo
>
> /v1/common/postEnvFileCreate
>
> /v1/common/postSecretCreate

#### Containers

All interfaces related to container management resides under here.

Endpoints:
> /v1/containers/getStatus
>
> /v1/containers/getStatus/\<id\>
>
> /v1/containers/postContainerCreate
>
> /v1/containers/postSetState

#### Pods

All interfaces related to pod management resides under here.

Endpoints:
> /v1/pods/getStatus
>
> /v1/pods/getStatus/\<id\>
>
> /v1/pods/postPodCreate
>
> /v1/pods/postSetState

#### Networks

All interfaces related to network management resides under here.

Endpoints:
> /v1/networks/getInfo
>
> /v1/networks/getInfo/\<id\>
>
> /v1/networks/postNetworkCreate

## Command line options

There is not many command line options for the server, but they are as follows.

```
Options:
  --config          path to configuration file including file name, if left out
                    it will check in the same directory as the binary for a file
                    named ocmapi.toml.
  -v, --version     show version for oCMAPI.
  --help, help      display usage information
```

## TODO

The following API:s are in the works and will be available shortly.

* API for removing `secrets`.
* API for inspecting `secrets`.
* API for removing `Env` files.
