FROM ubuntu:22.10
 
LABEL author="greycode"
LABEL version="1.0.0"
LABEL opensource="https://github.com/kn007/silk-v3-decoder"
LABEL desc="silkV3 decoder,\
            Please mount the folder address you want to decode to the /media directory of docker"
LABEL silkv3-decoder="#!/bin/bash\
        function lm_traverse_dir(){\
            for file in `ls $1`       \
            do\
                if [ -d $1"/"$file ]  \
                then\
                    lm_traverse_dir $1"/"$file $2\
                else  \
                    effect_name=$1"/"$file		\
                    echo $effect_name			\
                    sh converter.sh $effect_name $2\
                fi\
            done\
        }   \
        lm_traverse_dir $1 $2\
        "

COPY silk-v3-decoder /silk-v3-decoder
ENV PATH="/silk-v3-decoder:${PATH}"

RUN apt-get update && apt-get install -y gcc g++ make ffmpeg
RUN cd /silk-v3-decoder/silk && make && make decoder

WORKDIR /silk-v3-decoder


CMD ["silkv3-decoder","/media","mp3"]

