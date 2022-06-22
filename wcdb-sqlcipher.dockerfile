FROM alpine:3.16.0
 
LABEL author="greycode"
LABEL version="1.0.0"
LABEL desc="wcdb from wechat,this is decipher sqlcipher for wcdb!"

COPY wcdb-sqlcipher /usr/local/bin/wcdb-sqlcipher

RUN apk add gcc g++ make libffi-dev openssl-dev tcl git
RUN git clone https://github.com/sqlcipher/sqlcipher.git \
    && cd sqlcipher \
    && ./configure --enable-tempstore=yes CFLAGS="-DSQLITE_HAS_CODEC" LDFLAGS="-lcrypto" \
    && make \
    && make install

WORKDIR /wcdb
ENTRYPOINT ["/usr/local/bin/wcdb-sqlcipher"]
