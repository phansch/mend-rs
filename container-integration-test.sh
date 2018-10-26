docker-compose -f docker-compose.yml -f docker-compose.test.yml build --no-cache
FAKTORY_URL=tcp://localhost:7419 docker-compose up &

sleep 4

ruby tests/auxiliary/submit_job.rb

docker-compose down

