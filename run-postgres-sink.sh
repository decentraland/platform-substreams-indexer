PORT="5432"
METRICS_PORT="9102"
if [ $1 = "dev" ]; then
   PORT="5431"
   METRICS_PORT="9103"
fi

./substreams-sink-postgres run \
    "psql://substreams_admin:pass@localhost:$PORT/substreams_admin?sslmode=disable" \
    "polygon.streamingfast.io:443" \
    "substreams.yaml" \
    --development-mode=true \
    --undo-buffer-size=100 \
    --metrics-listen-addr=0.0.0.0:$METRICS_PORT \
    db_out