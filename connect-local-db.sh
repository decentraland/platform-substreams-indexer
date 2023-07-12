PORT="5432"
if [ $1 = "dev" ]; then
   PORT="5431"
fi

PGPASSWORD=pass psql -h 127.0.0.1 -p $PORT -U substreams_admin