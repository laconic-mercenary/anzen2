# What?

Open Source application for monitoring temporarily unattended babies and todders.

**WARNING:** This application is intended for short-term monitoring only and does not encourage or endorse leaving children unattended for extended periods. Always prioritize the safety and well-being of children by providing proper adult supervision.

# Why? 

During nighttime and early morning hours, it is easy for children to become unattended. This can be due to a variety of factors, including: chores, lack of sleep, or simply being unaware that they are unattended. This application provides a simple way to monitor the state of a child and alert the parents if they are unattended.

# How?

The application consists of two parts: Frontend and Backend. The Frontend is a web application written in plain HTML and JavaScript and the Backend is a Rust server application.

## Frontend

You can use the frontend on any device that has a camera and microphone. 

You can stream video from a commodity webcamera to the frontend and use the backend to detect if the child is unattended.

### Browser Support
THe app uses the [MediaDevices.getUserMedia()](https://developer.mozilla.org/en-US/docs/Web/API/MediaDevices/getUserMedia) API.

The frontend has been tested on the following browsers:
- Google Chrome
- Safari

More browsers may work but have not been tested.
 
### ToDo
- [ ] Add support for more browsers
- [ ] Add support for audio transmitting

## Backend

The backend is a Rust server application that runs on any device you can compile the Rust code to. It also supports docker. 

### Compiling using Cargo

#### Requirements
- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

#### Running
You can compile the backend using the following command:
```bash
RUN cargo build --release
```

And executing:
```bash
./target/release/anzen2
```

### Running using Docker

#### Requirements
- [Docker](https://docs.docker.com/get-docker/)

#### Running
You can run the backend using the following command:
```bash
docker build -t anzen2.
docker run -p 8080:8080 -it anzen2
```

### ToDo
- [ ] Add support for Azure or AWS image recognition
