#!/bin/bash

# Load the .env variables
if [ -f .env ]; then
    source .env
else
    echo "The file .env doesn't exist"
    exit 1
fi

# Subscribe to the "Publish" event
# XDR built with JS: sorobanClient.xdr.ScVal.scvString("Publish").toXDR("base64")
curl    -X POST \
        -H "Content-Type: application/json" \
        -H "Authorization: Bearer $MERCURY_JWT_TOKEN" \
        -d '{
            "contract_id": "'"$1"'",
            "topic1": "AAAADgAAAAdQdWJsaXNoAA=="
            }' \
        $MERCURY_BACKEND_ENDPOINT/event

# Subscribe to the "Deploy" event
# XDR built with JS: sorobanClient.xdr.ScVal.scvString("Deploy").toXDR("base64")
curl    -X POST \
        -H "Content-Type: application/json" \
        -H "Authorization: Bearer $MERCURY_JWT_TOKEN" \
        -d '{
            "contract_id": "'"$1"'",
            "topic1": "AAAADgAAAAZEZXBsb3kAAA=="
            }' \
        $MERCURY_BACKEND_ENDPOINT/event

# Subscribe to the "Claim" event
# XDR built with JS: sorobanClient.xdr.ScVal.scvString("Claim").toXDR("base64")
curl    -X POST \
        -H "Content-Type: application/json" \
        -H "Authorization: Bearer $MERCURY_JWT_TOKEN" \
        -d '{
            "contract_id": "'"$1"'",
            "topic1": "AAAADgAAAAVDbGFpbQAAAA=="
            }' \
        $MERCURY_BACKEND_ENDPOINT/event

echo "\n\nSuccessfully subscribed to the events"