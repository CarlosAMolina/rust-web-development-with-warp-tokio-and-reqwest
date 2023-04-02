get-questions:
	curl "localhost:3030/questions?start=0&end=200"

add-question:
	curl \
		--location \
		--request POST 'localhost:3030/questions' \
		--header 'Content-Type: application/json' \
		--data-raw '{"id": "2", "title": "New question", "content": "How does this work again?"}'

