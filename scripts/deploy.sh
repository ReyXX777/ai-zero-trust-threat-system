#!/bin/bash
docker-compose up --build
kubectl apply -f infra/kubernetes/deployment.yml
