# !/bin/sh
set -e

CUSTOMER_ID=${1:-advertiser1}
OFFER_ID=${2:-offer1}
BALANCE_FILE_PATH=${3:-./balance.json}

KEY="balance:$CUSTOMER_ID:$OFFER_ID"

echo "Seed Balance for User $KEY"
redis-cli -x SET $KEY <$BALANCE_FILE_PATH
 
