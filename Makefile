build:
  docker build -t=sequence_number_generator .
run:
  docker run --name=sequence_number_generator --rm=true -i -t -p 8080:8080 \
  -e DATABASE_URL=postgres://docker:docker@localhost:5432/docker \
  --link=postgresql:docker sequence_number_generator

stop:
  docker stop sequence_number_generator

start:
  docker start sequence_number_generator