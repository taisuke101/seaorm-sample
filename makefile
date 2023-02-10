start: 
	docker-compose up -d

end:
	docker-compose down

watch:
	cargo-watch -x run