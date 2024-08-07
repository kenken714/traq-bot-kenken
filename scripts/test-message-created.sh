#!/usr/bin/env bash

BODY='{
    "eventTime": "2019-05-08T13:33:51.690308239Z",
    "message": {
        "id": "bc9106b3-f9b2-4eca-9ba1-72b39b40954e",
        "user": {
            "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45",
            "name": "takashi_trap",
            "displayName": "寺田 健二",
            "iconId": "2bc06cda-bdb9-4a68-8000-62f907f36a92",
            "bot": false
        },
        "channelId": "9aba50da-f605-4cd0-a428-5e4558cb911e",
        "text": "!{\"type\": \"user\", \"raw\": \"@takashi_trap\", \"id\": \"dfdff0c9-5de0-46ee-9721-2525e8bb3d45\"} こんにちは",
        "plainText": "@takashi_trap こんにちは",
        "embedded": [
            {
                "raw": "@takashi_trap",
                "type": "user",
                "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45"
            }
        ],
        "createdAt": "2019-05-08T13:33:51.632149265Z",
        "updatedAt": "2019-05-08T13:33:51.632149265Z"
    }
}'

curl -vf -X POST \
    -H "Content-Type: application/json" \
    -H "X-TRAQ-BOT-TOKEN: $VERIFICATION_TOKEN" \
    -H "X-TRAQ-BOT-EVENT: MESSAGE_CREATED" \
    -d "$BODY" \
    'http://localhost:8080/'