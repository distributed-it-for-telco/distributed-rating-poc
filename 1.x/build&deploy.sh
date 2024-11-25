homeDir=$PWD
echo $homeDir
##===========Metaverse agent================
cd $homeDir/commons
wit-deps
wash build
##===========Metaverse agent================
cd $homeDir/metaverse-rating-agent
wit-deps
wash build
##===========Video on demand agent=================
cd $homeDir/orange-vod-rating-agent
wit-deps
wash build
wac plug ./build/orange_vod_ratingagent_s.wasm --plug ../commons/build/rating_commons_s.wasm -o ./build/orange_vod_ratingagent_s.wasm
##===========Balance Manager================
cd $homeDir/balance-management
wit-deps
wash build
wac plug ./build/balance_manager_s.wasm --plug ../commons/build/rating_commons_s.wasm -o ./build/balance_manager_s.wasm
##===========Rating Coordinator==============
cd $homeDir/rating-coordinator
wit-deps
wash build
##===========Api gateway=====================
cd $homeDir/api-gateway
wit-deps
wash build
wac plug ./build/api_gateway_s.wasm --plug ../commons/build/rating_commons_s.wasm -o ./build/api_gateway_s.wasm
##===========================================
cd $homeDir
wash up -d
sleep 5
wash app delete rating && wash app deploy wadm.yaml 
wash down