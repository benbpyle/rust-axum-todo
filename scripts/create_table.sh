aws --endpoint-url=http://localhost:8000 dynamodb --region=us-east-1 create-table \
    --table-name Todo \
    --attribute-definitions \
        AttributeName=id,AttributeType=S \
    --key-schema \
        AttributeName=id,KeyType=HASH \
    --billing-mode PAY_PER_REQUEST 
