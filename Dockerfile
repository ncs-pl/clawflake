FROM golang:1.19 as builder
WORKDIR /usr/src/clawflake
COPY . .
RUN make generator

FROM scratch
WORKDIR /app
COPY --from=builder /usr/src/clawflake/bin/generator ./
CMD [ "./generator" ]
