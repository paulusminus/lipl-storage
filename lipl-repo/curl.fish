#!/bin/fish

# curl http://localhost:3030/v1/lyric
# curl http://localhost:3030/v1/lyric/EvozjUPua8GYXU2Ec7T8LG

# curl -X POST -H "Content-Type: application/json" -d '{"title": "Whatever", "parts": [["Ja Ja"]]}' http://localhost:3030/v1/lyric

set base http://localhost:3030/v1
set json '-H "Content-Type: application/json"'

function list
    curl -X GET $base/$argv[1] 
end

function delete
    curl -X DELETE $base/$argv[1]/$argv[2]
end

function post
    curl -X POST $json -d '$argv[2]' $base/$argv[1] 
end

function put
    curl -X PUT $json -d $argv[3] $base/$argv[1]/$argv[2]
end

list lyric
list playlist

echo -e "$json\n"

post lyric '{"title": "Ik droom van jou", "parts": [["Ja Ja", "Nee dat is niet waar"]]}'
post playlist '{ "title": "Dromen", "members": []}'
