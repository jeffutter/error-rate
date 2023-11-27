FROM alpine AS setup
RUN adduser -D -H error-rate
COPY ./error-rate /usr/local/bin/
RUN chmod 755 /usr/local/bin/error-rate

FROM alpine

COPY --from=setup /etc/passwd /etc/passwd
COPY --from=setup /usr/local/bin/error-rate /usr/local/bin/error-rate
USER error-rate

CMD ["/usr/local/bin/error-rate"]
