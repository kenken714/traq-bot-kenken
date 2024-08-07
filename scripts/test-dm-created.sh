#!/usr/bin/env bash

BODY='{
    "eventTime": "2019-05-08T13:36:09.421492525Z",
    "message": {
        "id": "2d7ff3f5-c313-4f4a-a9bb-0b5f84d2b6f8",
        "user": {
            "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45",
            "name": "takashi_trap",
            "displayName": "寺田 健二",
            "iconId": "2bc06cda-bdb9-4a68-8000-62f907f36a92",
            "bot": false
        },
        "channelId": "c5a5a697-3bad-4540-b2da-93dc88181d34",
        "text": "!{\"type\": \"user\", \"raw\": \"@takashi_trap\", \"id\": \"dfdff0c9-5de0-46ee-9721-2525e8bb3d45\"} こんにちは",
        "plainText": "@takashi_trap こんにちは",
        "embedded": [
            {
                "raw": "@takashi_trap",
                "type": "user",
                "id": "dfdff0c9-5de0-46ee-9721-2525e8bb3d45"
            }
        ],
        "createdAt": "2019-05-08T13:36:09.365393261Z",
        "updatedAt": "2019-05-08T13:36:09.365393261Z"
    }
}'

curl -vf -X POST \
    -H "Content-Type: application/json" \
    -H "X-TRAQ-BOT-TOKEN: $VERIFICATION_TOKEN" \
    -H "X-TRAQ-BOT-EVENT: DIRECT_MESSAGE_CREATED" \
    -d "$BODY" \
    'http://localhost:8080/'