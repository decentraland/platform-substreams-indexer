PORT="5432"
CONTAINER="substreams"
if [ $1 = "dev" ]; then
   CONTAINER="substreams_dev"
fi

docker exec -e PGPASSWORD=pass -it $CONTAINER  psql -h 127.0.0.1 -p $PORT -U substreams_admin
