OUT="./nohup.out"
if [ "$1" = "dev" ]; then
   OUT="./nohup-dev.out"
fi

nohup sh run-postgres-sink.sh $1 > $OUT &