FROM maven:3.6.3-openjdk-15 as maven_install

RUN groupadd --system capture && \ 
    useradd --system --gid capture --create-home capture && \
    mkdir /home/capture/.m2 && \
    chown capture:capture /home/capture/.m2

USER capture

ENV MAVEN_OPTS="-XX:+TieredCompilation -XX:TieredStopAtLevel=1"

ADD settings.xml /home/capture/.m2
ADD pom.xml /home/capture
ADD src /home/capture/src

WORKDIR /home/capture

RUN mvn -Duser.home=/home/capture install


FROM openjdk:15-jdk-slim as capture_app

RUN groupadd --system capture && \ 
    useradd --system --gid capture capture

USER capture

COPY --from=maven_install /home/capture/target/capture-latest.jar /usr/local/bin

CMD ["java", "-jar", "/usr/local/bin/capture-latest.jar"]