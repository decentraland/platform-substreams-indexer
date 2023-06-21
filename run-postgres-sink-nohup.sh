PORT="5432"
OUT="./nohup.out"
if [ "$1" = "dev" ]; then
   PORT="5431"
   OUT="./nohup-dev.out"
fi

nohup sh run-postgres-sink.sh $PORT > $OUT &