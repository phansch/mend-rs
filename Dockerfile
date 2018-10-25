FROM buildpack-deps:stretch
MAINTAINER Philipp Hansch (https://github.com/phansch)

ENV RUST_LOG=info
ENV RUST_BACKTRACE=full

# Uncomment the line below for running the container from a mend-rs checkout
# COPY ./target/debug/mend-rs /bin/mend-rs
RUN curl -L "https://s3.amazonaws.com/mend-rs-releases/builds/mend-rs-master" > /bin/mend-rs
RUN chmod +x /bin/mend-rs

CMD ["/bin/mend-rs"]
