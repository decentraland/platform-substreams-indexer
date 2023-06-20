./substreams-sink-postgres run \
    "psql://substreams_admin:pass@localhost:5432/substreams_admin?sslmode=disable" \
    "polygon.streamingfast.io:443" \
    "substreams.yaml" \
    --development-mode=true \
    --undo-buffer-size=100 \
    db_out