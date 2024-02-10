#!/bin/bash

docker run --ulimit nofile=122880:122880 --rm -v (pwd):/local swaggerapi/swagger-codegen-cli generate \
  -i https://www.interactivebrokers.com/api/doc.json \
  -l rust \
  -o /local/out/rust
