call-return-error:
	curl \
		-X OPTIONS localhost:3030/questions \
		-H "Access-Control-Request-Method: PUT" \
		-H "Access-Control-Request-Headers: invalid-header" \
		-H "Origin: https://not-origin.io" \
		-verbose

get-answers:
	curl "localhost:3030/answers?start=0&end=200"

get-questions:
	curl "localhost:3030/questions?start=0&end=200"

get-question:
	curl "localhost:3030/questions/1"

# POST request with a JSON body
add-question:
	curl \
		--location \
		--request POST 'localhost:3030/questions' \
		--header 'Content-Type: application/json' \
		--data-raw '{"id": "2", "title": "New question", "content": "How does this work again?"}'

# POST curl for an application/x-www-form-urlencoded request
add-comment:
	curl \
		--location \
		--request POST 'localhost:3030/comments' \
		--header 'Content-Type: application/x-www-form-urlencoded' \
		--data-urlencode 'content=The solution is to ...' \
		--data-urlencode 'questionId=1'

update-question:
	curl \
		--location \
		--request PUT 'localhost:3030/questions/1' \
		--header 'Content-Type: application/json' \
		--data-raw '{ "id": "1", "title": "NEW TITLE", "content": "OLD CONTENT" }'

delete-question:
	curl \
		--location \
		--request DELETE 'localhost:3030/questions/1' \
		--header 'Content-Type: application/json'

