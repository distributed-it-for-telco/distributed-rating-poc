
# Seed data for user yann
customerId=${1:-'advertiser1'}

# offerId=${2:-'metaverse-vod'}

offerId='metaverse-vod'

KEY='balance:'$customerId:$offerId

echo "Seed Data for User $KEY"
redis-cli -x SET $KEY <./metaverse-vod-balance.json
 
