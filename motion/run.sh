docker build . --tag motion:latest
docker run motion:latest
docker image rm motion:latest --force