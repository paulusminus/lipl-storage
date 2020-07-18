#!/bin/bash

POST='-X POST'
JSON='-H "Content-Type: application/json"'
SERVER='http://localhost'
PORT='3030'
URL="v1/groceries/"
TARGET="${SERVER}:${PORT}/${URL}"

echo "${POST}"
echo "${JSON}"
echo "${TARGET}"

# add data
curl "${POST}" "${JSON}" -d '{ "name": "Apples", "quantity": 2 }' "${TARGET}"
curl "${POST}" "${JSON}" -d '{ "name": "Peers", "quantity": 5 }' "${TARGET}"
curl "${POST}" "${JSON}" -d '{ "name": "Cheese", "quantity": 1 }' "${TARGET}"
curl "${POST}" "${JSON}" -d '{ "name": "Computer", "quantity": 1 }' "${TARGET}"
curl "${POST}" "${JSON}" -d '{ "name": "Keyboard", "quantity": 1 }' "${TARGET}"
curl "${POST}" "${JSON}" -d '{ "name": "Mouse", "quantity": 1 }' "${TARGET}"

# delete data
curl -X DELETE -H "${JSON}" -d '{ "name": "Mouse" }' http://localhost:3030/v1/groceries
