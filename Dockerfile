FROM ubuntu:bionic
MAINTAINER Philipp Hansch (https://github.com/phansch)

RUN apt-get update && apt-get install -y libssl1.0.0

ENV RUST_LOG=info
ENV RUST_BACKTRACE=full

RUN curl -L "https://s3.amazonaws.com/mend-rs-releases/builds/mend-rs-master" > /bin/mend-rs
RUN chmod +x /bin/mend-rs

CMD ["/bin/mend-rs"]
