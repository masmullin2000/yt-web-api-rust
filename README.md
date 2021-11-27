Install Apache ab (https://httpd.apache.org/docs/2.2/programs/ab.html)

```shell
sudo apt install apache2-utils
```

Test the Rust api.

Build it:

```shell
docker-compose up --build -d
```

With keep-alive:

```shell
ab -k -n 50000 -c 1000 localhost:8081/api/v1/users
```

Without keep-alive:

```shell
ab -n 50000 -c 1000 localhost:8081/api/v1/users
```

Test the Laravel api.

Build:

```shell
docker-compose -f docker-compose-dev.yml up --build -d
```

With keep-alive:

```shell
ab -k -n 50000 -c 1000 localhost:8082/api/users
```

Without keep-alive:

```shell
ab -n 50000 -c 1000 localhost:8082/api/users
```