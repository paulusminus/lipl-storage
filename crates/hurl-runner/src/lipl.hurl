GET {{ PREFIX }}/lyric

HTTP 200
[Asserts]
jsonpath "$" count == 59

GET {{ PREFIX}}/lyric/daalt

HTTP 404

GET {{ PREFIX }}/lyric/U5jCFGBECj34LSqvZKRz92

HTTP 200
[Asserts]
jsonpath "$.title" == "'k Zag twee beren"

GET {{ PREFIX }}/playlist

HTTP 200
Content-Type: application/json
[Asserts]
jsonpath "$" count == 2
jsonpath "$[0].title" == "Kerst"
jsonpath "$[0].id" == "DHgNQn6c8UhadyKpCdCu5E"
jsonpath "$[1].title" == "Sinterklaas"
jsonpath "$[1].id" == "KxXGs5WaMtaBdgoDm7sMZ5"

GET {{ PREFIX }}/playlist/DHgNQn6c8UhadyKpCdCu5E

HTTP 200
Content-Type: application/json
[Asserts]
jsonpath "$.members[0]" == "QKKvuNZBAph1JaHLs3UNtu"
jsonpath "$.members[1]" == "3YEs82fSStkdhYRKASsn8U"
jsonpath "$.members[2]" == "3zB7JRnEJJs7AAos3BWeBv"

DELETE {{ PREFIX }}/lyric/5aK123aTb3tS7DvFyEn6A8

HTTP 200

GET {{ PREFIX }}/playlist/KxXGs5WaMtaBdgoDm7sMZ5

HTTP 200
Content-Type: application/json
