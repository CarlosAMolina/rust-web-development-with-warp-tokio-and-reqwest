doc:
	cd server && cargo doc && cargo doc --open

call-return-error:
	curl \
		-X OPTIONS localhost:3030/questions \
		-H "Access-Control-Request-Method: PUT" \
		-H "Access-Control-Request-Headers: invalid-header" \
		-H "Origin: https://not-origin.io" \
		-verbose

add-account:
	curl --location --request POST 'localhost:3030/registration' \
	--header 'Content-Type: application/json' \
	--data-raw '{ "email": "foo@bar.com", "password": "securedPassword" }'

get-answers:
	curl "localhost:3030/answers?offset=0&limit=200"

get-answers-of-question:
	curl "localhost:3030/questions/1/answers"

get-questions:
	curl "localhost:3030/questions?offset=0&limit=200"

get-questions-all:
	curl "localhost:3030/questions"

get-question:
	curl "localhost:3030/questions/1"

# POST request with a JSON body
add-question:
	curl \
		--location \
		--request POST 'localhost:3030/questions' \
		--header 'Content-Type: application/json' \
		--data-raw '{"title": "New question", "content": "How does this work again?"}'

# POST request with a JSON body
add-question-with-words-to-censor:
	curl \
		--location \
		--request POST 'localhost:3030/questions' \
		--header 'Content-Type: application/json' \
		--data-raw '{"title": "Shit title", "content": "Shit comment"}'


# POST curl for an application/x-www-form-urlencoded request
add-answer:
	curl \
		--location \
		--request POST 'localhost:3030/answers' \
		--header 'Content-Type: application/x-www-form-urlencoded' \
		--data-urlencode 'content=The solution is to ...' \
		--data-urlencode 'question_id=1'

update-question:
	curl \
		--location \
		--request PUT 'localhost:3030/questions/1' \
		--header 'Content-Type: application/json' \
		--data-raw '{ "id": 1, "title": "NEW TITLE", "content": "OLD CONTENT" }'

update-question-with-words-to-censor:
	curl \
		--location \
		--request PUT 'localhost:3030/questions/1' \
		--header 'Content-Type: application/json' \
		--data-raw '{ "id": 1, "title": "NEW shit TITLE", "content": "OLD shit CONTENT" }'

delete-question:
	curl \
		--location \
		--request DELETE 'localhost:3030/questions/0' \
		--header 'Content-Type: application/json'

login:
	curl \
		--location --request POST 'localhost:3030/login' \
		--header 'Content-Type: application/json' \
		--data-raw '{ "email": "foo@bar.com", "password": "securedPassword" }'

login-error-wrong-password:
	curl \
		--location --request POST 'localhost:3030/login' \
		--header 'Content-Type: application/json' \
		--data-raw '{ "email": "foo@bar.com", "password": "foo" }'


login-error-account-not-in-db:
	curl \
		--location --request POST 'localhost:3030/login' \
		--header 'Content-Type: application/json' \
		--data-raw '{ "email": "invented@foo.com", "password": "invented_pw" }'

