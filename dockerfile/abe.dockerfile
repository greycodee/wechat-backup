FROM openjdk:11.0.15-slim

WORKDIR /abe
RUN apt-get update \
    && apt-get install -y wget \
    && wget https://github.com/nelenkov/android-backup-extractor/releases/download/master-20220609062817-33a2f6c/abe.jar

CMD [ "/bin/bash" ]