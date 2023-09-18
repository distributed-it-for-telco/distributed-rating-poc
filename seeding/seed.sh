
# Seed data for user yann
echo "Seed Data for User Yann"
redis-cli -x SET yann <./yann-customer.json
redis-cli -x SET bucket:yann:2 <./yann-bucket.json
 
# Seed data for user Pascal
echo "Seed Data for User Pascal"
redis-cli -x SET pascal <./pascal-customer.json
redis-cli -x SET bucket:pascal:2 <./pascal-bucket.json


echo "Seed Data for Orange Usage proof"
redis-cli -x LPUSH rating:usage <./usage-orange-1.json
redis-cli -x LPUSH rating:usage <./usage-orange-2.json
