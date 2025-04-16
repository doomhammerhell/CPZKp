FROM rust:1.70 as builder

WORKDIR /usr/src/cpzkp
COPY . .

RUN apt-get update && apt-get install -y \
    python3-pip &&
    pip3 install maturin

RUN cd examples/webapp &&
    cargo install wasm-pack &&
    wasm-pack build --target web

FROM nginx:alpine

COPY --from=builder /usr/src/cpzkp/examples/webapp/pkg /usr/share/nginx/html/pkg
COPY --from=builder /usr/src/cpzkp/examples/webapp/index.html /usr/share/nginx/html/
COPY --from=builder /usr/src/cpzkp/examples/webapp/style.css /usr/share/nginx/html/

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
