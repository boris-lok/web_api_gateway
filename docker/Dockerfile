FROM postgres:13.4-bullseye

RUN apt update && apt install -y build-essential curl make gcc postgresql-server-dev-13 libicu-dev

# Install pg_bigm
RUN cd /tmp \
    && curl -L -O https://ja.osdn.net/dl/pgbigm/pg_bigm-1.2-20200228.tar.gz \
    && tar zxf pg_bigm-1.2-20200228.tar.gz \
    && cd pg_bigm-1.2-20200228 \
    && make USE_PGXS=1 \
    && make USE_PGXS=1 install \
    && sed -i "s/#shared_preload_libraries.+$/shared_preload_libraries = 'pg_bigm'/" /usr/share/postgresql/postgresql.conf.sample \
    && rm -rf /tmp/pg_bigm-1.2-20200228 \
    && apt clean

# Install postgis
RUN apt install -y postgis postgresql-13-postgis-3

# Install groonga
ENV PGROONGA_VERSION=2.2.9-1
RUN cd /tmp \
    && curl -L -O https://packages.groonga.org/debian/groonga-archive-keyring-latest-bullseye.deb \
    && apt install -y ./groonga-archive-keyring-latest-bullseye.deb \
    && apt update \
    && apt install -y postgresql-13-pgdg-pgroonga=${PGROONGA_VERSION}

COPY init.sql /docker-entrypoint-initdb.d