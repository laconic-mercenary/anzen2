docker build . --tag capture:latest
docker run capture:latest
docker image rm capture:latest --force