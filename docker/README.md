# Docker Setup

On some operating systems, e.g. Mac OSX, the setup is not working fully or the installation of a few programming languages and libraries on the host system is not desired. For this reason a Docker setup is available. It is based on a Docker image that comes with a recent Rust toolchain.

## Installation

Install Docker on your operating system, e.g. Docker Desktop on Mac OSX, also install [docker-compose](https://docs.docker.com/compose/install/). To build the Docker image run:

```
docker-compose build rust
```

Then to start the container and connect to it

```
docker-compose run rust /bin/bash
```

this starts a shell in the container. The project repository is mounted into the image.


## Google Authentication

In order to run all examples inside the Docker container, the setup requires the `GOOGLE_APPLICATION_CREDENTIALS` environment variable to be set, pointing to the Google credentials file. An alternative is to create an `.env` file inside the `./docker` folder, containing the variable with the file path.
When building the container this file is then copied into the Docker image and the env variable points to it.
