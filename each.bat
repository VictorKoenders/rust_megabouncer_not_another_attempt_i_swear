cd logger
cargo %*
cd ../
if %errorlevel% NEQ 0 goto:eof
cd server
cargo %*
cd ../
if %errorlevel% NEQ 0 goto:eof
cd shared
cargo %*
cd ../
if %errorlevel% NEQ 0 goto:eof
cd tcp_connector 
cargo %*
cd ../
if %errorlevel% NEQ 0 goto:eof
cd data_store 
cargo %*
cd ../
if %errorlevel% NEQ 0 goto:eof
cd irc_converter 
cargo %*
cd ../
if %errorlevel% NEQ 0 goto:eof

