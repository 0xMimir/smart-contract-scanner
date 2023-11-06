curl --request POST \
     --url $URL \
     --header 'accept: application/json' \
     --header 'content-type: application/json' \
     --data '
{
  "id": 1,
  "jsonrpc": "2.0",
  "method": "alchemy_getAssetTransfers",
  "params": [
    {
      "fromBlock": "0x0",
      "toBlock": "latest",
      "contractAddresses": ["0x70535c46ce04181adf749f34b65b6365164d6b6e"],
      "withMetadata": false,
      "excludeZeroValue": false,
      "maxCount": "0x3e8",
      "category": ["external"]
    }
  ]
}
' | jq .