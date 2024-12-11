docker exec kafka-dev sh /bin/kafka-topics --create --topic pipelines --bootstrap-server localhost:9092
docker exec kafka-dev sh /bin/kafka-topics --create --topic pipelines_output --bootstrap-server localhost:9092
