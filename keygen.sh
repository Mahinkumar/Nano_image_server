#!/bin/bash

mkdir -p sample_certs
openssl req -x509 -newkey rsa:2048 -keyout sample_certs/key.pem -out sample_certs/cert.pem -days 365 -nodes -subj "/CN=localhost"