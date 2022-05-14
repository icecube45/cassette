CURRENT_UID := $(shell id -u)


start-frontend:
	docker run -u ${CURRENT_UID} --network host  --rm -v ${PWD}/cassette-frontend:/app trion/ng-cli ng serve

debug-backend:
	cd cassette-backend
	