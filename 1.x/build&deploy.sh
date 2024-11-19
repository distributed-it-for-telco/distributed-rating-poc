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
##===========Rating Coordinator==============
cd $homeDir/rating-coordinator
wit-deps
wash build
##===========Api gateway=====================
cd $homeDir/api-gateway
wit-deps
wash build
##===========================================
cd $homeDir
wash up -d
sleep 5
wash app delete rating && wash app deploy wadm.yaml 
wash down