get-questions:
	curl "localhost:3030/questions?start=0&end=200"

get-question:
	curl "localhost:3030/questions/1"

add-question:
	curl \
		--location \
		--request POST 'localhost:3030/questions' \
		--header 'Content-Type: application/json' \
		--data-raw '{"id": "2", "title": "New question", "content": "How does this work again?"}'

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

