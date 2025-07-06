#! /bin/bash
docker run --name slab-corp-data -p 5432:5432 -e POSTGRES_PASSWORD=123456 -d postgres

docker cp sql slab-corp-data:/

sleep 5

docker exec --user postgres -it slab-corp-data bash -c '
    psql -f sql/tabelas.sql && \
    psql -f sql/instancias.sql && \
    psql -f sql/gatilho.sql
'