#!/bin/fish

# curl http://localhost:3030/v1/lyric
# curl http://localhost:3030/v1/lyric/EvozjUPua8GYXU2Ec7T8LG

# curl -X POST -H "Content-Type: application/json" -d '{"title": "Whatever", "parts": [["Ja Ja"]]}' http://localhost:3030/v1/lyric

curl -X PUT -H "Content-Type: application/json" -d '{"title": "Ik droom van jou", "parts": [["Ja Ja", "Nee dat is niet waar"]]}' http://localhost:3030/v1/lyric/Wyg6q31RyHfLCbE1HvgEaH

curl -X DELETE http://localhost:3030/v1/lyric/Wyg6q31RyHfLCbE1HvgEaH
