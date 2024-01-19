#!/bin/bash

curl 'http://127.0.0.1:7000/pod/inspect' \
  -H 'content-type: application/json; charset=UTF-8' \
  -X POST \
  --data-raw $'
    {
      "pod_id": "1"
    }
  ' \
  --compressed
