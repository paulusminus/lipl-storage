GET {{ PREFIX }}/lyric

HTTP 200
[Asserts]
jsonpath "$" count == 59


GET {{ PREFIX }}/playlist

HTTP 200
[Asserts]
jsonpath "$" count == 2
jsonpath "$[0].title" == "Kerst"
jsonpath "$[1].title" == "Sinterklaas"
