set CONTAINER_NAME "cultura-af2fce60"
set DOCKER_IMAGE "antham/cultura:master"

function fish_greeting
docker start $CONTAINER_NAME >/dev/null 2>&1 || docker run -d --name $CONTAINER_NAME $DOCKER_IMAGE >/dev/null 2>&1
docker exec -t $CONTAINER_NAME cultura fact generate-random
end
