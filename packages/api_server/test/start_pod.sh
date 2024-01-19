#!/bin/bash

curl 'http://127.0.0.1:7000/pod' \
  -H 'content-type: application/json; charset=UTF-8' \
  -X POST \
  --data-raw $'
    {
      "config": {
        "name": "test_pod",
        "pod_id": "DEMO_POD",
        "broker_list": [
          {
            "identifier": "longbridge",
            "config_map": {}
          }
        ],
        "persistent_kv_store": {
          "identifier": "MemoryKVStore",
          "config_map": {}
        },
        "strategy": {
          "identifier": "ExamplePrintLivePriceStrategy",
          "config_map": {}
        },
        "metrics_registry": {
          "identifier": "NoOpMetricRegistryFactory",
          "config_map": {}
        },
        "event_listener_list": []
      }
    }
  ' \
  --compressed
